use lazy_static::lazy_static;
use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::num::ParseIntError;
use std::str::FromStr;

struct Point(i64, i64, i64, i64);

impl FromStr for Point {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"(\-*\d+),(\-*\d+),(\-*\d+),(\-*\d+)").unwrap();
        }
        let c = RE
            .captures(s)
            .ok_or_else(|| format!("cannot parse string {:?}", s))
            .unwrap();

        let d1: i64 = c[1].parse().unwrap();
        let d2: i64 = c[2].parse().unwrap();
        let d3: i64 = c[3].parse().unwrap();
        let d4: i64 = c[4].parse().unwrap();

        Ok(Point(d1, d2, d3, d4))
    }
}

impl Point {
    fn distance_to(&self, other: &Point) -> u64 {
        ((self.0 - other.0).abs()
            + (self.1 - other.1).abs()
            + (self.2 - other.2).abs()
            + (self.3 - other.3).abs()) as u64
    }
}

pub fn answer1() {
    let points = read_file("input/input25.txt");

    let mut neighbours = vec![vec![]];
    for p1 in &points {
        let ns = points
            .iter()
            .enumerate()
            .filter(|(_, p2)| p1.distance_to(&p2) <= 3)
            .map(|(ix, _)| ix)
            .collect();
        neighbours.push(ns);
    }

    // https://docs.rs/pathfinding/1.1.10/pathfinding/undirected/connected_components/fn.components.html
    let constellations = pathfinding::undirected::connected_components::components(&neighbours);

    println!(
        "Four-Dimensional Adventure (1/2): {:?}",
        constellations.len()
    );
}

fn read_file(filename: &str) -> Vec<Point> {
    let file = File::open(filename).expect("cannot open file");
    let reader = BufReader::new(file);

    reader
        .lines()
        .filter_map(|result| result.ok())
        .map(|s| Point::from_str(&s))
        .filter_map(|result| result.ok())
        .collect()
}