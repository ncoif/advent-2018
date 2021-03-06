use crate::common::error::AocError;
use crate::common::response::AocResponse;

use lazy_static::lazy_static;
use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

// represent a single entry, for example #1 @ 1,3: 4x4
#[derive(Debug)]
pub struct Area {
    id: usize,
    left: usize,
    top: usize,
    width: usize,
    height: usize,
}

impl FromStr for Area {
    type Err = AocError;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(
                r"#(?P<id>\d+) @ (?P<left>\d+),(?P<top>\d+): (?P<width>\d+)x(?P<height>\d+)"
            )
            .unwrap();
        }

        let caps = RE
            .captures(text)
            .ok_or_else(|| format!("cannot parse area {:?}", text))?;

        let id = caps["id"].parse::<usize>()?;
        let left = caps["left"].parse::<usize>()?;
        let top = caps["top"].parse::<usize>()?;
        let width = caps["width"].parse::<usize>()?;
        let height = caps["height"].parse::<usize>()?;

        Ok(Area {
            id,
            left,
            top,
            width,
            height,
        })
    }
}

pub fn answer1() -> Result<AocResponse<usize>, AocError> {
    let areas = read_file()?;

    let width = areas.iter().map(|p| p.left + p.width).max().unwrap();
    let height = areas.iter().map(|p| p.top + p.height).max().unwrap();
    let mut claims = vec![0usize; width * height];

    let access = |x, y| (x + width * y) as usize;

    for p in &areas {
        for x in p.left..p.left + p.width {
            for y in p.top..p.top + p.height {
                claims[access(x, y)] += 1;
            }
        }
    }

    let conflicts = claims.iter().filter(|&c| *c > 1).count();

    Ok(AocResponse::new(
        3,
        1,
        "No Matter How You Slice It",
        conflicts,
    ))
}

pub fn answer2() -> Result<AocResponse<usize>, AocError> {
    let areas = read_file()?;

    let width = areas.iter().map(|p| p.left + p.width).max().unwrap();
    let height = areas.iter().map(|p| p.top + p.height).max().unwrap();
    let mut claims = vec![0usize; width * height];

    let access = |x, y| (x + width * y) as usize;

    for p in &areas {
        for x in p.left..p.left + p.width {
            for y in p.top..p.top + p.height {
                claims[access(x, y)] += 1;
            }
        }
    }

    let mut result = None;
    for p in &areas {
        let mut ok = true;
        for x in p.left..p.left + p.width {
            for y in p.top..p.top + p.height {
                if claims[access(x, y)] > 1 {
                    ok = false;
                }
            }
        }
        if ok {
            result = Some(p.id);
        }
    }

    match result {
        None => Err(AocError::ComputeNotFound),
        Some(r) => Ok(AocResponse::new(3, 2, "No Matter How You Slice It", r)),
    }
}

fn read_file() -> Result<Vec<Area>, AocError> {
    let filename = "input/input3.txt";
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let mut file_lines = vec![];
    for line in reader.lines() {
        let line = line?;
        let line = Area::from_str(&line)?;
        file_lines.push(line);
    }

    Ok(file_lines)
}
