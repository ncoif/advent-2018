fn cell_power(x: usize, y: usize, serial: i32) -> i32 {
    let rack_id = x as i32 + 10;
    let power_start = (rack_id * y as i32 + serial) * rack_id;
    let hundredth = (power_start / 100) % 10;
    (hundredth - 5) as i32
}

pub fn answer1() {
    let grid_size = 200;
    let serial = 8;

    let access = |x, y| ((x - 1) + grid_size * (y - 1)) as usize;
    let mut grid = vec![0; grid_size * grid_size];
    for x in 1..=grid_size {
        for y in 1..=grid_size {
            grid[access(x, y)] = cell_power(x, y, serial);
        }
    }

    println!("{}", grid[access(3, 5)]);
}

#[test]
fn cell_power_test() {
    assert_eq!(cell_power(3, 5, 8), 4);
    assert_eq!(cell_power(122, 79, 57), -5);
    assert_eq!(cell_power(217, 196, 39), 0);
    assert_eq!(cell_power(101, 153, 71), 4);
}
