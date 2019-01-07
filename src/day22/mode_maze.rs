use crate::common::error::AocError;
use crate::common::response::AocResponse;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RegionType {
    ROCKY,
    WET,
    NARROW,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Tool {
    NEITHER,
    CLIMBING,
    TORCH,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Region {
    geological_index: usize,
    erosion_level: usize,
    region_type: RegionType,
}

impl Region {
    fn new(geological_index: usize, depth: usize) -> Region {
        let erosion_level = (geological_index + depth) % 20183;
        let region_type = match erosion_level % 3 {
            0 => RegionType::ROCKY,
            1 => RegionType::WET,
            2 => RegionType::NARROW,
            _ => unreachable!(),
        };

        Region {
            geological_index,
            erosion_level,
            region_type,
        }
    }

    fn risk_level(&self) -> usize {
        match self.region_type {
            RegionType::ROCKY => 0,
            RegionType::WET => 1,
            RegionType::NARROW => 2,
        }
    }

    fn can_equip(&self, tool: Tool) -> bool {
        match (self.region_type, tool) {
            (RegionType::ROCKY, Tool::CLIMBING) => true,
            (RegionType::ROCKY, Tool::TORCH) => true,
            (RegionType::WET, Tool::CLIMBING) => true,
            (RegionType::WET, Tool::NEITHER) => true,
            (RegionType::NARROW, Tool::TORCH) => true,
            (RegionType::NARROW, Tool::NEITHER) => true,
            _ => false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Coord {
    x: usize,
    y: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Node {
    x: usize,
    y: usize,
    t: Tool,
}

struct Cave {
    target: Coord,
    bound: Coord,
    regions: Vec<Option<Region>>,
}

impl Cave {
    fn new(target: &Coord, depth: usize) -> Cave {
        let index_bound = Coord {
            x: (target.x + target.y) * 2,
            y: (target.x + target.y) * 2,
        };
        let regions = Cave::compute_geo_indices(&index_bound, target, depth);

        Cave {
            target: target.clone(),
            bound: index_bound,
            regions,
        }
    }

    fn compute_geo_indices(
        index_bound: &Coord,
        target: &Coord,
        depth: usize,
    ) -> Vec<Option<Region>> {
        let mut regions = vec![None; index_bound.x * index_bound.y];

        let access = |x, y| (x + index_bound.x * y) as usize;

        regions[access(0, 0)] = Some(Region::new(0, depth));
        regions[access(target.x, target.y)] = Some(Region::new(0, depth));

        for x in 0..index_bound.x {
            regions[access(x, 0)] = Some(Region::new(x * 16807, depth));
        }
        for y in 0..index_bound.y {
            regions[access(0, y)] = Some(Region::new(y * 48271, depth));
        }
        for y in 1..index_bound.y {
            for x in 1..index_bound.x {
                if x == target.x && y == target.y {
                    continue;
                }

                let left = regions[access(x, y - 1)].unwrap();
                let above = regions[access(x - 1, y)].unwrap();
                let geo_index = left.erosion_level * above.erosion_level;
                regions[access(x, y)] = Some(Region::new(geo_index, depth));
            }
        }

        regions
    }

    fn access(&self, x: usize, y: usize) -> usize {
        x + self.bound.x * y
    }

    fn risk_level(&self) -> usize {
        let mut risk = 0;
        for y in 0..=self.target.y {
            for x in 0..=self.target.x {
                risk += self.regions[self.access(x, y)].unwrap().risk_level();
            }
        }

        risk
    }

    fn around(&self, n: &Node) -> Vec<(Node, u64)> {
        let mut arounds = vec![];

        // add all the neighbours
        for (dx, dy) in [(-1isize, 0isize), (1, 0), (0, -1), (0, 1), (0, 0)].iter() {
            let new_x = n.x as isize + dx;
            let new_y = n.y as isize + dy;

            if new_x < 0 || new_y < 0 {
                continue;
            }

            if new_x > (self.bound.x as isize - 1) || new_y > (self.bound.y as isize - 1) {
                continue;
            }

            if !self.regions[self.access(new_x as usize, new_y as usize)]
                .expect("bound too small")
                .can_equip(n.t)
            {
                continue;
            }

            arounds.push((
                Node {
                    x: new_x as usize,
                    y: new_y as usize,
                    t: n.t,
                },
                1,
            ));
        }

        // also consider changing gear
        for t in [Tool::NEITHER, Tool::CLIMBING, Tool::TORCH].iter() {
            if *t == n.t {
                continue;
            }

            if !self.regions[self.access(n.x, n.y)]
                .expect("bound too small")
                .can_equip(*t)
            {
                continue;
            }

            arounds.push((
                Node {
                    x: n.x,
                    y: n.y,
                    t: *t,
                },
                7,
            ));
        }

        arounds
    }

    // cannot collect the iterator at any point here, as it will be collected by dijkstra_all
    // or else "temporary value moved while borrowing" error
    fn successors(&self, n: &Node) -> impl Iterator<Item = (Node, u64)> {
        let successors: Vec<_> = self.around(n);
        //println!("successors of {:?} are {:?}", n, successors);

        successors.into_iter()
    }

    fn shortest_path(&self) -> u64 {
        let start = Node {
            x: 0,
            y: 0,
            t: Tool::TORCH,
        };
        let target = Node {
            x: self.target.x,
            y: self.target.y,
            t: Tool::TORCH,
        };

        let shortest_path = pathfinding::directed::dijkstra::dijkstra(
            &start,
            |n| self.successors(n),
            |n| *n == target,
        );

        shortest_path.expect("no path found").1
    }
}

pub fn answer1() -> Result<AocResponse<usize>, AocError> {
    let cave = Cave::new(&Coord { x: 6, y: 770 }, 4845);
    Ok(AocResponse::new(22, 1, "Mode Maze", cave.risk_level()))
}

pub fn answer2() -> Result<AocResponse<u64>, AocError> {
    let cave = Cave::new(&Coord { x: 6, y: 770 }, 4845);
    Ok(AocResponse::new(22, 2, "Mode Maze", cave.shortest_path()))
}

#[test]
fn test_answer1() {
    let cave = Cave::new(&Coord { x: 10, y: 10 }, 510);
    assert_eq!(114, cave.risk_level());
}

#[test]
fn test_answer2() {
    let cave = Cave::new(&Coord { x: 10, y: 10 }, 510);
    assert_eq!(45, cave.shortest_path());
}
