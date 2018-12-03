use regex::Regex;
use std::collections::HashMap;
use std::fmt::Debug;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::Chars;

// represent a single entry, for example #1 @ 1,3: 4x4
#[derive(Debug)]
pub struct Area {
    id: u64,
    x: u64,
    y: u64,
    width: u64,
    length: u64,
}

impl Area {
    pub fn from_str(text: &str) -> Self {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"#(?P<id>\d+) @ (?P<x>\d+),(?P<y>\d+): (?P<width>\d+)x(?P<length>\d+)")
                    .unwrap();
        }

        for caps in RE.captures_iter(text) {
            let id1 = caps["id"].parse::<u64>().unwrap();
            return Area {
                id: caps["id"].parse::<u64>().unwrap(),
                x: caps["x"].parse::<u64>().unwrap(),
                y: caps["y"].parse::<u64>().unwrap(),
                width: caps["width"].parse::<u64>().unwrap(),
                length: caps["length"].parse::<u64>().unwrap(),
            };
        }
        unreachable!();
    }
}

pub fn run() {
    let inputs = read_file();
    inputs.into_iter().for_each(|a| println!("{:?}", a))
}

fn read_file() -> Vec<Area> {
    let filename = "input/input3.txt";
    let file = File::open(filename).expect("cannot open file");
    let reader = BufReader::new(file);

    reader
        .lines()
        .filter_map(|result| result.ok())
        .map(|s| Area::from_str(&s))
        .collect()
}
