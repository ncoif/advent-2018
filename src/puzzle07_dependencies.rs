use lazy_static::lazy_static;
use regex::Regex;
use std::collections::{BTreeSet, HashSet};
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

fn execute_in_parallel(prereqs: &mut Vec<Dependency>, workers: usize, offset: usize) -> usize {
    let mut steps = BTreeSet::new(); // guarantee that elements will be sorted
    for p in prereqs.iter() {
        steps.insert(p.before);
        steps.insert(p.after);
    }

    let total_tasks_nb = steps.len();

    // a dirty loop that iterate one second at a time until everything is done
    let mut started: Vec<(usize, char)> = Vec::new(); //(finish time, task)
    let mut done = HashSet::new();
    let mut time = 0;
    loop {
        // insert all completed tasks into done, and remove them from the started list
        for s in &started {
            if s.0 == time {
                done.insert(s.1);
                prereqs.retain(|p| p.before != s.1);
            }
        }
        started.retain(|job| job.0 > time);
        // if we did everything, then exit
        if done.len() == total_tasks_nb {
            break;
        }
        // for all available workers, add tasks
        while started.len() < workers {
            // find the next task available that is not started, is not finished, and doesn't depends on anything
            if let Some(&start) = steps.iter().find(|&&t| {
                // !started.iter().any(|s| s.1 == t) && !done.contains(&t) && !prereqs.iter().any(|prereq| prereq.after == t)
                !done.contains(&t) && !prereqs.iter().any(|prereq| prereq.after == t)
            }) {
                started.push((time + offset + (start as u8 - b'A' + 1) as usize, start));
                steps.remove(&start);
            } else {
                break;
            }
        }
        //println!("time: {} started: {:?} done: {:?}", time, started, done);
        time += 1;
    }
    time
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

pub fn answer2() {
    let mut dependencies = read_file();
    let total_time = execute_in_parallel(&mut dependencies, 5, 60);
    println!("answer2: {}", total_time);
}
