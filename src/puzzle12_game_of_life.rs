use std::collections::HashSet;

#[derive(Debug)]
struct Plantation {
    state: String,
    offset: isize,
}

impl Plantation {
    fn step(&self, rules: &HashSet<String>) -> Plantation {
        let mut next_state = "".to_string();
        for i in 0..(self.state.len() - 4) {
            let current = &self.state[i..][..5];
            next_state.push(if rules.contains(current) { '#' } else { '.' });
        }

        // find the first and last #
        let first = next_state.bytes().position(|c| c == b'#').unwrap();
        let last = next_state.bytes().rposition(|c| c == b'#').unwrap();

        let mut formated_state = String::from("....");
        formated_state.push_str(&next_state[first..][..(last - first + 1)]);
        formated_state.push_str("....");

        let formated_offset = self.offset + 2 - first as isize;

        Plantation {
            state: formated_state,
            offset: formated_offset,
        }
    }

    fn sum(&self) -> isize {
        self.state
            .chars()
            .enumerate()
            .filter(|(_i, c)| *c == '#')
            .map(|(i, _c)| i as isize - self.offset)
            .sum()
    }
}

fn generations(s: &str) -> isize {
    let mut lines = s.split("\n");
    let mut state = lines.next().unwrap().split(" ").nth(2).unwrap().to_string();

    // add 4 . on each side
    state.insert_str(0, "....");
    state.push_str("....");

    lines.next(); // skip blank line
    let rules: HashSet<String> = lines
        .filter(|l| l.ends_with(" => #")) // onyl rule that produce alive elements
        .map(|s| s.split(" ").next().unwrap().parse::<String>().unwrap())
        .collect();

    let mut p = Plantation { state, offset: 4 }; // the center is at offset 4 because we added 4 characters
    (0..20).for_each(|_| p = p.step(&rules));

    p.sum()
}

pub fn answer1() {
    let s = std::fs::read_to_string("input/input12.txt").expect("cannot read file");
    println!(
        "Day 12: Subterranean Sustainability (1/2): {}",
        generations(&s)
    );
}

pub fn answer2() {
    let s = std::fs::read_to_string("input/input12.txt").expect("cannot read file");

    let mut lines = s.split("\n");
    let mut state = lines.next().unwrap().split(" ").nth(2).unwrap().to_string();

    // add 4 . on each side
    state.insert_str(0, "....");
    state.push_str("....");

    lines.next(); // skip blank line
    let rules: HashSet<String> = lines
        .filter(|l| l.ends_with(" => #")) // onyl rule that produce alive elements
        .map(|s| s.split(" ").next().unwrap().parse::<String>().unwrap())
        .collect();

    let mut current = Plantation { state, offset: 4 }; // the center is at offset 4 because we added 4 characters
    let mut i = 0;

    let result = loop {
        let next = current.step(&rules);
        // if the state is indentical (but the offset might change), then we are stable and we break
        if next.state == current.state {
            let diff = next.sum() - current.sum();
            // rust is awsome, you can return a value in a break!!!
            break (50_000_000_000 - i) * diff + current.sum();
        }
        i += 1;
        current = next;
    };

    println!("Day 12: Subterranean Sustainability (2/2): {}", result);
}

#[test]
fn test_step() {
    let p = Plantation {
        state: "....#..#.#..##......###...###....".to_string(),
        offset: 4,
    };
    let mut rules = HashSet::new();
    rules.insert("...##".to_string());
    rules.insert("..#..".to_string());
    rules.insert(".#...".to_string());
    rules.insert(".#.#.".to_string());
    rules.insert(".#.##".to_string());
    rules.insert(".##..".to_string());
    rules.insert(".####".to_string());
    rules.insert("#.#.#".to_string());
    rules.insert("#.###".to_string());
    rules.insert("##.#.".to_string());
    rules.insert("##.##".to_string());
    rules.insert("###..".to_string());
    rules.insert("###.#".to_string());
    rules.insert("####.".to_string());

    let new_p = p.step(&rules);

    assert_eq!(new_p.state, "....#...#....#.....#..#..#..#....".to_string());
    assert_eq!(new_p.offset, 4);
    assert_eq!(new_p.sum(), 91);
}

#[test]
fn test_generation() {
    assert_eq!(
        generations(
            r#"initial state: #..#.#..##......###...###

...## => #
..#.. => #
.#... => #
.#.#. => #
.#.## => #
.##.. => #
.#### => #
#.#.# => #
#.### => #
##.#. => #
##.## => #
###.. => #
###.# => #
####. => #
"#
        ),
        325
    );
}
