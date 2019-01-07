use crate::common::error::AocError;
use crate::common::response::AocResponse;

use self::State::*;
use lazy_static::lazy_static;
use regex::Regex;
use std::cmp::{max, min};
use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct Vein {
    x_start: u32,
    x_end: u32,
    y_start: u32,
    y_end: u32,
}

impl Vein {
    fn from_str(s: &str) -> Vein {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"([xy]{1})=(\d+), ([xy]{1})=(\d+)..(\d+)").unwrap();
        }
        let c = RE
            .captures(s)
            .ok_or_else(|| format!("cannot parse date {:?}", s))
            .unwrap();

        let first: char = c[1].parse().unwrap();
        let first_v: u32 = c[2].parse().unwrap();
        let _second: char = c[3].parse().unwrap();
        let second_s: u32 = c[4].parse().unwrap();
        let second_e: u32 = c[5].parse().unwrap();

        if first == 'x' {
            Vein {
                x_start: first_v,
                x_end: first_v,
                y_start: second_s,
                y_end: second_e,
            }
        } else {
            Vein {
                x_start: second_s,
                x_end: second_e,
                y_start: first_v,
                y_end: first_v,
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Flow {
    Down,
    Side,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum State {
    Sand,
    Clay,
    Still,
    Flow,
}

struct Ground {
    field: Vec<Vec<State>>,
    x_min: usize,
    x_max: usize,
    y_min: usize,
    y_max: usize,
}

impl Ground {
    fn from_veins(veins: &[Vein]) -> Ground {
        let x_min = veins.iter().map(|v| min(v.x_start, v.x_end)).min().unwrap() as usize;
        let x_max = veins.iter().map(|v| max(v.x_start, v.x_end)).max().unwrap() as usize;
        let y_min = veins.iter().map(|v| min(v.y_start, v.y_end)).min().unwrap() as usize;
        let y_max = veins.iter().map(|v| max(v.y_start, v.y_end)).max().unwrap() as usize;

        let mut field = vec![vec![State::Sand; x_max + 2]; y_max + 1];
        for vein in veins {
            for x in vein.x_start..=vein.x_end {
                for y in vein.y_start..=vein.y_end {
                    field[y as usize][x as usize] = State::Clay;
                }
            }
        }

        Ground {
            field,
            x_min,
            x_max,
            y_min,
            y_max,
        }
    }

    // Recursively fill the grid in given direction, starting at given point.
    fn fill(&mut self, x: usize, y: usize, flow: Flow) {
        //println!("fill: x={}, y={}, flow={:?}", x, y, flow);
        //println!("{}", self);
        match flow {
            Flow::Down => {
                for dy in 1.. {
                    if y + dy > self.y_max {
                        break; // stop at the bottom of the grid
                    }
                    match self.field[y + dy][x] {
                        Sand => self.field[y + dy][x] = Flow,
                        Flow => break,
                        Clay | Still => {
                            self.fill(x, y + dy - 1, Flow::Side);
                            break;
                        }
                    }
                }
            }
            Flow::Side => {
                self.field[y][x] = Flow;
                let mut limit = [0, 0];
                // dir == 0 try left, dir == 1, try right
                for (dir, limit_dir) in limit.iter_mut().enumerate() {
                    for dx in 1.. {
                        let new_x = if dir == 0 { x - dx } else { x + dx };
                        match self.field[y][new_x] {
                            Clay | Still => {
                                *limit_dir = new_x;
                                break;
                            }
                            Flow | Sand => {
                                self.field[y][new_x] = Flow;
                                if self.field[y + 1][new_x] == Sand {
                                    // if there is a space under me
                                    self.fill(new_x, y, Flow::Down);
                                    break;
                                }
                            }
                        }
                    }
                }
                // if we reached both side, make it still, and flow from the level above
                if limit[0] > 0 && limit[1] > 0 {
                    for cur_x in limit[0] + 1..limit[1] {
                        self.field[y][cur_x] = Still;
                    }
                    self.fill(x, y - 1, Flow::Side);
                }
            }
        }
    }

    fn count_water(&self) -> (u32, u32) {
        let mut still_count = 0;
        let mut flow_count = 0;
        for y in self.y_min..=self.y_max {
            for x in self.x_min - 1..=self.x_max + 1 {
                if self.field[y][x] == Still {
                    still_count += 1;
                } else if self.field[y][x] == Flow {
                    flow_count += 1;
                }
            }
        }

        (still_count, flow_count)
    }
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            State::Sand => write!(f, "."),
            State::Clay => write!(f, "#"),
            State::Still => write!(f, "~"),
            State::Flow => write!(f, "|"),
        }
    }
}

impl fmt::Display for Ground {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Ground:")?;
        for y in 0..=self.y_max {
            for x in self.x_min - 1..=self.x_max + 1 {
                write!(f, "{}", self.field[y][x])?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

fn read_file(filename: &str) -> Result<Vec<Vein>, AocError> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let mut file_lines = vec![];
    for line in reader.lines() {
        let line = line?;
        let line = Vein::from_str(&line);
        file_lines.push(line);
    }

    Ok(file_lines)
}

pub fn answer1() -> Result<AocResponse<u32>, AocError> {
    let veins = read_file("input/input17.txt")?;

    let mut ground = Ground::from_veins(&veins);
    ground.fill(500, 0, Flow::Down);
    //println!("{}", ground);

    let count = ground.count_water();

    Ok(AocResponse::new(
        17,
        1,
        "Reservoir Research",
        count.0 + count.1,
    ))
}

pub fn answer2() -> Result<AocResponse<u32>, AocError> {
    let veins = read_file("input/input17.txt")?;

    let mut ground = Ground::from_veins(&veins);
    ground.fill(500, 0, Flow::Down);

    let count = ground.count_water();
    Ok(AocResponse::new(17, 2, "Reservoir Research", count.0))
}

#[test]
fn test_parse() {
    let veins = read_file("input/input17_debug.txt").unwrap();

    let field = Ground::from_veins(&veins);

    println!("{}", field);
    assert_eq!(495, field.x_min);
    assert_eq!(506, field.x_max);
    //assert_eq!(1, field.y_min);
    assert_eq!(13, field.y_max);
}

#[test]
fn test_fill() {
    let veins = read_file("input/input17_debug.txt").unwrap();

    let mut ground = Ground::from_veins(&veins);
    println!("{}", ground);

    ground.fill(500, 0, Flow::Down);
    println!("{}", ground);
    assert_eq!((29, 28), ground.count_water());
}
