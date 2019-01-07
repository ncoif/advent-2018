use crate::common::error::AocError;
use crate::common::response::AocResponse;

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::Chars;

pub fn answer1() -> Result<AocResponse<i32>, AocError> {
    let inputs = read_file()?;

    let frequencies: Vec<HashMap<char, u32>> =
        inputs.iter().map(|s| frequencies(s.chars())).collect();

    let (double, triple) = frequencies
        .iter()
        .fold((0, 0), |(mut double, mut triple), freq| {
            if freq.values().any(|&v| v == 2) {
                double += 1;
            }
            if freq.values().any(|&v| v == 3) {
                triple += 1;
            }
            (double, triple)
        });

    Ok(AocResponse::new(
        2,
        1,
        "Inventory Management System",
        double * triple,
    ))
}

pub fn answer2() -> Result<AocResponse<String>, AocError> {
    let inputs = read_file()?;

    let mut result = None;

    'outer: for x in &inputs {
        for y in &inputs {
            let common = common_letters(&x, &y);
            if common.len() == x.len() - 1 {
                result = Some(common);
                break 'outer;
            }
        }
    }

    match result {
        None => Err(AocError::ComputeNotFound),
        Some(r) => Ok(AocResponse::new(2, 2, "Inventory Management System", r)),
    }
}

fn read_file() -> Result<Vec<String>, AocError> {
    let filename = "input/input2.txt";
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let mut file_lines = vec![];
    for line in reader.lines() {
        let line = line?;
        file_lines.push(line);
    }

    Ok(file_lines)
}

fn frequencies(s: Chars) -> HashMap<char, u32> {
    let mut frequencies = HashMap::new();

    for c in s {
        frequencies.entry(c).and_modify(|e| *e += 1).or_insert(1);
    }

    frequencies
}

fn common_letters(s1: &str, s2: &str) -> String {
    s1.chars()
        .zip(s2.chars())
        .filter(|(c1, c2)| c1 == c2)
        .map(|(c1, _c2)| c1)
        .collect()
}

#[test]
fn frequencies_no_duplicates() {
    let frequencies = frequencies("abcdef".chars());
    assert_eq!(frequencies.get(&'a'), Some(&1));
    assert_eq!(frequencies.get(&'b'), Some(&1));
    assert_eq!(frequencies.get(&'g'), None);
}

#[test]
fn frequencies_with_duplicates() {
    let frequencies = frequencies("bababc".chars());
    assert_eq!(frequencies.get(&'a'), Some(&2));
    assert_eq!(frequencies.get(&'b'), Some(&3));
    assert_eq!(frequencies.get(&'c'), Some(&1));
}

#[test]
fn common_letters_test() {
    assert_eq!(
        common_letters(&String::from("abc"), &String::from("abd")),
        String::from("ab")
    );
    assert_eq!(
        common_letters(&String::from("abc"), &String::from("bbc")),
        String::from("bc")
    );
}
