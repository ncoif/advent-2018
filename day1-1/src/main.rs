use std::io::{BufRead, BufReader};
use std::fs::File;

fn main() {
    let file = File::open("input.txt").expect("cannot open file");
    let file = BufReader::new(file);
    for line in file.lines() {
        println!("{}", line.unwrap());
    }
}
