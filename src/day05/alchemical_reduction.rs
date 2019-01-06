use crate::common::error::AocError;
use crate::common::response::AocResponse;

use std::collections::{HashSet, LinkedList};
use std::fs::File;
use std::io::{BufReader, Read};

fn read_file() -> Result<String, AocError> {
    let filename = "input/input5.txt";
    let file = File::open(filename)?;
    let mut reader = BufReader::new(file);

    let mut initial_polymer = String::new();
    reader.read_to_string(&mut initial_polymer)?;

    Ok(String::from(initial_polymer.trim()))
}

fn char_matches(c1: char, c2: char) -> bool {
    c1.eq_ignore_ascii_case(&c2) && c1 != c2
}

fn reduce1(poly: &str) -> usize {
    let mut tail = poly.chars().collect::<LinkedList<_>>();
    let mut head: LinkedList<char> = LinkedList::new();

    while let Some(unit1) = tail.pop_front() {
        if let Some(unit2) = head.back() {
            if char_matches(unit1, *unit2) {
                head.pop_back();
            } else {
                head.push_back(unit1);
            }
        } else {
            head.push_back(unit1);
        }
    }

    head.len()
}

pub fn answer1() -> Result<AocResponse<usize>, AocError> {
    let poly = read_file()?;
    let reduce1 = reduce1(&poly);

    Ok(AocResponse::new(5, 1, "Alchemical Reduction", reduce1))
}

pub fn answer2() -> Result<AocResponse<usize>, AocError> {
    let poly = read_file()?;

    let mut letters = HashSet::new();
    for c in poly.chars() {
        letters.insert(c.to_ascii_uppercase());
    }
    let mut minimal_length = poly.len();
    for l in letters {
        let current_poly = poly.replace(|a: char| a.to_ascii_uppercase() == l, "");
        let candidate = reduce1(&current_poly);
        if candidate < minimal_length {
            minimal_length = candidate;
        }
    }

    Ok(AocResponse::new(
        5,
        2,
        "Alchemical Reduction",
        minimal_length,
    ))
}
