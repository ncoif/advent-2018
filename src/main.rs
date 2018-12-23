use std::env;

mod puzzle01_frequency;
mod puzzle02_checksum;
mod puzzle03_fabric;
mod puzzle04_nightshift;
mod puzzle05_polymer;
mod puzzle06_coords;
mod puzzle07_dependencies;
mod puzzle08_tree;
mod puzzle09_marble;
mod puzzle10_stars;
mod puzzle11_fuel_cells;
mod puzzle12_game_of_life;
mod puzzle13_minecarts;
mod puzzle14_chocolate_receipt;
mod puzzle15_combat;
mod puzzle16_opcode;

fn main() {
    let day: u32 = env::args()
        .nth(1)
        .expect("missing day")
        .parse::<u32>()
        .expect("invalid day");

    let problem: u32 = env::args()
        .nth(2)
        .expect("missing problem")
        .parse::<u32>()
        .expect("invalid problem");

    match day * 10 + problem {
        11 => puzzle01_frequency::answer1(),
        12 => puzzle01_frequency::answer2(),

        21 => puzzle02_checksum::answer1(),
        22 => puzzle02_checksum::answer2(),

        31 => puzzle03_fabric::answer1(),
        32 => puzzle03_fabric::answer2(),

        41 => puzzle04_nightshift::answer1(),
        42 => puzzle04_nightshift::answer2(),

        51 => puzzle05_polymer::answer1(),
        52 => puzzle05_polymer::answer2(),

        61 => puzzle06_coords::answer1(),
        62 => puzzle06_coords::answer2(),

        71 => puzzle07_dependencies::answer1(),
        72 => puzzle07_dependencies::answer2(),

        81 => puzzle08_tree::answer1(),
        82 => puzzle08_tree::answer2(),

        91 => puzzle09_marble::answer1(),
        92 => puzzle09_marble::answer2(),

        101 => puzzle10_stars::answer1(),

        111 => puzzle11_fuel_cells::answer1(),
        112 => puzzle11_fuel_cells::answer2(),

        121 => puzzle12_game_of_life::answer1(),
        122 => puzzle12_game_of_life::answer2(),

        131 => puzzle13_minecarts::answer1(),
        132 => puzzle13_minecarts::answer2(),

        141 => puzzle14_chocolate_receipt::answer1(),
        142 => puzzle14_chocolate_receipt::answer2(),

        151 => puzzle15_combat::answer1(),
        152 => puzzle15_combat::answer2(),

        161 => puzzle16_opcode::answer1(),
        162 => puzzle16_opcode::answer2(),

        _ => panic!("Invalid problem day: {}, {}", day, problem),
    }
}
