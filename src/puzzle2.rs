use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::Chars;

pub fn run() {
    let inputs = read_file();

    let mut double_frequencies_box = 0;
    let mut triple_frequencies_box = 0;
    for input in inputs {
        if input.values().find(|&v| *v == 2).is_some() {
            double_frequencies_box += 1;
        }
        if input.values().find(|&v| *v == 3).is_some() {
            triple_frequencies_box += 1;
        }
    }

    println!(
        "Checksum: {}",
        double_frequencies_box * triple_frequencies_box
    );
}

fn read_file() -> Vec<HashMap<char, u32>> {
    let filename = "input/input2.txt";
    let file = File::open(filename).expect("cannot open file");
    let reader = BufReader::new(file);

    reader
        .lines()
        .filter_map(|result| result.ok())
        .map(|s| frequencies(s.chars()))
        .collect()
}

fn frequencies(s: Chars) -> HashMap<char, u32> {
    let mut frequencies = HashMap::new();

    for c in s {
        let count = frequencies.entry(c).or_insert(0);
        *count += 1;
    }

    frequencies
}

#[cfg(test)]
mod tests {
    use puzzle2;

    #[test]
    fn frequencies_no_duplicates() {
        let frequencies = puzzle2::frequencies("abcdef".chars());
        assert_eq!(frequencies.get(&'a'), Some(&1));
        assert_eq!(frequencies.get(&'b'), Some(&1));
        assert_eq!(frequencies.get(&'g'), None);
    }

    #[test]
    fn frequencies_with_duplicates() {
        let frequencies = puzzle2::frequencies("bababc".chars());
        assert_eq!(frequencies.get(&'a'), Some(&2));
        assert_eq!(frequencies.get(&'b'), Some(&3));
        assert_eq!(frequencies.get(&'c'), Some(&1));
    }
}
