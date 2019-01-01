use lazy_static::lazy_static;
use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::num::ParseIntError;
use std::str::FromStr;
use z3::{Ast, Config, Context, Optimize};

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
struct Nanobot {
    x: i64,
    y: i64,
    z: i64,
    r: u64,
}

impl FromStr for Nanobot {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"pos=<(\-*\d+),(\-*\d+),(\-*\d+)>, r=(\d+)").unwrap();
        }
        let c = RE
            .captures(s)
            .ok_or_else(|| format!("cannot parse string {:?}", s))
            .unwrap();

        let x: i64 = c[1].parse().unwrap();
        let y: i64 = c[2].parse().unwrap();
        let z: i64 = c[3].parse().unwrap();
        let r: u64 = c[4].parse().unwrap();

        Ok(Nanobot { x, y, z, r })
    }
}

impl Nanobot {
    fn distance_to(&self, other: &Nanobot) -> u64 {
        ((self.x - other.x).abs() + (self.y - other.y).abs() + (self.z - other.z).abs()) as u64
    }

    // a copy of all the nanobots in range from the given list
    fn in_range(&self, others: &Vec<Nanobot>) -> Vec<Nanobot> {
        others
            .iter()
            .filter(|n| self.distance_to(&n) <= self.r)
            .map(|n| n.clone())
            .collect()
    }
}

fn zabssub<'ctx>(ctx: &'ctx Context, zv: &Ast<'ctx>, v: i64) -> Ast<'ctx> {
    zabs(&ctx, &zv.sub(&[&Ast::from_i64(&ctx, v)]))
}

fn zabs<'ctx>(ctx: &'ctx Context, v: &Ast<'ctx>) -> Ast<'ctx> {
    v.ge(&Ast::from_i64(ctx, 0)).ite(v, &v.minus())
}

fn read_file(filename: &str) -> Vec<Nanobot> {
    let file = File::open(filename).expect("cannot open file");
    let reader = BufReader::new(file);

    reader
        .lines()
        .filter_map(|result| result.ok())
        .map(|s| Nanobot::from_str(&s))
        .filter_map(|result| result.ok())
        .collect()
}

fn opmitize_solver(nanobots: &Vec<Nanobot>) -> i64 {
    let ctx = Context::new(&Config::new());
    let zx = ctx.named_int_const("x");
    let zy = ctx.named_int_const("y");
    let zz = ctx.named_int_const("z");
    let mut in_ranges = Ast::from_i64(&ctx, 0);
    for nanobot in nanobots {
        in_ranges = zabssub(&ctx, &zx, nanobot.x)
            .add(&[
                &zabssub(&ctx, &zy, nanobot.y),
                &zabssub(&ctx, &zz, nanobot.z),
            ])
            .le(&Ast::from_i64(&ctx, nanobot.r as i64))
            .ite(&Ast::from_i64(&ctx, 1), &Ast::from_i64(&ctx, 0))
            .add(&[&in_ranges]);
    }

    let optimize = Optimize::new(&ctx);
    optimize.maximize(&in_ranges);

    let sum = zabs(&ctx, &zx).add(&[&zabs(&ctx, &zy), &zabs(&ctx, &zz)]);
    optimize.minimize(&sum);
    optimize.check();

    let sum = optimize
        .get_model()
        .eval(&sum)
        .ok_or("Variable not available")
        .unwrap()
        .as_i64()
        .ok_or("Variable not obtainable as i64")
        .unwrap()
        .abs();

    sum
}

pub fn answer1() {
    let nanobots = read_file("input/input23.txt");

    let max_radius_nanobot = nanobots.iter().max_by_key(|n| n.r).unwrap();
    let in_range = max_radius_nanobot.in_range(&nanobots);

    println!(
        "Day 23: Experimental Emergency Teleportation (1/2): {:?}",
        in_range.len()
    );
}

pub fn answer2() {
    let nanobots = read_file("input/input23.txt");

    let sum = opmitize_solver(&nanobots);

    println!("Experimental Emergency Teleportation (2/2): {:?}", sum);
}

#[test]
fn test_nanobot_distance() {
    let origin = Nanobot {
        x: 0,
        y: 0,
        z: 0,
        r: 0,
    };
    let n = Nanobot {
        x: 1,
        y: 0,
        z: 0,
        r: 0,
    };
    assert_eq!(1, origin.distance_to(&n));

    let n = Nanobot {
        x: 1,
        y: 1,
        z: 1,
        r: 0,
    };
    assert_eq!(3, origin.distance_to(&n));

    let n = Nanobot {
        x: 1,
        y: 3,
        z: 1,
        r: 0,
    };
    assert_eq!(5, origin.distance_to(&n));
}

#[test]
fn test_in_range() {
    let nanobots = read_file("input/input23_debug.txt");

    let max_radius_nanobot = nanobots.iter().max_by_key(|n| n.r).unwrap();
    let in_range = max_radius_nanobot.in_range(&nanobots);

    println!("max_radius_nanobot: {:?}", max_radius_nanobot);
    println!("in_range: {:?}", in_range);

    assert_eq!(4, max_radius_nanobot.r);
    assert_eq!(7, in_range.len());
}

#[test]
fn test_optimize() {
    let nanobots = read_file("input/input23_debug2.txt");

    assert_eq!(36, opmitize_solver(&nanobots));
}
