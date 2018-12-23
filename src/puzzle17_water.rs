use lazy_static::lazy_static;
use regex::Regex;
use std::cmp::{max, min};
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct Vein {
    x_start: u32,
    x_end: u32,
    y_start: u32,
    y_end: u32,
}

impl Vein {
    fn from_str(s: &str) -> Vein {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"([xy]{1})=(\d+), ([xy]{1})=(\d+)..(\d+)").unwrap();
        }
        let c = RE
            .captures(s)
            .ok_or_else(|| format!("cannot parse date {:?}", s))
            .unwrap();

        let first: char = c[1].parse().unwrap();
        let first_v: u32 = c[2].parse().unwrap();
        let _second: char = c[3].parse().unwrap();
        let second_s: u32 = c[4].parse().unwrap();
        let second_e: u32 = c[5].parse().unwrap();

        if first == 'x' {
            Vein {
                x_start: first_v,
                x_end: first_v,
                y_start: second_s,
                y_end: second_e,
            }
        } else {
            Vein {
                x_start: second_s,
                x_end: second_e,
                y_start: first_v,
                y_end: first_v,
            }
        }
    }
}

struct Field {
    field: Vec<Vec<bool>>,
}

impl Field {
    fn from_veins(veins: &Vec<Vein>) -> Field {
        let min_x = veins.iter().map(|v| min(v.x_start, v.x_end)).max().unwrap();
        let max_x = veins.iter().map(|v| max(v.x_start, v.x_end)).max().unwrap();

        Field {
            field: vec![vec![]],
        }
    }
}

fn read_file(filename: &str) -> Vec<Vein> {
    let file = File::open(filename).expect("cannot open file");
    let reader = BufReader::new(file);

    reader
        .lines()
        .filter_map(|result| result.ok())
        .map(|s| Vein::from_str(&s))
        .collect()
}

pub fn answer1() {
    let veins = read_file("input/input17.txt");
    println!("veins: {:?}", veins);

    println!("Reservoir Research (1/2): {}", 0);
}

#[test]
fn test_parse() {
    let veins = read_file("input/input17_debug.txt");
}
