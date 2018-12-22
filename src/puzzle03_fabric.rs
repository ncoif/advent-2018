use lazy_static::lazy_static;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};

// represent a single entry, for example #1 @ 1,3: 4x4
pub struct Area {
    id: u64,
    points: Box<HashSet<String>>,
}

lazy_static! {
    static ref RE: Regex =
        Regex::new(r"#(?P<id>\d+) @ (?P<x>\d+),(?P<y>\d+): (?P<width>\d+)x(?P<length>\d+)")
            .unwrap();
}

impl Area {
    pub fn from_str(text: &str) -> Self {
        for caps in RE.captures_iter(text) {
            let id = caps["id"].parse::<u64>().unwrap();
            let x = caps["x"].parse::<u64>().unwrap();
            let y = caps["y"].parse::<u64>().unwrap();
            let width = caps["width"].parse::<u64>().unwrap();
            let length = caps["length"].parse::<u64>().unwrap();

            let mut points = Box::new(HashSet::new());
            for i in 0..width {
                for j in 0..length {
                    points.insert(format!("{}x{}", x + i, y + j));
                }
            }

            return Area {
                id: id,
                points: points,
            };
        }
        unreachable!();
    }

    pub fn intersect(&self, area: &Area) -> bool {
        let intersect_set: HashSet<_> = self.points.intersection(&area.points).collect();
        !intersect_set.is_empty()
    }
}

impl fmt::Debug for Area {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Area {{ id: {}, number of points: {} }}",
            self.id,
            self.points.len()
        )
    }
}

pub fn answer1() {
    let inputs = read_file();

    let mut total_points = HashMap::new();
    for area in &inputs {
        for key in area.points.iter() {
            total_points.entry(key).and_modify(|e| *e += 1).or_insert(1);
        }
    }

    total_points.retain(|_k, v| *v >= 2);
    println!("No Matter How You Slice It (1/2): {}", total_points.len());
}

pub fn answer2() {
    let inputs = read_file();

    'candidat: for candidat in &inputs {
        //println!("evaluating {:?}", candidat);
        for area in &inputs {
            if candidat.id != area.id && candidat.intersect(&area) {
                //println!("{:?} intersects with {:?}", candidat.id, area.id);
                continue 'candidat;
            }
        }
        // candidat intersect with no-one
        println!("No Matter How You Slice It (2/2): {:?}", candidat.id);
        return;
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
