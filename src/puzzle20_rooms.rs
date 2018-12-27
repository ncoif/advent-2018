use regex_syntax::hir::{Hir, HirKind};
use regex_syntax::ParserBuilder;
use std::collections::HashMap;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Coord {
    x: i64,
    y: i64,
}

impl Coord {
    fn mv(self, direction: char) -> Result<Coord, ()> {
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
            _ => Err(()),
        }
    }
}

// discovering some kind of type alias in rust
type Distances = HashMap<Coord, usize>;

fn distances(expr: &Hir, dists: &mut Distances, c: Coord) {}

fn build_distances(s: &str) -> Distances {
    // use regex-syntax to build a high-level intermediate representation ("HIR") of regular expression
    let expr = ParserBuilder::new()
        .nest_limit(1000)
        .build()
        .parse(s.trim())
        .unwrap();

    let mut dists = Distances::new();
    let origin = Coord { x: 0, y: 0 };
    dists.insert(origin, 0);

    distances(&expr, &mut dists, origin);

    dists
}

fn max_distance(dists: &Distances) -> usize {
    *dists.values().max().unwrap()
}

pub fn answer1() {
    let s = std::fs::read_to_string("input/input20.txt").expect("cannot read file");

    println!("A Regular Map (1/2): {:?}", 0);
}

#[test]
fn test_distance_1() {
    let s = "^WNE$";
    let dists = build_distances(s);
    let result = max_distance(&dists);

    assert_eq!(3, result);
}

#[test]
fn test_distance_2() {
    let s = "^ENWWW(NEEE|SSE(EE|N))$";
    let dists = build_distances(s);
    let result = max_distance(&dists);

    assert_eq!(10, result);
}

#[test]
fn test_distance_3() {
    let s = "^ENNWSWW(NEWS|)SSSEEN(WNSE|)EE(SWEN|)NNN$";
    let dists = build_distances(s);
    let result = max_distance(&dists);

    assert_eq!(18, result);
}
