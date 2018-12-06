use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(PartialEq, Eq, Hash, Debug)]
struct Coord {
    x: i32,
    y: i32,
}

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

        Ok(Coord { x: x, y: y })
    }
}

impl Coord {
    fn man_distance(&self, coord: &Coord) -> i32 {
        (self.x - coord.x).abs() + (self.y - coord.y).abs()
    }
}

fn read_file() -> Vec<Coord> {
    //let filename = "input/input6_debug.txt";
    let filename = "input/input6.txt";
    let file = File::open(filename).expect("cannot open file");
    let reader = BufReader::new(file);

    reader
        .lines()
        .filter_map(|result| result.ok())
        .map(|s| Coord::from_str(&s).unwrap())
        .collect()
}


pub fn answer1() {
    let coords = read_file();

    let (mut min_x, mut max_x, mut min_y, mut max_y) =
        (coords[0].x, coords[0].x, coords[0].y, coords[0].y);
    for c in coords.iter() {
        if c.x < min_x {
            min_x = c.x
        }
        if c.x > max_x {
            max_x = c.x
        }
        if c.y < min_y {
            min_y = c.y
        }
        if c.y > max_y {
            max_y = c.y
        }
    }

    // max_x, max_y will be use to find the size of the array
    println!("grid size: ({}x{}) to ({}x{})", min_x, min_y, max_x, max_y);

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
    for x in min_x..(max_x + 1) {
        for y in min_y..(max_y + 1) {
            let current_coord = Coord { x: x, y: y };
            for candidate in coords.iter() {
                //for id in 1..(coords.len() + 1) {
                //let candidate = coords_map.get(&id).unwrap();
                //println!("evaluating {}, {:?}", id, candidate);
                let distance = current_coord.man_distance(candidate);

                // myself, not doing anything
                if distance == 0 {
                    closest_coord = Some(candidate);
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
                    closest_coord = Some(candidate);
                }
            }

            //println!("closest from {}, {} is {:?} with a distance of {}", x, y, closest_coord, closest_distance);

            if closest_coord.is_some() {
                let closest = closest_coord.unwrap();
                grid.insert(current_coord, closest);
            }

            closest_coord = None;
            closest_distance = max_x + max_y;
        }
    }

    // find the id with the maximum iterations
    let mut counts = HashMap::new();
    for (_coord, closest) in grid.iter() {
        if closest.x != min_x && closest.x != max_x && closest.y != min_y && closest.y != max_y {
            let c = counts.entry(closest).or_insert(0);
            *c += 1;
        }
    }

    let (_, count) = counts.iter().max_by_key(|(_, c)| *c).unwrap();
    println!("Answer1: {}", count);
}
