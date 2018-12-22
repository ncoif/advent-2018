use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::Chars;

pub fn answer1() {
    let inputs = read_file();

    let frequencies: Vec<HashMap<char, u32>> =
        inputs.iter().map(|s| frequencies(s.chars())).collect();

    let (double, triple) = frequencies
        .iter()
        .fold((0, 0), |(mut double, mut triple), freq| {
            if freq.values().find(|&v| *v == 2).is_some() {
                double += 1;
            }
            if freq.values().find(|&v| *v == 3).is_some() {
                triple += 1;
            }
            (double, triple)
        });

    println!("Inventory Management System (1/2): {}", double * triple);
}

pub fn answer2() {
    let inputs = read_file();

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

    println!("Inventory Management System (2/2): {:?}", result.unwrap());
}

fn read_file() -> Vec<String> {
    let filename = "input/input2.txt";
    let file = File::open(filename).expect("cannot open file");
    let reader = BufReader::new(file);

    reader.lines().filter_map(|result| result.ok()).collect()
}

fn frequencies(s: Chars) -> HashMap<char, u32> {
    let mut frequencies = HashMap::new();

    for c in s {
        frequencies.entry(c).and_modify(|e| *e += 1).or_insert(1);
    }

    frequencies
}

fn common_letters(s1: &String, s2: &String) -> String {
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
