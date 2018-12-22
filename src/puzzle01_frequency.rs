use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn answer1() {
    let mut current_frequency = 0;
    let mut seen = HashSet::new();
    let mut first_seen = None;

    let inputs = read_file();
    while first_seen.is_none() {
        for input in &inputs {
            current_frequency += input;

            if first_seen.is_none() {
                if seen.contains(&current_frequency) {
                    first_seen = Some(current_frequency);
                }
                seen.insert(current_frequency);
            }
        }
    }

    println!("Chronal Calibration (1/2): {:?}", first_seen.unwrap());
}

pub fn answer2() {
    let mut current_frequency = 0;
    let mut seen = HashSet::new();
    let mut first_seen = None;
    let mut final_frequency = None;

    let inputs = read_file();
    while first_seen.is_none() {
        for input in &inputs {
            current_frequency += input;

            if first_seen.is_none() {
                if seen.contains(&current_frequency) {
                    first_seen = Some(current_frequency);
                }
                seen.insert(current_frequency);
            }
        }

        final_frequency.get_or_insert(current_frequency);
    }

    println!("Chronal Calibration (2/2): {:?}", final_frequency.unwrap());
}

fn read_file() -> Vec<i32> {
    let filename = "input/input1.txt";
    let file = File::open(filename).expect("cannot open file");
    let reader = BufReader::new(file);

    reader
        .lines()
        .filter_map(|result| result.ok())
        .map(|s| s.parse::<i32>().expect("invalid number"))
        .collect()
}
