use std::fmt;

#[derive(Debug)]
struct World {
    lines: Vec<Vec<u8>>, // u8 an octet (the smallest we can use to store a state)
    carts: Vec<Cart>,
}

#[derive(Debug)]
struct Cart {
    x: usize,
    y: usize,
    dir: Dir,
    rot: Dir,
    crashed: bool,
}

#[derive(PartialEq, Eq, Debug)]
enum Dir {
    LEFT,
    UP,
    RIGHT,
    DOWN,
}

impl World {
    fn parse(s: &str) -> World {
        let mut lines: Vec<Vec<u8>> = s
            .split("\n")
            .filter(|l| l.len() > 0)
            .map(|s| format!("{}", s).as_bytes().to_vec())
            .collect();

        let mut carts = Vec::new();
        for (y, line) in lines.iter().enumerate() {
            for (x, char) in line.iter().enumerate() {
                let cart = match char {
                    b'>' => Some(Cart {
                        x,
                        y,
                        dir: Dir::RIGHT,
                        rot: Dir::LEFT,
                        crashed: false,
                    }),
                    b'<' => Some(Cart {
                        x,
                        y,
                        dir: Dir::LEFT,
                        rot: Dir::LEFT,
                        crashed: false,
                    }),
                    b'^' => Some(Cart {
                        x,
                        y,
                        dir: Dir::UP,
                        rot: Dir::LEFT,
                        crashed: false,
                    }),
                    b'v' => Some(Cart {
                        x,
                        y,
                        dir: Dir::DOWN,
                        rot: Dir::LEFT,
                        crashed: false,
                    }),
                    _ => None,
                };

                cart.map(|c| carts.push(c));
            }
        }

        // replace the carts characters in the world
        for c in &carts {
            lines[c.y][c.x] = if lines[c.y][c.x] == b'<' || lines[c.y][c.x] == b'>' {
                b'-'
            } else {
                b'|'
            };
        }

        World { lines, carts }
    }
}

impl fmt::Display for World {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "World: {} active carts", &self.carts.len())?;
        writeln!(f, "Carts: {:?}", &self.carts)?;
        for (y, line) in self.lines.iter().enumerate() {
            for (x, c) in line.iter().enumerate() {
                if let Some(c) = &self.carts.iter().find(|c| c.x == x && c.y == y) {
                    let char = match c.dir {
                        Dir::UP => '^',
                        Dir::DOWN => 'v',
                        Dir::LEFT => '<',
                        Dir::RIGHT => '>',
                    };
                    write!(f, "{}", char as char)?;
                } else {
                    write!(f, "{}", *c as char)?;
                }
            }
            writeln!(f, "")?;
        }

        Ok(())
    }
}

fn simulate(s: &str) -> (usize, usize) {
    let w = World::parse(s);
    println!("{}", w);

    (0, 0)
}

pub fn answer1() {
    let s = std::fs::read_to_string("input/input13.txt").expect("cannot read file");
    println!("answer1: {:?}", simulate(&s));
}

#[test]
fn test() {
    let s = std::fs::read_to_string("input/input13_debug.txt").expect("cannot read file");
    assert_eq!(simulate(&s), (7, 3));
}
