#[derive(Debug)]
struct State {
    list: Vec<u8>,
    elf1: usize,
    elf2: usize,
}

impl State {
    fn step(&mut self) {
        let sum = self.list[self.elf1] + self.list[self.elf2];
        if sum / 10 != 0 {
            self.list.push(sum / 10);
        }
        self.list.push(sum % 10);
        self.elf1 = (self.elf1 + 1 + self.list[self.elf1] as usize) % self.list.len();
        self.elf2 = (self.elf2 + 1 + self.list[self.elf2] as usize) % self.list.len();
    }

    fn result1(&self, n: usize) -> String {
        self.list[n..][..10]
            .iter()
            .map(|r| (b'0' + r) as char)
            .collect()
    }
}

fn simulate1(n: usize) -> String {
    let mut s = State {
        list: vec![3, 7],
        elf1: 0,
        elf2: 1,
    };
    (0..n + 10).for_each(|_| s.step());

    s.result1(n)
}

fn simulate2(input: &[u8]) -> usize {
    let mut s = State {
        list: vec![3, 7],
        elf1: 0,
        elf2: 1,
    };

    let mut steps_count = input.len();
    loop {
        let slice = if s.list.len() > 10 {
            // only check the latest 10 elements
            &s.list[(s.list.len() - 10)..]
        } else {
            &*s.list
        };
        if slice.windows(input.len()).any(|s| s == input) {
            break;
        }
        s.step();

        steps_count += 1;
        if steps_count % 1000_000 == 0 {
            println!("Tried {} steps so far", steps_count);
        }
    }

    s.list
        .windows(input.len())
        .position(|s| s == input)
        .unwrap()
}

pub fn answer1() {
    println!("answer1: {:?}", simulate1(580741));
}

pub fn answer2() {
    println!("answer1: {:?}", simulate2(&vec![5, 8, 0, 7, 4, 1]));
}

#[test]
fn test_simulate1() {
    assert_eq!(simulate1(9), String::from("5158916779"));
    assert_eq!(simulate1(5), String::from("0124515891"));
    assert_eq!(simulate1(18), String::from("9251071085"));
    assert_eq!(simulate1(2018), String::from("5941429882"));
}

#[test]
fn test_simulate2() {
    assert_eq!(simulate2(&vec![5, 1, 5, 8, 9]), 9);
    assert_eq!(simulate2(&vec![0, 1, 2, 4, 5]), 5);
    assert_eq!(simulate2(&vec![9, 2, 5, 1, 0]), 18);
    assert_eq!(simulate2(&vec![5, 9, 4, 1, 4]), 2018);
}
