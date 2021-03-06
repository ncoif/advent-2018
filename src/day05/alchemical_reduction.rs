use crate::common::error::AocError;
use crate::common::response::AocResponse;

use std::collections::{HashSet, VecDeque};
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
    c1 != c2 && c1.eq_ignore_ascii_case(&c2)
}

fn reduce1(poly: &str) -> usize {
    let mut tail = poly.chars().collect::<VecDeque<_>>();
    let mut head: VecDeque<_> = VecDeque::new();

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

fn reduce2(poly: &str) -> usize {
    let letters: HashSet<_> = poly.chars().map(|c| c.to_ascii_uppercase()).collect();

    let mut minimal_length = poly.len();
    for l in letters {
        let current_poly: String = poly
            .chars()
            .filter(|c| c.to_ascii_uppercase() != l)
            .collect();
        let candidate = reduce1(&current_poly);
        if candidate < minimal_length {
            minimal_length = candidate;
        }
    }

    minimal_length
}

pub fn answer1() -> Result<AocResponse<usize>, AocError> {
    let poly = read_file()?;
    let reduce1 = reduce1(&poly);

    Ok(AocResponse::new(5, 1, "Alchemical Reduction", reduce1))
}

pub fn answer2() -> Result<AocResponse<usize>, AocError> {
    let poly = read_file()?;
    let reduce2 = reduce2(&poly);

    Ok(AocResponse::new(5, 2, "Alchemical Reduction", reduce2))
}

#[test]
fn test_answer1() {
    assert_eq!(10, reduce1("dabAcCaCBAcCcaDA"));
}

#[test]
fn test_answer2() {
    assert_eq!(4, reduce2("dabAcCaCBAcCcaDA"));
}
