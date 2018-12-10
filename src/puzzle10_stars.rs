use std::fs::File;
use std::io::{BufRead, BufReader};
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
struct Star {
    pos_x: i32,
    pos_y: i32,
    vel_x: i32,
    vel_y: i32,
}

impl FromStr for Star {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let two: Vec<&str> = s
            .trim_start_matches("position=<")
            .trim_end_matches(">")
            .split("> velocity=<")
            .collect();
        let pos: Vec<&str> = two[0].split(", ").collect();
        let vel: Vec<&str> = two[1].split(", ").collect();

        let pos_x: i32 = pos[0].trim().parse().unwrap();
        let pos_y: i32 = pos[1].trim().parse().unwrap();
        let vel_x: i32 = vel[0].trim().parse().unwrap();
        let vel_y: i32 = vel[1].trim().parse().unwrap();

        Ok(Star {
            pos_x,
            pos_y,
            vel_x,
            vel_y,
        })
    }
}

impl Star {
    fn next(&self) -> Star {
        let pos_x = self.pos_x + self.vel_x;
        let pos_y = self.pos_y + self.vel_y;
        let vel_x = self.vel_x;
        let vel_y = self.vel_y;
        Star {
            pos_x,
            pos_y,
            vel_x,
            vel_y,
        }
    }
}

struct Field {
    stars: Vec<Star>,
    time: u32,
}

impl Field {
    fn new(stars: Vec<Star>) -> Self {
        Field {
            stars: stars,
            time: 0,
        }
    }

    fn step(self: &mut Self) {
        for s in &mut self.stars {
            *s = s.next();
        }
        self.time += 1
    }

    fn display(&mut self) {
        let min_x = self
            .stars
            .iter()
            .map(|s| s.pos_x)
            .min_by(|x1, x2| x1.cmp(x2))
            .unwrap();
        let max_x = self
            .stars
            .iter()
            .map(|s| s.pos_x)
            .max_by(|x1, x2| x1.cmp(x2))
            .unwrap();
        let min_y = self
            .stars
            .iter()
            .map(|s| s.pos_y)
            .min_by(|y1, y2| y1.cmp(y2))
            .unwrap();
        let max_y = self
            .stars
            .iter()
            .map(|s| s.pos_y)
            .max_by(|y1, y2| y1.cmp(y2))
            .unwrap();

        // only display images of reasonable size
        if (max_x - min_x) < 100 && (max_y - min_y) < 30 {
            println!("{}: {}x{}: {}x{}", self.time, min_x, max_x, min_y, max_y);

            let mut pic = vec![false; ((max_x - min_x + 1) * (max_y - min_y + 1)) as usize];
            let access = |x, y| (x - min_x + (max_x - min_x + 1) * (y - min_y)) as usize;

            for star in &self.stars {
                pic[access(star.pos_x, star.pos_y)] = true;
            }

            for y in min_y..=max_y {
                for x in min_x..=max_x {
                    match pic[access(x, y)] {
                        true => print!("#"),
                        _ => print!("."),
                    }
                }
                println!("");
            }
        }
    }
}

fn read_file() -> Vec<Star> {
    //let filename = "input/input10_debug.txt";
    let filename = "input/input10.txt";
    let file = File::open(filename).expect("cannot open file");
    let reader = BufReader::new(file);

    reader
        .lines()
        .filter_map(|result| result.ok())
        .map(|s| Star::from_str(&s).unwrap())
        .collect()
}

pub fn answer1() {
    let stars = read_file();
    let mut field = Field::new(stars);
    for _i in 0..12000 {
        field.step();
        field.display();
    }

    // time: 10036
    // JJXZHKFP
}
