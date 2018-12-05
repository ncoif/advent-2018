use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::num::ParseIntError;
use std::str::FromStr;

use std::cmp::Ordering;

#[derive(Ord, PartialOrd, Eq, PartialEq)]
struct Date {
    year: u32,
    month: u32,
    day: u32,
    hour: u32,
    minute: u32,
}

impl FromStr for Date {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"\[([0-9]{4})-([0-9]{2})-([0-9]{2}) ([0-9]{2}):([0-9]{2})\]").unwrap();
        }
        let c = RE
            .captures(s)
            .ok_or_else(|| format!("cannot parse date {:?}", s))
            .unwrap();

        let year: u32 = c[1].parse().unwrap();
        let month: u32 = c[2].parse().unwrap();
        let day: u32 = c[3].parse().unwrap();
        let hour: u32 = c[4].parse().unwrap();
        let minute: u32 = c[5].parse().unwrap();

        Ok(Date {
            year: year,
            month: month,
            day: day,
            hour: hour,
            minute: minute,
        })
    }
}

#[derive(Ord, PartialOrd, Eq, PartialEq)]
struct Guard(u32);

impl FromStr for Guard {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"Guard #([0-9]+)").unwrap();
        }
        let c = RE
            .captures(s)
            .ok_or_else(|| format!("cannot parse guard {:?}", s))
            .unwrap();

        let id: u32 = c[1].parse().unwrap();
        Ok(Guard(id))
    }
}

#[derive(Ord, PartialOrd, Eq, PartialEq)]
enum Status {
    ShiftStart(Guard),
    WakesUp,
    FallsAsleep,
}

impl FromStr for Status {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        if s == "wakes up" {
            Ok(Status::WakesUp)
        } else if s == "falls asleep" {
            Ok(Status::FallsAsleep)
        } else {
            Ok(Status::ShiftStart(Guard::from_str(s).unwrap()))
        }
    }
}

#[derive(PartialEq, Eq)]
struct Event {
    date: Date,
    status: Status,
}

impl FromStr for Event {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Event {
            date: Date::from_str(&s[0..18]).unwrap(),
            status: Status::from_str(&s[19..]).unwrap(),
        })
    }
}

impl Ord for Event {
    fn cmp(&self, other: &Event) -> Ordering {
        self.date.cmp(&other.date)
    }
}

impl PartialOrd for Event {
    fn partial_cmp(&self, other: &Event) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

pub fn run() {
    let mut inputs = read_file();
    inputs.sort();
}

fn read_file<'a>() -> Vec<Event> {
    let filename = "input/input4.txt";
    let file = File::open(filename).expect("cannot open file");
    let reader = BufReader::new(file);

    reader
        .lines()
        .filter_map(|result| result.ok())
        .map(|s| Event::from_str(&s).unwrap())
        .collect()
}
