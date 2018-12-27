use itertools::Itertools;
use std::fmt;
use std::str::FromStr;

#[derive(Debug)]
enum Acre {
    Open,
    Trees,
    Lumberyard,
}

#[derive(Debug)]
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

pub fn answer1() {
    println!("Settlers of The North Pole (1/2): {}", 0);
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
