use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(PartialEq, Eq, Hash, Debug)]
struct Edge {
    before: char,
    after: char,
}

impl FromStr for Edge {
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

        Ok(Edge {
            before: before,
            after: after,
        })
    }
}

#[derive(Debug)]
struct Graph {
    adj: Box<HashMap<char, Vec<char>>>, // adjacency list
}

impl Graph {

    fn new() -> Graph {
        let mut adj = Box::new(HashMap::new());
        Graph { adj: adj}
    }

    fn add_edge(&mut self, e: &Edge) {
        let adj_list = self.adj.entry(e.before).or_insert(Vec::new());
        adj_list.push(e.after);
    }
}

fn read_file() -> Vec<Edge> {
    let filename = "input/input7_debug.txt";
    //let filename = "input/input7.txt";
    let file = File::open(filename).expect("cannot open file");
    let reader = BufReader::new(file);

    reader
        .lines()
        .filter_map(|result| result.ok())
        .map(|s| Edge::from_str(&s).unwrap())
        .collect()
}

pub fn answer1() {
    let edge = read_file();

    let mut graph = Graph::new();
    for e in edge.iter() {
        println!("{:?}", e);
    }
}
