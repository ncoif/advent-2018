use crate::common::error::AocError;
use crate::common::response::AocResponse;

use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

type Frequency = i32;

pub fn answer1() -> Result<AocResponse<Frequency>, AocError> {
    let mut current_frequency = 0;
    let mut seen = HashSet::new();
    let mut first_seen = None;

    let inputs = read_file()?;
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

    match first_seen {
        None => Err(AocError::ComputeNotFound),
        Some(freq) => Ok(AocResponse::new(1, 1, "Chronal Calibration", freq)),
    }
}

pub fn answer2() -> Result<AocResponse<Frequency>, AocError> {
    let mut current_frequency = 0;
    let mut seen = HashSet::new();
    let mut first_seen = None;
    let mut final_frequency = None;

    let inputs = read_file()?;
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

    match final_frequency {
        None => Err(AocError::ComputeNotFound),
        Some(freq) => Ok(AocResponse::new(1, 2, "Chronal Calibration", freq)),
    }
}

fn read_file() -> Result<Vec<Frequency>, AocError> {
    let filename = "input/input1.txt";
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let mut file_lines = vec![];
    for line in reader.lines() {
        let line = line?;
        let line = line.parse::<i32>()?;
        file_lines.push(line);
    }

    Ok(file_lines)
}
