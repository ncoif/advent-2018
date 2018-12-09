use lazy_static::lazy_static;
use regex::Regex;
use std::collections::BTreeSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(PartialEq, Eq, Hash, Debug)]
struct Dependency {
    before: char,
    after: char,
}

impl FromStr for Dependency {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"Step ([A-Z]) must be finished before step ([A-Z]) can begin.")
                    .unwrap();
        }
        let c = RE
            .captures(s)
            .ok_or_else(|| format!("cannot parse coord {:?}", s))
            .unwrap();

        let before: char = c[1].parse().unwrap();
        let after: char = c[2].parse().unwrap();

        Ok(Dependency {
            before: before,
            after: after,
        })
    }
}

fn instruction_order(prereqs: &mut Vec<Dependency>) -> String {
    let mut unvisited = BTreeSet::new(); // guarantee that elements will be sorted
    for p in prereqs.iter() {
        unvisited.insert(p.before);
        unvisited.insert(p.after);
    }
    let mut output = String::with_capacity(unvisited.len());
    // find the next step, i.e. the first step (alphabetically) which is a prerequiste for nothing
    while let Some(&next_step) = unvisited
        .iter()
        .find(|&&step| !prereqs.iter().any(|prereq| prereq.after == step))
    {
        // process it
        output.push(next_step as char);
        // and remove all it's dependencies from the dependency set
        // i.e retains only dependencies that were not depending of it
        prereqs.retain(|p| p.before != next_step);
        unvisited.remove(&next_step);
    }
    output
}

fn read_file() -> Vec<Dependency> {
    //let filename = "input/input7_debug.txt";
    let filename = "input/input7.txt";
    let file = File::open(filename).expect("cannot open file");
    let reader = BufReader::new(file);

    reader
        .lines()
        .filter_map(|result| result.ok())
        .map(|s| Dependency::from_str(&s).unwrap())
        .collect()
}

pub fn answer1() {
    let mut dependencies = read_file();
    let order = instruction_order(&mut dependencies);
    println!("answer1: {}", order);
}
