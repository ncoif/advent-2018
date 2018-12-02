use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::Chars;

pub fn run() {
    let inputs = read_file();

    {
        let frequencies: Vec<HashMap<char, u32>> =
            inputs.iter().map(|s| frequencies(s.chars())).collect();

        let (double, triple) = frequencies
            .iter()
            .fold((0, 0), |(mut double, mut triple), freq| {
                if freq.values().find(|&v| *v == 2).is_some() {
                    double += 1;
                }
                if freq.values().find(|&v| *v == 3).is_some() {
                    triple += 1;
                }
                (double, triple)
            });

        println!("Checksum: {}", double * triple);
    }

    {
        let mut result = None;

        'outer: for x in &inputs {
            for y in &inputs {
                let common = common_letters(&x, &y);
                if common.len() == x.len() - 1 {
                    result = Some(common);
                    break 'outer;
                }
            }
        }

        println!("Common letter between correct boxes: {:?}", result);
    }
}

fn read_file() -> Vec<String> {
    let filename = "input/input2.txt";
    let file = File::open(filename).expect("cannot open file");
    let reader = BufReader::new(file);

    reader.lines().filter_map(|result| result.ok()).collect()
}

fn frequencies(s: Chars) -> HashMap<char, u32> {
    let mut frequencies = HashMap::new();

    for c in s {
        let count = frequencies.entry(c).or_insert(0);
        *count += 1;
    }

    frequencies
}

fn common_letters(s1: &String, s2: &String) -> String {
    s1.chars()
        .zip(s2.chars())
        .filter(|(c1, c2)| c1 == c2)
        .map(|(c1, _c2)| c1)
        .collect()
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

    #[test]
    fn common_letters() {
        assert_eq!(
            puzzle2::common_letters(&String::from("abc"), &String::from("abd")),
            String::from("ab")
        );
        assert_eq!(
            puzzle2::common_letters(&String::from("abc"), &String::from("bbc")),
            String::from("bc")
        );
    }
}
