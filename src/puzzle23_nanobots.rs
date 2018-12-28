use lazy_static::lazy_static;
use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug)]
struct Nanobot {
    x: i64,
    y: i64,
    z: i64,
    r: i64,
}

impl FromStr for Nanobot {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"pos=<(\-*\d+),(\-*\d+),(\-*\d+)>, r=(\d+)").unwrap();
        }
        let c = RE
            .captures(s)
            .ok_or_else(|| format!("cannot parse string {:?}", s))
            .unwrap();

        let x: i64 = c[1].parse().unwrap();
        let y: i64 = c[2].parse().unwrap();
        let z: i64 = c[3].parse().unwrap();
        let r: i64 = c[4].parse().unwrap();

        Ok(Nanobot { x, y, z, r })
    }
}

impl Nanobot {
    fn distance_to(&self, other: &Nanobot) -> u64 {
        ((self.x - other.x).abs() + (self.y - other.y).abs() + (self.z - other.z).abs()) as u64
    }
}

fn read_file() -> Vec<Nanobot> {
    let filename = "input/input23.txt";
    let file = File::open(filename).expect("cannot open file");
    let reader = BufReader::new(file);

    reader
        .lines()
        .filter_map(|result| result.ok())
        .map(|s| Nanobot::from_str(&s))
        .filter_map(|result| result.ok())
        .collect()
}

pub fn answer1() {
    let nanobots = read_file();

    println!(
        "Experimental Emergency Teleportation (1/2): {:?}",
        nanobots.len()
    );
}

#[test]
fn test_nanobot_distance() {
    let origin = Nanobot {
        x: 0,
        y: 0,
        z: 0,
        r: 0,
    };
    let n = Nanobot {
        x: 1,
        y: 0,
        z: 0,
        r: 0,
    };
    assert_eq!(1, origin.distance_to(&n));

    let n = Nanobot {
        x: 1,
        y: 1,
        z: 1,
        r: 0,
    };
    assert_eq!(3, origin.distance_to(&n));

    let n = Nanobot {
        x: 1,
        y: 3,
        z: 1,
        r: 0,
    };
    assert_eq!(5, origin.distance_to(&n));
}
