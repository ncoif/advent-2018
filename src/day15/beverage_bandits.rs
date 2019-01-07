use crate::common::error::AocError;
use crate::common::response::AocResponse;

use std::collections::HashSet;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Unit {
    x: usize,
    y: usize,
    elf: bool,
    hp: u32,
    ap: u32,
}

impl Unit {
    fn new(x: usize, y: usize, elf: bool, hp: u32, ap: u32) -> Unit {
        Unit { x, y, elf, hp, ap }
    }
}

#[derive(PartialEq, Eq)]
struct State {
    walls: Vec<Vec<bool>>,
    units: Vec<Unit>,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
struct Node(usize, usize); // (x,y)

impl State {
    fn parse(s: &str, ap_elf: u32, ap_gob: u32) -> State {
        let lines = s.split('\n');

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
                        units.push(Unit::new(x, y, false, 200, ap_gob));
                    }
                    'E' => {
                        wall.push(false);
                        units.push(Unit::new(x, y, true, 200, ap_elf));
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

    fn unit_at(&self, n: &Node) -> Option<&Unit> {
        self.units.iter().find(|u| u.x == n.0 && u.y == n.1)
    }

    fn unit_at_mut(&mut self, n: &Node) -> &mut Unit {
        self.units
            .iter_mut()
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
        let is_elf = self.unit_at(n).unwrap().elf;
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
    fn reachables(&self, n: &Node) -> Vec<Node> {
        let reachables = pathfinding::directed::dijkstra::dijkstra_all(n, |n| {
            // cannot collect the iterator at any point here, as it will be collected by dijkstra_all
            // or else "temporary value moved while borrowing" error
            Self::around(*n).filter(|n| self.is_free(n)).map(|n| (n, 1)) // cost of 1
        });

        let mut reachables_nodes: Vec<_> = reachables.iter().map(|(k, v)| (*k, v.1)).collect();
        reachables_nodes.sort_by_key(|e| {
            let distance = e.1;
            let node = e.0;
            (distance, node.1, node.0)
        });

        reachables_nodes.iter().map(|e| e.0).collect()
    }

    // use dijkstra_all to find the best node
    fn find_target(&self, start: &Node) -> Option<Node> {
        let in_range = self.in_range(start);
        let reachables = self.reachables(start);

        reachables
            .iter()
            .find(|n| in_range.iter().any(|in_range| in_range == *n))
            .cloned()
    }

    fn path_cost_to(&self, n: &Node, to_n: &Node) -> Option<usize> {
        let shortest_path = pathfinding::directed::dijkstra::dijkstra(
            n,
            |n| {
                // cannot collect the iterator at any point here, as it will be collected by dijkstra_all
                // or else "temporary value moved while borrowing" error
                Self::around(*n).filter(|n| self.is_free(n)).map(|n| (n, 1)) // cost of 1
            },
            |n| *n == *to_n,
        );

        if shortest_path.is_some() {
            Some(shortest_path.unwrap().1)
        } else {
            None
        }
    }

    fn find_move_toward(&self, start: &Node, target: &Node) -> Node {
        // what is the cost of all my neighbourds?
        let moves_and_costs: Vec<_> = Self::around(*start)
            .filter(|n| self.is_free(n))
            .filter_map(|n| self.path_cost_to(&n, target).map(|c| (n, c)))
            .collect();
        let next_move = moves_and_costs
            .iter()
            .min_by_key(|(n, c)| (c, n.1, n.0))
            .unwrap();
        (*next_move).0
    }

    fn step_unit(&mut self, n: &mut Node) {
        let me = self.unit_at(n).unwrap().clone();
        // if no enemy around me
        if !Self::around(*n).any(|n| self.unit_at(&n).map(|u| u.elf != me.elf).unwrap_or(false)) {
            if let Some(chosen) = self.find_target(n) {
                //println!("{:?} moving to {:?}", n, chosen);
                let next_node = self.find_move_toward(n, &chosen);
                let mut unit = self.unit_at_mut(n);
                unit.x = next_node.0;
                unit.y = next_node.1;
                n.0 = next_node.0;
                n.1 = next_node.1;
            }
        }
        // if enemy around me
        if let Some(t) = Self::around(*n)
            .filter_map(|n| self.unit_at(&n).filter(|u| u.elf != me.elf))
            .min_by_key(|p| (p.hp, p.y, p.x))
        {
            //println!("{:?} attacking {:?}", n, t);
            let target = self.unit_at_mut(&Node(t.x, t.y));
            // saturating_sub cap to 0 instead of overflowing
            target.hp = target.hp.saturating_sub(me.ap);
            if target.hp == 0 {
                self.units.retain(|u| u.hp > 0);
            }
        }
        //println!("state: {:?}", self);
    }

    fn step(&mut self) {
        let units_at_start = self.units.clone();
        for unit in units_at_start {
            let mut unit_node = Node(unit.x, unit.y);
            if self.unit_at(&unit_node).is_none() {
                continue; //not a unit, skip it
            }
            self.step_unit(&mut unit_node);
        }
        // sort again the units
        self.units.sort_by_key(|u| (u.y, u.x));
    }

    // return the number of rounds
    fn fight_to_death(&mut self) -> u32 {
        let mut r = 0;
        loop {
            self.step();
            if self.units.iter().all(|u| u.elf) || self.units.iter().all(|u| !u.elf) {
                return r;
            }
            r += 1;
        }
    }

    fn remaining_hp(&self) -> u32 {
        self.units.iter().map(|u| u.hp).sum()
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
            writeln!(f)?;
        }

        Ok(())
    }
}

fn combat1(s: &str) -> (u32, u32) {
    let mut state = State::parse(s, 3, 3);

    let rounds = state.fight_to_death();
    let hps = state.remaining_hp();

    (rounds, hps)
}

fn combat2(s: &str) -> (u32, u32) {
    'outer: for e in 3.. {
        let mut s = State::parse(s, e, 3);
        let elves = s.units.iter().filter(|u| u.elf).count();
        let mut i = 0;
        loop {
            s.step();
            if s.units.iter().filter(|u| u.elf).count() < elves {
                continue 'outer;
            }
            if s.units.len() == elves {
                let remain = s.units.iter().map(|u| u.hp).sum::<u32>();
                return (i, remain);
            }
            i += 1;
        }
    }
    panic!()
}

pub fn answer1() -> Result<AocResponse<u32>, AocError> {
    let s = std::fs::read_to_string("input/input15.txt")?;

    let result = combat1(&s);
    Ok(AocResponse::new(
        15,
        1,
        "Beverage Bandits",
        result.0 * result.1,
    ))
}

pub fn answer2() -> Result<AocResponse<u32>, AocError> {
    let s = std::fs::read_to_string("input/input15.txt").expect("cannot read file");

    let result = combat2(&s);
    Ok(AocResponse::new(
        15,
        2,
        "Beverage Bandits",
        result.0 * result.1,
    ))
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
        3,
        3,
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
        3,
        3,
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
fn test_reacheables() {
    let state = State::parse(
        r#"
#######
#E..G.#
#...#.#
#.G.#G#
#######"#,
        3,
        3,
    );
    println!("{:?}", state);

    let actual = state.reachables(&Node(1, 1));
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
        3,
        3,
    );
    println!("{:?}", state);

    let target = state.find_target(&Node(1, 1));
    assert_eq!(target, Some(Node(3, 1)));
}

#[test]
fn test_target_elf_only() {
    let state = State::parse(
        r#"
#######
#E..E.#
#...#.#
#.E.#E#
#######"#,
        3,
        3,
    );
    println!("{:?}", state);

    let target = state.find_target(&Node(1, 1));
    assert_eq!(target, None);
}

#[test]
fn test_target_no_target() {
    let state = State::parse(
        r#"
#######
#E....#
#...#.#
#...#.#
#######"#,
        3,
        3,
    );
    println!("{:?}", state);

    let target = state.find_target(&Node(1, 1));
    assert_eq!(target, None);
}

#[test]
fn test_step_move_unit() {
    let mut state = State::parse(
        r#"
#########
#G..G..G#
#.......#
#.......#
#G..E..G#
#.......#
#.......#
#G..G..G#
#########"#,
        0,
        0,
    );

    let expected_state1 = State::parse(
        r#"
#########
#.G...G.#
#...G...#
#...E..G#
#.G.....#
#.......#
#G..G..G#
#.......#
#########"#,
        0,
        0,
    );

    state.step();
    assert_eq!(expected_state1, state);

    let expected_state2 = State::parse(
        r#"
#########
#..G.G..#
#...G...#
#.G.E.G.#
#.......#
#G..G..G#
#.......#
#.......#
#########"#,
        0,
        0,
    );

    state.step();
    assert_eq!(expected_state2, state);

    let expected_state3 = State::parse(
        r#"
#########
#.......#
#..GGG..#
#..GEG..#
#G..G...#
#......G#
#.......#
#.......#
#########"#,
        0,
        0,
    );

    state.step();
    assert_eq!(expected_state3, state);
}

#[test]
fn test_hps() {
    let mut state = State::parse(
        r#"
#######
#.G...#
#...EG#
#.#.#G#
#..G#E#
#.....#
#######"#,
        3,
        3,
    );
    println!("Initial: {:?}", state);

    let mut expected_state1 = State::parse(
        r#"
#######
#..G..#
#...EG#
#.#G#G#
#...#E#
#.....#
#######"#,
        3,
        3,
    );
    expected_state1.unit_at_mut(&Node(4, 2)).hp = 197;
    expected_state1.unit_at_mut(&Node(5, 2)).hp = 197;
    expected_state1.unit_at_mut(&Node(5, 3)).hp = 197;
    expected_state1.unit_at_mut(&Node(5, 4)).hp = 197;

    state.step();
    println!("Rouund 1: {:?}", state);
    assert_eq!(expected_state1, state);

    let mut expected_state2 = State::parse(
        r#"
#######
#...G.#
#..GEG#
#.#.#G#
#...#E#
#.....#
#######"#,
        3,
        3,
    );
    expected_state2.unit_at_mut(&Node(4, 2)).hp = 188;
    expected_state2.unit_at_mut(&Node(5, 2)).hp = 194;
    expected_state2.unit_at_mut(&Node(5, 3)).hp = 194;
    expected_state2.unit_at_mut(&Node(5, 4)).hp = 194;

    state.step();
    println!("Round 2: {:?}", state);
    assert_eq!(expected_state2, state);
}

#[test]
fn test_combat_1() {
    let mut state = State::parse(
        r#"
#######
#.G...#
#...EG#
#.#.#G#
#..G#E#
#.....#
#######"#,
        3,
        3,
    );

    assert_eq!(state.fight_to_death(), 46);
    assert_eq!(state.remaining_hp(), 590);
}

#[test]
fn test_combat_2() {
    let mut state = State::parse(
        r#"
#######
#E..EG#
#.#G.E#
#E.##E#
#G..#.#
#..E#.#
#######"#,
        3,
        3,
    );

    assert_eq!(state.fight_to_death(), 46);
    assert_eq!(state.remaining_hp(), 859);
}
