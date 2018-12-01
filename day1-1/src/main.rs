use std::io::{BufRead, BufReader};
use std::fs::File;
use std::collections::HashSet;

fn main() {
    let mut current_frequency = 0;
    let mut seen = HashSet::new();
    let mut first_seen = None;
    let mut final_frequency = None;

    while first_seen.is_none() {
        let file = File::open("input.txt").expect("cannot open file");
        let file = BufReader::new(file);

        for line in file.lines().filter_map(|result| result.ok()) {
            let current = line.parse::<i32>().unwrap();
            current_frequency += current;
            if first_seen.is_none() && seen.contains(&current_frequency) {
                first_seen = Some(current_frequency);
            }
            seen.insert(current_frequency);
        }

        if final_frequency.is_none() {
            final_frequency = Some(current_frequency);
        }
    }

    println!("First seen frequency: {:?}", first_seen);
    println!("Final frequency: {:?}", final_frequency);
}

