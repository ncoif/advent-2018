use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::Chars;

pub fn run() {
    let inputs = read_file();

    {
        let frequencies: Vec<HashMap<char, u32>> =
            inputs.iter().map(|s| frequencies(s.chars())).collect();

        let mut double_frequencies_box = 0;
        let mut triple_frequencies_box = 0;
        for input in &frequencies {
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

    {
        let box_size = inputs[0].chars().count();
        let mut result = None;

        for char_index in 0..box_size {
            // to store the frequency of the box when a given letter is removed
            let mut box_frequencies = HashMap::new();
            for input in &inputs {
                let box_with_removed_char = remove_char_at(input, char_index as u32);
                let count = box_frequencies.entry(box_with_removed_char).or_insert(0);
                *count += 1;
            }

            //if we get a frequency of 2 exactly, then it's our winner
            //else it's the wrong letter that we removed, and we try with the next one
            for (key, value) in box_frequencies {
                if value == 2 {
                    result = Some(key);
                }
            }

            if result.is_some() {
                break;
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

fn remove_char_at(s: &String, index: u32) -> String {
    let mut result = String::from("");
    for (i, c) in s.char_indices() {
        if i as u32 != index {
            result.push(c);
        }
    }
    result
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
    fn string_remove_at() {
        assert_eq!(
            puzzle2::remove_char_at(&String::from("abcdef"), 2),
            String::from("abdef")
        );
        assert_eq!(
            puzzle2::remove_char_at(&String::from("abcdef"), 0),
            String::from("bcdef")
        );
        assert_eq!(
            puzzle2::remove_char_at(&String::from("abcdef"), 5),
            String::from("abcde")
        );
    }
}
