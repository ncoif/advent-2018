use std::io::{BufRead, BufReader};
use std::fs::File;
use std::str::Chars;
use std::collections::HashSet;

fn main() {
    let mut current_frequency: i64 = 0;
    let mut seen = HashSet::new();
    let mut first_seen = None;
    let mut final_frequency = None;

    while first_seen.is_none() {
        let file = File::open("input.txt").expect("cannot open file");
        let file = BufReader::new(file);

        for line in file.lines().filter_map(|result| result.ok()) {
            let mut chars = line.chars();
            let sign = chars.next().expect("sign missing");
            current_frequency =
                if '+' == sign {
                    current_frequency + chars_to_int(chars) as i64
                } else {
                    current_frequency - chars_to_int(chars) as i64
                };
            if first_seen.is_none() && seen.contains(&current_frequency) {
                first_seen = Some(current_frequency);
            }
            seen.insert(current_frequency);
        }

        if final_frequency.is_none() {
            final_frequency = Some(current_frequency);
        }
    }

    println!("First seen frequency: {:?}", first_seen);
    println!("Final frequency: {:?}", final_frequency);
}

fn chars_to_int(arr: Chars) -> u64 {
    const RADIX: u32 = 10;
    let mut pos = 0;
    let mut result = 0;
    for c_char in arr.rev() {
        let c_int = c_char.to_digit(RADIX).unwrap();
        result = result + (RADIX.pow(pos) * c_int) as u64;
        pos += 1;
    }
    return result;
}

#[cfg(test)]
mod tests {
    use chars_to_int;

    #[test]
    fn single_digit_convert() {
        assert_eq!(chars_to_int("1".chars()), 1);
        assert_eq!(chars_to_int("9".chars()), 9);
    }

    #[test]
    fn multiple_digits_convert_small_number() {
        assert_eq!(chars_to_int("123".chars()), 123);
        assert_eq!(chars_to_int("987".chars()), 987);
    }

    #[test]
    fn multiple_digits_convert_large_number() {
        assert_eq!(chars_to_int("12345".chars()), 12345);
        assert_eq!(chars_to_int("98765".chars()), 98765);
    }
}
