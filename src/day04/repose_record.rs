use crate::common::error::AocError;
use crate::common::response::AocResponse;

use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
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
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"\[([0-9]{4})-([0-9]{2})-([0-9]{2}) ([0-9]{2}):([0-9]{2})\]").unwrap();
        }
        let c = RE
            .captures(s)
            .ok_or_else(|| format!("cannot parse date {:?}", s))?;

        let year: u32 = c[1].parse()?;
        let month: u32 = c[2].parse()?;
        let day: u32 = c[3].parse()?;
        let hour: u32 = c[4].parse()?;
        let minute: u32 = c[5].parse()?;

        Ok(Date {
            year,
            month,
            day,
            hour,
            minute,
        })
    }
}

impl Date {
    fn new() -> Date {
        Date {
            year: 0,
            month: 0,
            day: 0,
            hour: 0,
            minute: 0,
        }
    }

    fn duration_since(&self, date: &Date) -> u32 {
        let modulo = 60 * 24;
        (self.minute - date.minute + 60 * (self.hour - date.hour) % modulo + modulo) % modulo
    }
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug)]
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
        match s.trim() {
            "wakes up" => Ok(Status::WakesUp),
            "falls asleep" => Ok(Status::FallsAsleep),
            _ => Ok(Status::ShiftStart(Guard::from_str(s).unwrap())),
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

// find the worst guard, i.e. the one with the longuest sleep
fn sleep_times(events: &[Event]) -> Guard {
    let mut sleep_times_per_guards = HashMap::new();
    let mut current_guard = 0;
    let mut last_sleep_start = &Date::new();

    // compute a map of total slept time per guard
    for e in events {
        match &e.status {
            Status::ShiftStart(g) => current_guard = g.0,
            Status::FallsAsleep => last_sleep_start = &e.date,
            Status::WakesUp => {
                let sleep_time = sleep_times_per_guards.entry(current_guard).or_insert(0);
                *sleep_time += e.date.duration_since(&last_sleep_start);
            }
        }
    }

    // find the worst guard
    let mut max_guard = 0;
    let mut max_time = 0;
    for (guard, time) in sleep_times_per_guards.iter() {
        if *time > max_time {
            max_guard = *guard;
            max_time = *time;
        }
    }

    Guard(max_guard)
}

fn best_minute(events: &[Event], worst_guard: &Guard) -> u32 {
    let mut asleep_on = HashMap::new();
    let mut last_sleep_start = 0;
    let mut our_guard = false;
    for e in events {
        match &e.status {
            Status::ShiftStart(g) => {
                if g.0 == worst_guard.0 {
                    our_guard = true
                } else {
                    our_guard = false
                }
            }
            Status::FallsAsleep => {
                if our_guard {
                    last_sleep_start = e.date.minute
                }
            }
            Status::WakesUp => {
                if our_guard {
                    for m in last_sleep_start..e.date.minute {
                        asleep_on.entry(m).and_modify(|e| *e += 1).or_insert(1);
                    }
                }
            }
        }
    }

    //find the max
    let (&min, _) = asleep_on.iter().max_by_key(|(_, nb)| *nb).unwrap();
    min
}

fn most_frequently_asleep(events: &[Event]) -> (u32, u32) {
    let mut asleep_on = HashMap::new();
    let mut current_guard = 0;
    let mut last_sleep_start = 0;
    for e in events {
        match &e.status {
            Status::ShiftStart(g) => current_guard = g.0,
            Status::FallsAsleep => last_sleep_start = e.date.minute,
            Status::WakesUp => {
                for m in last_sleep_start..e.date.minute {
                    asleep_on
                        .entry((current_guard, m))
                        .and_modify(|e| *e += 1)
                        .or_insert(1);
                }
            }
        }
    }

    //find the max
    let (&(g, m), _) = asleep_on.iter().max_by_key(|(_, nb)| *nb).unwrap();
    (g, m)
}

pub fn answer1() -> Result<AocResponse<u32>, AocError> {
    let mut inputs = read_file()?;
    inputs.sort();

    let worst_guard = sleep_times(&inputs);
    //println!("worst guard: {:?}", worst_guard);

    let best_minute = best_minute(&inputs, &worst_guard);
    //println!("worst_minute: {:?}", best_minute);

    Ok(AocResponse::new(
        4,
        1,
        "Repose Record",
        worst_guard.0 * best_minute,
    ))
}

pub fn answer2() -> Result<AocResponse<u32>, AocError> {
    let mut inputs = read_file()?;
    inputs.sort();

    let solution2 = most_frequently_asleep(&inputs);

    Ok(AocResponse::new(
        4,
        2,
        "Repose Record",
        solution2.0 * solution2.1,
    ))
}

fn read_file() -> Result<Vec<Event>, AocError> {
    let filename = "input/input4.txt";
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let mut file_lines = vec![];
    for line in reader.lines() {
        let line = line?;
        let line = Event::from_str(&line)?;
        file_lines.push(line);
    }

    Ok(file_lines)
}
