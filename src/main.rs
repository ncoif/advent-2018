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
mod puzzle17_water;
mod puzzle18_trees;
mod puzzle19_opcode_jumps;
mod puzzle20_rooms;
mod puzzle21_opcode_halt;
mod puzzle22_cave;
mod puzzle23_nanobots;
mod puzzle24_army;

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

        171 => puzzle17_water::answer1(),
        172 => puzzle17_water::answer2(),

        181 => puzzle18_trees::answer1(),
        182 => puzzle18_trees::answer2(),

        191 => puzzle19_opcode_jumps::answer1(),
        192 => puzzle19_opcode_jumps::answer2(),

        201 => puzzle20_rooms::answer1(),
        202 => puzzle20_rooms::answer2(),

        211 => puzzle21_opcode_halt::answer1(),
        212 => puzzle21_opcode_halt::answer2(),

        221 => puzzle22_cave::answer1(),
        222 => puzzle22_cave::answer2(),

        231 => puzzle23_nanobots::answer1(),

        241 => puzzle24_army::answer1(),

        _ => panic!("Invalid problem day: {}, {}", day, problem),
    }
}
