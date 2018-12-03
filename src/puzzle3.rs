use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

// represent a single entry, for example #1 @ 1,3: 4x4
#[derive(Debug)]
pub struct Area {
    id: u64,
    x: u64,
    y: u64,
    width: u64,
    length: u64,
}

lazy_static! {
    static ref RE: Regex =
        Regex::new(r"#(?P<id>\d+) @ (?P<x>\d+),(?P<y>\d+): (?P<width>\d+)x(?P<length>\d+)")
            .unwrap();
}

impl Area {
    pub fn from_str(text: &str) -> Self {
        for caps in RE.captures_iter(text) {
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

    pub fn keys(&self) -> Vec<String> {
        let mut result = Vec::new();
        for i in 0..self.width {
            for j in 0..self.length {
                result.push(format!("{}x{}", self.x + i, self.y + j));
            }
        }
        result
    }

    pub fn intersect(&self, area: &Area) -> bool {
        let mut points = HashSet::new();
        for key in self.keys() {
            points.insert(key);
        }

        for key in area.keys() {
            if points.contains(&key) {
                return true;
            }
        }
        return false;
    }
}

pub fn run() {
    let inputs = read_file();

    {
        let mut points = HashMap::new();
        for area in &inputs {
            for key in area.keys() {
                let count = points.entry(key).or_insert(0);
                *count += 1;
            }
        }

        points.retain(|_k, v| *v >= 2);
        println!("Answer1: {}", points.len());
    }

    {
        'candidat: for candidat in &inputs {
            //println!("evaluating {:?}", candidat);
            for area in &inputs {
                if candidat.id != area.id && candidat.intersect(&area) {
                    //println!("{:?} intersects with {:?}", candidat.id, area.id);
                    continue 'candidat;
                }
            }
            // candidat intersect with no-one
            println!("Answer2: {:?}", candidat);
            return;
        }
    }
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
