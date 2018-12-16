use std::fmt;

#[derive(Debug)]
struct Unit {
    x: usize,
    y: usize,
    elf: bool,
    hp: u32,
}

impl Unit {
    fn new(x: usize, y: usize, elf: bool, hp: u32) -> Unit {
        Unit { x, y, elf, hp }
    }
}

struct State {
    walls: Vec<Vec<bool>>,
    units: Vec<Unit>,
}

impl State {
    fn parse(s: &str) -> State {
        let lines = s.split("\n");

        let mut units = vec![];
        let mut walls = vec![];
        for (y, line) in lines.filter(|l| *l != "").enumerate() {
            let mut wall = vec![];
            for (x, case) in line.chars().enumerate() {
                match case {
                    '#' => wall.push(true),
                    '.' => wall.push(false),
                    'G' => {
                        wall.push(false);
                        units.push(Unit::new(x, y, false, 300));
                    }
                    'E' => {
                        wall.push(false);
                        units.push(Unit::new(x, y, true, 300));
                    }
                    _ => panic!("Unexpected token"),
                }
            }
            walls.push(wall);
        }

        State { walls, units }
    }
}

impl fmt::Debug for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "State: {} units", &self.units.len())?;
        writeln!(f, "Units: {:?}", &self.units)?;
        for (y, wall) in self.walls.iter().enumerate() {
            for (x, case) in wall.iter().enumerate() {
                let unit = &self.units.iter().find(|u| u.x == x && u.y == y);
                let char = match (unit, case) {
                    (None, true) => '#',
                    (None, false) => '.',
                    (Some(u), _) => {
                        if u.elf {
                            'E'
                        } else {
                            'G'
                        }
                    }
                };
                write!(f, "{}", char)?;
            }
            writeln!(f, "")?;
        }

        Ok(())
    }
}

fn combat1(s: &str) -> u32 {
    let state = State::parse(s);
    println!("{:?}", state);

    0
}

pub fn answer1() {
    let s = std::fs::read_to_string("input/input15.txt").expect("cannot read file");
    println!("answer1: {:?}", combat1(&s));
}

#[test]
fn test_parse() {
    let state = State::parse(
        r#"
#######
#E..G.#
#...#.#
#.G.#G#
#######"#,
    );
    println!("{:?}", state);

    assert_eq!(state.units.len(), 4);
    assert_eq!(state.walls.len(), 5);
    assert_eq!(state.walls[0].len(), 7);
}
