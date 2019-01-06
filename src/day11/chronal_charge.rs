use crate::common::error::AocError;
use crate::common::response::AocResponse;

fn cell_power(x: usize, y: usize, serial: i32) -> i32 {
    let rack_id = x as i32 + 10;
    let power_start = (rack_id * y as i32 + serial) * rack_id;
    let hundredth = (power_start / 100) % 10;
    (hundredth - 5) as i32
}

pub fn answer1() -> Result<AocResponse<String>, AocError> {
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

    Ok(AocResponse::new(
        11,
        1,
        "Chronal Charge",
        format!("{},{}", result_x, result_y),
    ))
}

pub fn answer2() -> Result<AocResponse<String>, AocError> {
    let grid_size = 300;
    let serial = 7689;

    // https://en.wikipedia.org/wiki/Viola%E2%80%93Jones_object_detection_framework
    // https://en.wikipedia.org/wiki/Summed-area_table
    // for optimisation, compute an integral image
    let mut integral = vec![vec![0i32; 301]; 301]; // 301 because the first row/colum is 0
    for y in 1..=300 {
        let mut s = 0;
        for x in 1..=300 {
            s += cell_power(x, y, serial);
            integral[y][x] = integral[y - 1][x] + s;
        }
    }

    // for all grid sizes
    let mut max_sum = 0;
    let mut result_x = 1;
    let mut result_y = 1;
    let mut max_size = 0;

    for s in 3..=300 {
        // (x,y) is the corner of the square
        for x in 1..=(grid_size - s) {
            for y in 1..=(grid_size - s) {
                // now the sum is just the smaller squares
                let sum = integral[y + s - 1][x + s - 1]
                    - integral[y + s - 1][x - 1]
                    - integral[y - 1][x + s - 1]
                    + integral[y - 1][x - 1];
                //println!("{}: {}x{} sum: {}", s, x, y, sum);

                if sum > max_sum {
                    max_sum = sum;
                    result_x = x;
                    result_y = y;
                    max_size = s;
                }
            }
        }
    }

    Ok(AocResponse::new(
        11,
        2,
        "Chronal Charge",
        format!("{},{},{}", result_x, result_y, max_size),
    ))
}

#[test]
fn cell_power_test() {
    assert_eq!(cell_power(3, 5, 8), 4);
    assert_eq!(cell_power(122, 79, 57), -5);
    assert_eq!(cell_power(217, 196, 39), 0);
    assert_eq!(cell_power(101, 153, 71), 4);
}
