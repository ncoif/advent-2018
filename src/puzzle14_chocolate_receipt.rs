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

    fn result(&self, n: usize) -> String {
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

    s.result(n)
}

pub fn answer1() {
    println!("answer1: {:?}", simulate1(580741));
}

pub fn answer2() {}

#[test]
fn test() {
    assert_eq!(simulate1(9), String::from("5158916779"));
    assert_eq!(simulate1(5), String::from("0124515891"));
    assert_eq!(simulate1(18), String::from("9251071085"));
    assert_eq!(simulate1(2018), String::from("5941429882"));
}
