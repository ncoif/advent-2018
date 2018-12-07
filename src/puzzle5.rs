use std::collections::HashSet;
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
    (c1.is_uppercase() && c2.is_lowercase() && c2.to_ascii_uppercase() == c1)
        || (c2.is_uppercase() && c1.is_lowercase() && c1.to_ascii_uppercase() == c2)
}

fn reduce1(poly: String) -> usize {
    let mut current_poly = poly;
    let mut was_modified = true;
    while was_modified {
        was_modified = false;
        let mut new_poly = String::new();
        let mut last_char = ' ';
        for c in current_poly.chars() {
            if last_char == ' ' {
                last_char = c;
            } else {
                if !char_matches(c, last_char) {
                    new_poly.push(last_char);
                    last_char = c;
                } else {
                    was_modified = true;
                    last_char = ' ';
                }
            }
        }
        if last_char != ' ' {
            new_poly.push(last_char)
        }
        current_poly = new_poly;
    }

    current_poly.len()
}

pub fn answer1() {
    let poly = read_file();
    let reduce1 = reduce1(poly);

    println!("Answer1: {}", reduce1);
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
    println!("Answer2: {}", minimal_length);
}
