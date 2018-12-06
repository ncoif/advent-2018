use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug)]
struct Coord(i32, i32);

impl FromStr for Coord {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"([0-9]+), ([0-9]+)").unwrap();
        }
        let c = RE
            .captures(s)
            .ok_or_else(|| format!("cannot parse coord {:?}", s))
            .unwrap();

        let x: i32 = c[1].parse().unwrap();
        let y: i32 = c[2].parse().unwrap();

        Ok(Coord(x, y))
    }
}

impl Coord {
    fn man_distance(&self, coord: &Coord) -> i32 {
        (self.0 - coord.0).abs() + (self.1 - coord.1).abs()
    }
}

pub fn run() {
    let inputs = read_file();

    answer1(&inputs);
}

fn read_file() -> Vec<Coord> {
    let filename = "input/input6_debug.txt";
    let file = File::open(filename).expect("cannot open file");
    let reader = BufReader::new(file);

    reader
        .lines()
        .filter_map(|result| result.ok())
        .map(|s| Coord::from_str(&s).unwrap())
        .collect()
}

fn answer1(coords: &Vec<Coord>) {
    // max_x, max_y will be use to find the size of the array
    let max_x = coords.iter().max_by_key(|c| c.0).unwrap().0;
    let max_y = coords.iter().max_by_key(|c| c.1).unwrap().1;
    println!("grid size: {} per {}", max_x, max_y);

    let mut coords_map = HashMap::new();
    let mut id = 1;
    for c in coords.iter() {
        coords_map.insert(id, c);
        id += 1;
    }
    println!("Number of coords: {}", coords_map.len());

    // iterate over all x and all y, to compute all distances
    let mut grid = HashMap::new();
    let mut closest_coord = None;
    let mut closest_distance = max_x + max_y;
    for x in 0..(max_x + 1) {
        for y in 0..(max_y + 1) {
            let current_coord = Coord(x, y);
            for id in 1..(coords.len() + 1) {
                let candidate = coords_map.get(&id).unwrap();
                //println!("evaluating {}, {:?}", id, candidate);
                let distance = current_coord.man_distance(candidate);

                // myself, not doing anything
                if distance == 0 {
                    closest_coord = Some(id);
                    closest_distance = 0;
                    continue;
                }

                //it's a tie
                if distance != 0 && distance == closest_distance {
                    closest_coord = None;
                }

                // if we have a closet coord
                if distance != 0 && distance < closest_distance {
                    //println!("{} and {:?} are at the same distance of ({}, {})", id, closest_coord, x, y);
                    closest_distance = distance;
                    closest_coord = Some(id);
                }
            }

            //println!("closest from {}, {} is {:?} with a distance of {}", x, y, closest_coord, closest_distance);

            if closest_coord.is_some() {
                let closest = closest_coord.unwrap();
                grid.insert(format!("{}x{}", x, y), closest);
                print!("{} ", closest);
            } else {
                print!(". ");
            }

            closest_coord = None;
            closest_distance = max_x + max_y;
        }
        println!("");
    }

    // find the id with the maximum iterations
    let mut counts = HashMap::new();
    for g in grid.iter() {
        let c = counts.entry(g.1).or_insert(0);
        *c += 1;
    }

    let (_, count) = counts.iter().max_by_key(|(_, c)| *c).unwrap();
    println!("Answer1: {}", count);
}
