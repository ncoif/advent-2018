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
    rot: u8, // i.e, how many intersections the cart has visited so far
    crashed: bool,
}

#[repr(u8)] // to be able to transmute it
#[derive(Debug, Copy, Clone)]
enum Dir {
    RIGHT = 0,
    UP,
    LEFT,
    DOWN,
}

impl Cart {
    fn new(x: usize, y: usize, dir: Dir) -> Cart {
        Cart {
            x,
            y,
            dir,
            rot: 0,
            crashed: false,
        }
    }
}

impl World {
    fn parse(s: &str) -> World {
        let mut lines: Vec<Vec<u8>> = s
            .split("\n")
            .filter(|l| l.len() > 0)
            .map(|s| format!("{}", s).as_bytes().to_vec())
            .collect();

        let mut carts = vec![];
        for (y, line) in lines.iter().enumerate() {
            for (x, char) in line.iter().enumerate() {
                let cart = match char {
                    b'>' => Some(Cart::new(x, y, Dir::RIGHT)),
                    b'<' => Some(Cart::new(x, y, Dir::LEFT)),
                    b'^' => Some(Cart::new(x, y, Dir::UP)),
                    b'v' => Some(Cart::new(x, y, Dir::DOWN)),
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

    // return crashed carts
    fn step(&mut self) -> Vec<Cart> {
        self.carts.sort_by_key(|c| (c.y, c.x));

        for i in 0..self.carts.len() {
            // move the cart
            match self.carts[i].dir {
                Dir::UP => self.carts[i].y -= 1,
                Dir::DOWN => self.carts[i].y += 1,
                Dir::LEFT => self.carts[i].x -= 1,
                Dir::RIGHT => self.carts[i].x += 1,
            }

            // is there any chars with the same coords?
            if let Some((idx, _)) = self
                .carts
                .iter()
                .enumerate()
                .find(|(idx, c)| *idx != i && c.x == self.carts[i].x && c.y == self.carts[i].y)
            {
                self.carts[idx].crashed = true;
                self.carts[i].crashed = true;
            }

            // find the next direction for the cart
            let (x, y) = (self.carts[i].x, self.carts[i].y);
            let dir = match (self.carts[i].dir, self.lines[y][x]) {
                (Dir::UP, b'/') => Dir::RIGHT,
                (Dir::RIGHT, b'/') => Dir::UP,
                (Dir::LEFT, b'/') => Dir::DOWN,
                (Dir::DOWN, b'/') => Dir::LEFT,
                (Dir::UP, b'\\') => Dir::LEFT,
                (Dir::LEFT, b'\\') => Dir::UP,
                (Dir::DOWN, b'\\') => Dir::RIGHT,
                (Dir::RIGHT, b'\\') => Dir::DOWN,
                (d, b'+') => {
                    // use rotation to compute next direction
                    let next_d = (5 - self.carts[i].rot + d as u8) % 4;
                    self.carts[i].rot = (self.carts[i].rot + 1) % 3;
                    // convert a u8 into a dir
                    unsafe { std::mem::transmute(next_d) }
                }
                (d, _) => d,
            };
            self.carts[i].dir = dir;
        }

        let mut crashed = vec![];
        for i in (0..self.carts.len()).rev() {
            // .rev() to remove the elements without index out of bound
            if self.carts[i].crashed {
                crashed.insert(0, self.carts.remove(i));
            }
        }

        crashed
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

fn simulate1(s: &str) -> (usize, usize) {
    let mut w = World::parse(s);
    //println!("{}", w);

    let crashed = loop {
        let crashed = w.step();
        //println!("{}", w);
        //println!("crashed: {:?}", crashed);
        if !crashed.is_empty() {
            break crashed;
        }
    };

    (crashed[0].x, crashed[0].y)
}

fn simulate2(s: &str) -> (usize, usize) {
    let mut w = World::parse(s);

    loop {
        w.step();
        if w.carts.len() == 1 {
            break;
        }
    }

    (w.carts[0].x, w.carts[0].y)
}

pub fn answer1() {
    let s = std::fs::read_to_string("input/input13.txt").expect("cannot read file");
    println!("answer1: {:?}", simulate1(&s));
}

pub fn answer2() {
    let s = std::fs::read_to_string("input/input13.txt").expect("cannot read file");
    println!("answer2: {:?}", simulate2(&s));
}

#[test]
fn test() {
    let s = std::fs::read_to_string("input/input13_debug.txt").expect("cannot read file");
    assert_eq!(simulate1(&s), (7, 3));
}
