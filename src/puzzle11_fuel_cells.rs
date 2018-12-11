fn cell_power(x: usize, y: usize, serial: i32) -> i32 {
    let rack_id = x as i32 + 10;
    let power_start = (rack_id * y as i32 + serial) * rack_id;
    let hundredth = (power_start / 100) % 10;
    (hundredth - 5) as i32
}

pub fn answer1() {
    let grid_size = 300;
    let serial = 7689;

    let access = |x, y| ((x - 1) + grid_size * (y - 1)) as usize;
    let mut grid = vec![0; grid_size * grid_size];
    for x in 1..=grid_size {
        for y in 1..=grid_size {
            grid[access(x, y)] = cell_power(x, y, serial);
        }
    }

    // for all 3x3 grid centers
    let mut max_sum = 0;
    let mut result_x = 1;
    let mut result_y = 1;
    for x in 2..=(grid_size - 1) {
        for y in 2..=(grid_size - 1) {
            let mut sum = 0;
            sum += grid[access(x - 1, y - 1)];
            sum += grid[access(x, y - 1)];
            sum += grid[access(x + 1, y - 1)];
            sum += grid[access(x - 1, y)];
            sum += grid[access(x, y)];
            sum += grid[access(x + 1, y)];
            sum += grid[access(x - 1, y + 1)];
            sum += grid[access(x, y + 1)];
            sum += grid[access(x + 1, y + 1)];

            if sum > max_sum {
                max_sum = sum;
                result_x = x - 1;
                result_y = y - 1;
            }
        }
    }

    println!("answer1: {}x{}: max {}", result_x, result_y, max_sum);
}

pub fn answer2() {
    let grid_size = 300;
    let serial = 7689;

    let access = |x, y| ((x - 1) + grid_size * (y - 1)) as usize;
    let mut grid = vec![0; (grid_size * grid_size) as usize];
    for x in 1..=grid_size {
        for y in 1..=grid_size {
            grid[access(x, y)] = cell_power(x, y, serial);
        }
    }

    // for all grid sizes
    let mut max_sum = 0;
    let mut result_x = 1;
    let mut result_y = 1;
    let mut max_size = 0;

    for s in 3..=300 {
        let edge = (s / 2) + 1;
        for x in (edge + 1)..=(grid_size - edge + 1) {
            for y in (edge + 1)..=(grid_size - edge + 1) {
                let mut sum = 0;

                for offset_x in 0..s {
                    for offset_y in 0..s {
                        let temp_x = (x as i32 - edge as i32 + offset_x as i32) as usize;
                        let temp_y = (y as i32 - edge as i32 + offset_y as i32) as usize;
                        sum += grid[access(temp_x, temp_y)];
                    }
                }

                if sum > max_sum {
                    max_sum = sum;
                    result_x = x - edge;
                    result_y = y - edge;
                    max_size = s;
                }
            }
        }
    }

    println!(
        "answer2: {}x{} (size {}): max {}",
        result_x, result_y, max_size, max_sum
    );
}

#[test]
fn cell_power_test() {
    assert_eq!(cell_power(3, 5, 8), 4);
    assert_eq!(cell_power(122, 79, 57), -5);
    assert_eq!(cell_power(217, 196, 39), 0);
    assert_eq!(cell_power(101, 153, 71), 4);
}
