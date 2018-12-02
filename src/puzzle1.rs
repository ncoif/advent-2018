use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub(crate) fn run() {
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

    println!("First seen frequency: {:?}", first_seen);
    println!("Final frequency: {:?}", final_frequency);
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
