use lazy_static::lazy_static;
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

fn find_min_max(coords: &Vec<Coord>) -> (i32, i32, i32, i32) {
    let min_x = coords
        .iter()
        .map(|c| c.x)
        .min_by(|x1, x2| x1.cmp(x2))
        .unwrap();
    let max_x = coords
        .iter()
        .map(|c| c.x)
        .max_by(|x1, x2| x1.cmp(x2))
        .unwrap();

    let min_y = coords
        .iter()
        .map(|c| c.y)
        .min_by(|y1, y2| y1.cmp(y2))
        .unwrap();
    let max_y = coords
        .iter()
        .map(|c| c.y)
        .max_by(|y1, y2| y1.cmp(y2))
        .unwrap();

    (min_x, max_x, min_y, max_y)
}

pub fn answer1() {
    let coords = read_file();

    let (min_x, max_x, min_y, max_y) = find_min_max(&coords);

    // iterate over all x and all y, to compute all distances
    let mut grid = HashMap::new();
    let mut closest_coord = None;
    let mut closest_distance = max_x + max_y;
    for x in min_x..(max_x + 1) {
        for y in min_y..(max_y + 1) {
            let current_coord = Coord { x: x, y: y };
            for candidate in coords.iter() {
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
                    closest_distance = distance;
                    closest_coord = Some(candidate);
                }
            }

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
            counts.entry(closest).and_modify(|e| *e += 1).or_insert(1);
        }
    }

    let (_, count) = counts.iter().max_by_key(|(_, c)| *c).unwrap();
    println!("Chronal Coordinates (1/2): {}", count);
}

pub fn answer2() {
    let coords = read_file();
    let limit = 10000;

    let (min_x, max_x, min_y, max_y) = find_min_max(&coords);

    let mut nb_points_in_region = 0;
    for x in min_x..(max_x + 1) {
        for y in min_y..(max_y + 1) {
            let candidate = Coord { x: x, y: y };
            let sum = coords.iter().fold(0, |mut sum, c| {
                sum += candidate.man_distance(c);
                sum
            });
            if sum < limit {
                nb_points_in_region += 1;
            }
        }
    }

    println!("Chronal Coordinates (2/2): {}", nb_points_in_region);
}
