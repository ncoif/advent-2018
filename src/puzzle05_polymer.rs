use std::collections::{HashSet, LinkedList};
use std::fs::File;
use std::io::{BufReader, Read};

fn read_file() -> String {
    let filename = "input/input5.txt";
    //let filename = "input/input5_debug.txt";
    let file = File::open(filename).expect("cannot open file");
    let mut reader = BufReader::new(file);

    let mut initial_polymer = String::new();
    reader
        .read_to_string(&mut initial_polymer)
        .expect("error reading the file");

    String::from(initial_polymer.trim())
}

fn char_matches(c1: char, c2: char) -> bool {
    c1.eq_ignore_ascii_case(&c2) && c1 != c2
}

fn reduce1(poly: String) -> usize {
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

pub fn answer1() {
    let poly = read_file();
    let reduce1 = reduce1(poly);

    println!("Alchemical Reduction (1/2): {}", reduce1);
}

pub fn answer2() {
    let poly = read_file();

    let mut letters = HashSet::new();
    for c in poly.chars() {
        letters.insert(c.to_ascii_uppercase());
    }
    let mut minimal_length = poly.len();
    for l in letters {
        let current_poly = poly.replace(|a: char| a.to_ascii_uppercase() == l, "");
        let candidate = reduce1(current_poly);
        if candidate < minimal_length {
            minimal_length = candidate;
        }
    }
    println!("Alchemical Reduction (2/2): {}", minimal_length);
}
