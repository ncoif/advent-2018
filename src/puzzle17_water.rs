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

struct Field {
    field: Vec<Vec<bool>>,
    x_min: usize,
    x_max: usize,
    y_min: usize,
    y_max: usize,
}

impl Field {
    fn from_veins(veins: &Vec<Vein>) -> Field {
        let x_min = veins.iter().map(|v| min(v.x_start, v.x_end)).min().unwrap() as usize;
        let x_max = veins.iter().map(|v| max(v.x_start, v.x_end)).max().unwrap() as usize;
        let y_min = veins.iter().map(|v| min(v.y_start, v.y_end)).min().unwrap() as usize;
        let y_max = veins.iter().map(|v| max(v.y_start, v.y_end)).max().unwrap() as usize;

        let mut field = vec![vec![false; x_max + 1]; y_max + 1];
        for vein in veins {
            for x in vein.x_start..=vein.x_end {
                for y in vein.y_start..=vein.y_end {
                    field[y as usize][x as usize] = true;
                }
            }
        }

        Field {
            field,
            x_min,
            x_max,
            y_min,
            y_max,
        }
    }
}

impl fmt::Display for Field {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Field:")?;
        for y in self.y_min..=self.y_max {
            for x in self.x_min..=self.x_max {
                let char = if self.field[y][x] { "#" } else { "." };
                write!(f, "{}", char)?;
            }
            writeln!(f, "")?;
        }

        Ok(())
    }
}

fn read_file(filename: &str) -> Vec<Vein> {
    let file = File::open(filename).expect("cannot open file");
    let reader = BufReader::new(file);

    reader
        .lines()
        .filter_map(|result| result.ok())
        .map(|s| Vein::from_str(&s))
        .collect()
}

pub fn answer1() {
    let veins = read_file("input/input17.txt");
    println!("veins: {:?}", veins);

    println!("Reservoir Research (1/2): {}", 0);
}

#[test]
fn test_parse() {
    let veins = read_file("input/input17_debug.txt");

    let field = Field::from_veins(&veins);

    println!("{}", field);
    assert_eq!(495, field.x_min);
    assert_eq!(506, field.x_max);
    assert_eq!(1, field.y_min);
    assert_eq!(13, field.y_max);
}
