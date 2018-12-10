use lazy_static::lazy_static;
use regex::Regex;
use std::collections::{BTreeSet, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(PartialEq, Eq, Hash, Debug)]
struct Star {
    pos_x: i32,
    pos_y: i32,
    vel_x: i32,
    vel_y: i32,
}

impl FromStr for Star {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let two: Vec<&str> = s
            .trim_start_matches("position=<")
            .trim_end_matches(">")
            .split("> velocity=<")
            .collect();
        let pos: Vec<&str> = two[0].split(", ").collect();
        let vel: Vec<&str> = two[1].split(", ").collect();

        let pos_x: i32 = pos[0].trim().parse().unwrap();
        let pos_y: i32 = pos[1].trim().parse().unwrap();
        let vel_x: i32 = vel[0].trim().parse().unwrap();
        let vel_y: i32 = vel[1].trim().parse().unwrap();

        Ok(Star {
            pos_x,
            pos_y,
            vel_x,
            vel_y,
        })
    }
}

fn read_file() -> Vec<Star> {
    let filename = "input/input10.txt";
    let file = File::open(filename).expect("cannot open file");
    let reader = BufReader::new(file);

    reader
        .lines()
        .filter_map(|result| result.ok())
        .map(|s| Star::from_str(&s).unwrap())
        .collect()
}

pub fn answer1() {
    let stars = read_file();

    println!("answer1: {}", 0);
}
