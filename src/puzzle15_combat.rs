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

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
struct Node(usize, usize); // (x,y)

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
                        units.push(Unit::new(x, y, false, 200));
                    }
                    'E' => {
                        wall.push(false);
                        units.push(Unit::new(x, y, true, 200));
                    }
                    _ => panic!("Unexpected token"),
                }
            }
            walls.push(wall);
        }

        // sort units by turn order
        units.sort_by_key(|u| (u.y, u.x));

        State { walls, units }
    }

    fn around(n: Node) -> impl Iterator<Item = Node> {
        [(-1isize, 0isize), (1, 0), (0, -1), (0, 1)]
            .iter()
            .map(move |(dx, dy)| (n.0 as isize + dx, n.1 as isize + dy))
            .map(|(x, y)| Node(x as usize, y as usize))
    }

    fn is_free(&self, n: &Node) -> bool {
        !self.walls[n.1][n.0]
            && self
                .units
                .iter()
                .find(|u| u.x == n.0 && u.y == n.1)
                .is_none()
    }

    // use dijkstra_all to find the best node
    fn find_target(&self, start: &Node) -> Option<Node> {
        let all_reachables = pathfinding::directed::dijkstra::dijkstra_all(start, |n| {
            // cannot collect the iterator at any point here, as it will be collected by dijkstra_all
            // or else "temporary value moved while borrowing" error
            Self::around(n.clone())
                .filter(|n| self.is_free(n))
                .map(|n| n.clone())
                .map(|n| (n, 1)) // cost of 1
        });
        println!("all_reachables: {:?}", all_reachables);
        None
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

#[test]
fn test_find_target() {
    let state = State::parse(
        r#"
#######
#E..G.#
#...#.#
#.G.#G#
#######"#,
    );
    println!("{:?}", state);

    let target = state.find_target(&Node(1, 1));
    assert_eq!(target, Some(Node(4, 1)));
}
