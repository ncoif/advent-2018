use std::io::{BufRead, BufReader};
use std::fs::File;
use std::str::Chars;

fn main() {
    let file = File::open("input.txt").expect("cannot open file");
    let file = BufReader::new(file);

    let mut result: i64 = 0;
    for line in file.lines().filter_map(|result| result.ok()) {
        let mut chars = line.chars();
        let sign = chars.next().expect("sign missing");
        result =
            if '+' == sign {
                result + chars_to_int(chars) as i64 
            } else {
                result - chars_to_int(chars) as i64
            };
    }

    println!("Result: {}", result);
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
