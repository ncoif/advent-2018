use lazy_static::lazy_static;
use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

// represent a single entry, for example #1 @ 1,3: 4x4
#[derive(Debug)]
pub struct Area {
    id: usize,
    left: usize,
    top: usize,
    width: usize,
    height: usize,
}

lazy_static! {
    static ref RE: Regex =
        Regex::new(r"#(?P<id>\d+) @ (?P<left>\d+),(?P<top>\d+): (?P<width>\d+)x(?P<height>\d+)")
            .unwrap();
}

impl Area {
    pub fn from_str(text: &str) -> Self {
        for caps in RE.captures_iter(text) {
            let id = caps["id"].parse::<usize>().unwrap();
            let left = caps["left"].parse::<usize>().unwrap();
            let top = caps["top"].parse::<usize>().unwrap();
            let width = caps["width"].parse::<usize>().unwrap();
            let height = caps["height"].parse::<usize>().unwrap();

            return Area {
                id,
                left,
                top,
                width,
                height,
            };
        }
        unreachable!();
    }
}

pub fn answer1() {
    let areas = read_file();

    let width = areas.iter().map(|p| p.left + p.width).max().unwrap();
    let height = areas.iter().map(|p| p.top + p.height).max().unwrap();
    let mut claims = vec![vec![0usize; width]; height];
    for p in &areas {
        for x in p.left..p.left + p.width {
            for y in p.top..p.top + p.height {
                claims[y][x] += 1;
            }
        }
    }

    let conflicts = claims
        .iter()
        .map(|v| v.iter().filter(|&c| *c > 1).count())
        .sum::<usize>();

    println!("No Matter How You Slice It (1/2): {}", conflicts);
}

pub fn answer2() {
    let areas = read_file();

    let width = areas.iter().map(|p| p.left + p.width).max().unwrap();
    let height = areas.iter().map(|p| p.top + p.height).max().unwrap();
    let mut claims = vec![vec![0usize; width]; height];
    for p in &areas {
        for x in p.left..p.left + p.width {
            for y in p.top..p.top + p.height {
                claims[y][x] += 1;
            }
        }
    }

    for p in &areas {
        let mut ok = true;
        for x in p.left..p.left + p.width {
            for y in p.top..p.top + p.height {
                if claims[y][x] > 1 {
                    ok = false;
                }
            }
        }
        if ok {
            println!("No Matter How You Slice It (2/2): {:?}", p.id);
        }
    }
}

fn read_file<'a>() -> Vec<Area> {
    let filename = "input/input3.txt";
    let file = File::open(filename).expect("cannot open file");
    let reader = BufReader::new(file);

    reader
        .lines()
        .filter_map(|result| result.ok())
        .map(|s| Area::from_str(&s))
        .collect()
}
