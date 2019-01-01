use regex_syntax::hir::{self, Hir, HirKind};
use regex_syntax::ParserBuilder;
use std::cmp;
use std::collections::HashMap;
use std::error::Error;
use std::result;

type Result<T> = result::Result<T, Box<Error>>;

macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<Error>::from(format!($($tt)*))) }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Coord {
    x: i64,
    y: i64,
}

impl Coord {
    fn mv(self, direction: char) -> Result<Coord> {
        match direction {
            'N' => Ok(Coord {
                x: self.x,
                y: self.y - 1,
            }),
            'S' => Ok(Coord {
                x: self.x,
                y: self.y + 1,
            }),
            'W' => Ok(Coord {
                x: self.x - 1,
                y: self.y,
            }),
            'E' => Ok(Coord {
                x: self.x + 1,
                y: self.y,
            }),
            _ => err!("unexpected character"),
        }
    }
}

// discovering some kind of type alias in rust
type Distances = HashMap<Coord, usize>;

// TODO: trying a proper Result response, so it seems I must return something, so I'm returning something meaningless
fn distances(expr: &Hir, dists: &mut Distances, c: Coord) -> Result<Coord> {
    match *expr.kind() {
        HirKind::Literal(hir::Literal::Unicode(ch)) => {
            let nextc = c.mv(ch)?;
            let mut dist = dists[&c] + 1;
            if dists.contains_key(&nextc) {
                dist = cmp::min(dist, dists[&nextc])
            }
            dists.insert(nextc, dist);
            Ok(nextc)
        }
        HirKind::Group(ref g) => distances(&g.hir, dists, c),
        HirKind::Concat(ref exprs) => {
            let mut nextc = c;
            for e in exprs {
                nextc = distances(e, dists, nextc)?;
            }
            Ok(nextc)
        }
        HirKind::Alternation(ref exprs) => {
            for e in exprs {
                distances(e, dists, c)?;
            }
            Ok(c)
        }
        _ => Ok(c), // we don't care
    }
}

fn build_distances(s: &str) -> Result<Distances> {
    // use regex-syntax to build a high-level intermediate representation ("HIR") of regular expression
    // https://docs.rs/regex-syntax/0.6.4/regex_syntax/
    let expr = ParserBuilder::new()
        .nest_limit(1000)
        .build()
        .parse(s.trim())?;

    let mut dists = Distances::new();
    let origin = Coord { x: 0, y: 0 };
    dists.insert(origin, 0);

    distances(&expr, &mut dists, origin)?;

    Ok(dists)
}

fn max_distance(dists: &Distances) -> usize {
    *dists.values().max().unwrap()
}

pub fn answer1() {
    let s = std::fs::read_to_string("input/input20.txt").expect("cannot read file");
    let dists = build_distances(&s).unwrap();
    let result = max_distance(&dists);

    println!("Day 20: A Regular Map (1/2): {:?}", result);
}

pub fn answer2() {
    let s = std::fs::read_to_string("input/input20.txt").expect("cannot read file");
    let dists = build_distances(&s).unwrap();
    let rooms = dists.values().filter(|&&d| d >= 1000).count();

    println!("Day 20: A Regular Map (2/2): {:?}", rooms);
}

#[test]
fn test_distance_1() {
    let s = "^WNE$";
    let dists = build_distances(&s).unwrap();
    let result = max_distance(&dists);

    assert_eq!(3, result);
}

#[test]
fn test_distance_2() {
    let s = "^ENWWW(NEEE|SSE(EE|N))$";
    let dists = build_distances(&s).unwrap();
    let result = max_distance(&dists);

    assert_eq!(10, result);
}

#[test]
fn test_distance_3() {
    let s = "^ENNWSWW(NEWS|)SSSEEN(WNSE|)EE(SWEN|)NNN$";
    let dists = build_distances(&s).unwrap();
    let result = max_distance(&dists);

    assert_eq!(18, result);
}
