use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::FromIterator;
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
        let list = Box::new(HashSet::new());
        let adj = Box::new(HashMap::new());
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

    fn find_deps(&self, v: &char) -> Vec<char> {
        self.adj.get(v).unwrap_or(&Vec::new()).to_vec()
    }

    // return true if all dep for this letter have been seen
    fn all_seen(&self, v: &char, visited: &HashSet<char>) -> bool {
        let mut dependencies: HashSet<_> =
            HashSet::from_iter(self.adj.get(v).unwrap().iter().clone());
        for current in visited {
            dependencies.remove(&current);
        }
        dependencies.is_empty()
    }

    fn scan_graph_alphabetically(&self) -> Vec<char> {
        let mut candidates = Vec::new();
        let mut visited = HashSet::new();
        let mut response = Vec::new();

        let starting_node = self.find_starting_node();
        response.push(starting_node);
        visited.insert(starting_node);
        for deps in self.find_deps(&starting_node) {
            candidates.push(deps);
        }

        while !candidates.is_empty() {
            // sort candidates alphabetically
            candidates.sort_by(|a, b| b.cmp(a));
            println!("{:?}", candidates);
            println!("{:?}", visited);

            let current_node_option = candidates.pop();
            if current_node_option.is_some() {
                let current_node = current_node_option.unwrap();

                println!("all_seen: {:?}", self.all_seen(&current_node, &visited));

                if self.all_seen(&current_node, &visited) {
                    response.push(current_node);
                    visited.insert(current_node);
                    for deps in self.find_deps(&current_node) {
                        candidates.push(deps);
                    }
                }
            }
        }

        response
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

    let response = graph.scan_graph_alphabetically();
    print!("answer1: ");
    for c in response {
        print!("{}", c);
    }
    println!("")
}
