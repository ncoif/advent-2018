use lazy_static::lazy_static;
use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
struct Nanobot {
    x: i64,
    y: i64,
    z: i64,
    r: u64,
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
        let r: u64 = c[4].parse().unwrap();

        Ok(Nanobot { x, y, z, r })
    }
}

impl Nanobot {
    fn distance_to(&self, other: &Nanobot) -> u64 {
        ((self.x - other.x).abs() + (self.y - other.y).abs() + (self.z - other.z).abs()) as u64
    }

    // a copy of all the nanobots in range from the given list
    fn in_range(&self, others: &Vec<Nanobot>) -> Vec<Nanobot> {
        others
            .iter()
            .filter(|n| self.distance_to(&n) <= self.r)
            .map(|n| n.clone())
            .collect()
    }
}

fn read_file(filename: &str) -> Vec<Nanobot> {
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
    let nanobots = read_file("input/input23.txt");

    let max_radius_nanobot = nanobots.iter().max_by_key(|n| n.r).unwrap();
    let in_range = max_radius_nanobot.in_range(&nanobots);

    println!(
        "Experimental Emergency Teleportation (1/2): {:?}",
        in_range.len()
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

#[test]
fn test_in_range() {
    let nanobots = read_file("input/input23_debug.txt");

    let max_radius_nanobot = nanobots.iter().max_by_key(|n| n.r).unwrap();
    let in_range = max_radius_nanobot.in_range(&nanobots);

    println!("max_radius_nanobot: {:?}", max_radius_nanobot);
    println!("in_range: {:?}", in_range);

    assert_eq!(4, max_radius_nanobot.r);
    assert_eq!(7, in_range.len());
}
