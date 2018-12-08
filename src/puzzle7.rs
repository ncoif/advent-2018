use regex::Regex;
use std::collections::{HashMap, HashSet};
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

#[derive(Debug)]
struct Graph {
    list: Box<HashSet<char>>,           // list of available nodes
    adj: Box<HashMap<char, Vec<char>>>, // adjacency list
}

impl Graph {
    fn new() -> Graph {
        let mut list = Box::new(HashSet::new());
        let mut adj = Box::new(HashMap::new());
        Graph {
            list: list,
            adj: adj,
        }
    }

    fn add_edge(&mut self, d: &Dependency) {
        self.list.insert(d.before);
        self.list.insert(d.after);
        let adj_list = self.adj.entry(d.before).or_insert(Vec::new());
        adj_list.push(d.after);
    }

    /// find the node without dependency, throw error if multiple of them
    fn find_starting_node(&self) -> char {
        let mut nodes = self.list.clone();
        for vect in self.adj.values() {
            for v in vect {
                nodes.remove(v);
            }
        }
        *nodes
            .iter()
            .next()
            .ok_or("expected single valid starting node")
            .unwrap()
    }
}

fn read_file() -> Vec<Dependency> {
    let filename = "input/input7_debug.txt";
    //let filename = "input/input7.txt";
    let file = File::open(filename).expect("cannot open file");
    let reader = BufReader::new(file);

    reader
        .lines()
        .filter_map(|result| result.ok())
        .map(|s| Dependency::from_str(&s).unwrap())
        .collect()
}

pub fn answer1() {
    let dependencies = read_file();

    let mut graph = Graph::new();
    for d in dependencies.iter() {
        graph.add_edge(d);
    }
    println!("graph: {:?}", graph);

    let starting_node = graph.find_starting_node();
    println!("starting node: {}", starting_node);
}
