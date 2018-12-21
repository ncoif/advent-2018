use std::collections::HashSet;
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

    fn unit_at(&self, n: &Node) -> &Unit {
        self.units
            .iter()
            .find(|u| u.x == n.0 && u.y == n.1)
            .unwrap()
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

    // all node from opposite side that are in range of the given one
    fn in_range(&self, n: &Node) -> Vec<Node> {
        let is_elf = self.unit_at(n).elf;
        let mut set = HashSet::new();
        for gob in self.units.iter() {
            if gob.elf == is_elf {
                continue; // skip units in the same side
            }

            set.extend(Self::around(Node(gob.x, gob.y)).filter(|n| self.is_free(n))); // i.e addAll(iterator)
        }

        let mut vec: Vec<_> = set.into_iter().collect();
        // sort units by turn order
        vec.sort_by_key(|n| (n.1, n.0));

        vec
    }

    // all reachable nodes from the given one, ordered by distance, and then per units turn order
    fn reachable(&self, n: &Node) -> Vec<Node> {
        let reachables = pathfinding::directed::dijkstra::dijkstra_all(n, |n| {
            // cannot collect the iterator at any point here, as it will be collected by dijkstra_all
            // or else "temporary value moved while borrowing" error
            Self::around(n.clone())
                .filter(|n| self.is_free(n))
                .map(|n| n.clone())
                .map(|n| (n, 1)) // cost of 1
        });

        let mut reachables_nodes: Vec<_> = reachables.iter().map(|(k, v)| (*k, v.1)).collect();
        reachables_nodes.sort_by_key(|e| (e.1, (e.0).1, (e.0).0));

        reachables_nodes.iter().map(|e| e.0).collect()
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

        // for all enemis, find all in_reach nodes that are also in all_reachables
        //FIXME
        println!("in_range: {:?}", self.in_range(start));
        let mut possible_targets: Vec<_> = self
            .in_range(start)
            .iter()
            .filter_map(|n| all_reachables.get(n))
            .collect();
        println!("possible_targets: {:?}", possible_targets);

        // find the closest one, and then unit turn order
        //possible_targets.sort_by_key(|(n,_c)| (n.1, n.0));
        //let min_distance = possible_targets.iter().min_by_key(|(_n, c)| c);
        //println!("min_distance: {:?}", min_distance);

        // return if let Some((n, _c)) = min_distance {
        //     Some(n.clone())
        // } else {
        //     None
        // }
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
fn test_in_range() {
    let state = State::parse(
        r#"
#######
#E..G.#
#...#.#
#.G.#G#
#######"#,
    );
    println!("{:?}", state);

    let actual = state.in_range(&Node(1, 1));
    let expected = vec![
        Node(3, 1),
        Node(5, 1),
        Node(2, 2),
        Node(5, 2),
        Node(1, 3),
        Node(3, 3),
    ];
    assert_eq!(actual, expected);
}

#[test]
fn test_reacheable() {
    let state = State::parse(
        r#"
#######
#E..G.#
#...#.#
#.G.#G#
#######"#,
    );
    println!("{:?}", state);

    let actual = state.reachable(&Node(1, 1));
    let expected = vec![
        Node(2, 1),
        Node(1, 2),
        Node(3, 1),
        Node(2, 2),
        Node(1, 3),
        Node(3, 2),
        Node(3, 3),
    ];
    assert_eq!(actual, expected);
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
