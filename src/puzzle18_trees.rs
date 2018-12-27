use itertools::Itertools;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::fmt;
use std::mem;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
enum Acre {
    Open,
    Trees,
    Lumberyard,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
struct World {
    grid: Vec<Vec<Acre>>,
}

impl fmt::Display for Acre {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Acre::Open => write!(f, "."),
            Acre::Trees => write!(f, "|"),
            Acre::Lumberyard => write!(f, "#"),
        }
    }
}

impl FromStr for World {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s.split("\n");
        let grid = lines
            .filter(|line| *line != "")
            .map(|line| {
                line.chars()
                    .map(|ch| match ch {
                        '.' => Acre::Open,
                        '|' => Acre::Trees,
                        '#' => Acre::Lumberyard,
                        _ => panic!("invalid state"),
                    })
                    .collect_vec()
            })
            .collect_vec();

        Ok(World { grid })
    }
}

impl fmt::Display for World {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.grid.iter() {
            for c in line.iter() {
                write!(f, "{}", c)?;
            }
            writeln!(f, "")?;
        }

        Ok(())
    }
}

impl World {
    fn step(&mut self) {
        let mut new = vec![vec![Acre::Open; self.width()]; self.height()];
        for y in 0..self.height() {
            for x in 0..self.width() {
                self.step_cell(x, y, &mut new);
            }
        }
        mem::replace(&mut self.grid, new);
    }

    fn step_cell(&self, x: usize, y: usize, new: &mut Vec<Vec<Acre>>) {
        use self::Acre::*;

        new[y][x] = self.grid[y][x];
        match self.grid[y][x] {
            Open => {
                let adj_trees =
                    self.count_neighbors(
                        x,
                        y,
                        0,
                        |count, cell| {
                            if cell == Trees {
                                count + 1
                            } else {
                                count
                            }
                        },
                    );
                if adj_trees >= 3 {
                    new[y][x] = Trees;
                }
            }
            Trees => {
                let adj_lumber = self.count_neighbors(x, y, 0, |count, cell| {
                    if cell == Lumberyard {
                        count + 1
                    } else {
                        count
                    }
                });
                if adj_lumber >= 3 {
                    new[y][x] = Lumberyard;
                }
            }
            Lumberyard => {
                let (has_lumber, has_trees) =
                    self.count_neighbors(x, y, (false, false), |(lumber, trees), n| {
                        (lumber || n == Lumberyard, trees || n == Trees)
                    });
                if has_lumber && has_trees {
                    new[y][x] = Lumberyard;
                } else {
                    new[y][x] = Open;
                }
            }
        }
    }

    // apply the fold f on the neightbours of (ox, oy)
    fn count_neighbors<T>(
        &self,
        ox: usize,
        oy: usize,
        init: T,
        mut f: impl FnMut(T, Acre) -> T,
    ) -> T {
        let mut ret = init;
        for y in oy.saturating_sub(1)..=oy.saturating_add(1) {
            for x in ox.saturating_sub(1)..=ox.saturating_add(1) {
                if x == ox && y == oy {
                    continue;
                }
                if x >= self.width() || y >= self.height() {
                    continue;
                }
                ret = f(ret, self.grid[y][x]);
            }
        }
        ret
    }

    fn width(&self) -> usize {
        self.grid[0].len()
    }

    fn height(&self) -> usize {
        self.grid.len()
    }

    fn count_resources(&self) -> (u32, u32) {
        let (mut wooded, mut lumber) = (0, 0);

        for y in 0..self.height() {
            for x in 0..self.width() {
                if self.grid[y][x] == Acre::Trees {
                    wooded += 1;
                }
                if self.grid[y][x] == Acre::Lumberyard {
                    lumber += 1;
                }
            }
        }

        (wooded, lumber)
    }
}

pub fn answer1() {
    let s = std::fs::read_to_string("input/input18.txt").expect("cannot read file");
    let mut world = World::from_str(&s).expect("failed to parse world");

    (0..10).for_each(|_| world.step());
    let resources = world.count_resources();

    println!(
        "Settlers of The North Pole (1/2): {}",
        resources.0 * resources.1
    );
}

pub fn answer2() {
    let s = std::fs::read_to_string("input/input18.txt").expect("cannot read file");
    let mut world = World::from_str(&s).expect("failed to parse world");

    let mut seen = HashMap::new();

    let mut i = 0;
    let (mut current_i, period) = loop {
        match seen.entry(world.clone()) {
            Entry::Occupied(o) => {
                let prev_i = *o.get();
                let period = i - prev_i;
                break (i, period);
            }
            Entry::Vacant(v) => {
                v.insert(i);
            }
        }

        world.step();
        i += 1;
    };

    let target_i = 1000000000u64;
    let target_modulo = target_i % period as u64;

    let mut current_mod = current_i % period as u64;
    while current_mod != target_modulo {
        world.step();
        current_i += 1;
        current_mod = current_i % period;
    }

    //println!("period: {}, target_modulo: {}, final_i: {}", period, target_modulo, current_i);

    let resources = world.count_resources();
    println!(
        "Settlers of The North Pole (2/2): {}",
        resources.0 * resources.1
    );
}

#[test]
fn test_parse() {
    let world = World::from_str(
        r#"
.#.#...|#.
.....#|##|
.|..|...#.
..|#.....#
#.#|||#|#|
...#.||...
.|....|...
||...#|.#|
|.||||..|.
...#.|..|."#,
    )
    .unwrap();
    println!("{}", world);

    assert_eq!(world.grid.len(), 10);
    assert_eq!(world.grid[0].len(), 10);
}

#[test]
fn test_step() {
    let mut world = World::from_str(
        r#"
.#.#...|#.
.....#|##|
.|..|...#.
..|#.....#
#.#|||#|#|
...#.||...
.|....|...
||...#|.#|
|.||||..|.
...#.|..|."#,
    )
    .unwrap();
    world.step();
    println!("test: {}", world);

    let expected = World::from_str(
        r#"
.......##.
......|###
.|..|...#.
..|#||...#
..##||.|#|
...#||||..
||...|||..
|||||.||.|
||||||||||
....||..|."#,
    )
    .unwrap();

    assert_eq!(expected, world);
}

#[test]
fn test_count_resources() {
    let mut world = World::from_str(
        r#"
.#.#...|#.
.....#|##|
.|..|...#.
..|#.....#
#.#|||#|#|
...#.||...
.|....|...
||...#|.#|
|.||||..|.
...#.|..|."#,
    )
    .unwrap();
    (0..10).for_each(|_| world.step());

    assert_eq!((37, 31), world.count_resources());
}
