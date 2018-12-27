#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RegionType {
    ROCKY,
    WET,
    NARROW,
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
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Coord {
    x: usize,
    y: usize,
}

struct Cave {
    _depth: usize,
    target: Coord,
    _bound: Coord,
    regions: Vec<Vec<Option<Region>>>,
}

impl Cave {
    fn new(target: &Coord, depth: usize) -> Cave {
        let index_bound = Coord {
            x: target.x + target.y,
            y: target.x + target.y,
        };
        let regions = Cave::compute_geo_indices(&index_bound, target, depth);

        Cave {
            _depth: depth,
            target: target.clone(),
            _bound: index_bound,
            regions: regions,
        }
    }

    fn compute_geo_indices(
        index_bound: &Coord,
        target: &Coord,
        depth: usize,
    ) -> Vec<Vec<Option<Region>>> {
        let mut regions = vec![vec![None; index_bound.x]; index_bound.y];

        regions[0][0] = Some(Region::new(0, depth));
        regions[target.y][target.x] = Some(Region::new(0, depth));

        for x in 0..index_bound.x {
            regions[0][x] = Some(Region::new(x * 16807, depth));
        }
        for y in 0..index_bound.y {
            regions[y][0] = Some(Region::new(y * 48271, depth));
        }
        for y in 1..index_bound.y {
            for x in 1..index_bound.x {
                if x == target.x && y == target.y {
                    continue;
                }

                let left = regions[y - 1][x].unwrap();
                let above = regions[y][x - 1].unwrap();
                let geo_index = left.erosion_level * above.erosion_level;
                regions[y][x] = Some(Region::new(geo_index, depth));
            }
        }

        regions
    }

    fn risk_level(&self) -> usize {
        let mut risk = 0;
        for y in 0..=self.target.y {
            for x in 0..=self.target.x {
                risk += self.regions[y][x].unwrap().risk_level();
            }
        }

        risk
    }
}

pub fn answer1() {
    let cave = Cave::new(&Coord { x: 6, y: 770 }, 4845);
    println!("Mode Maze (1/2): {:?}", cave.risk_level());
}

#[test]
fn test_distance_2() {
    let cave = Cave::new(&Coord { x: 10, y: 10 }, 510);
    assert_eq!(114, cave.risk_level());
}
