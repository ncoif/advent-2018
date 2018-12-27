use lazy_static::lazy_static;
use regex::Regex;
use std::str::FromStr;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Opcode {
    Addr,
    Addi,
    Mulr,
    Muli,
    Banr,
    Bani,
    Borr,
    Bori,
    Setr,
    Seti,
    Gtir,
    Gtri,
    Gtrr,
    Eqir,
    Eqri,
    Eqrr,
}

impl FromStr for Opcode {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use self::Opcode::*;
        match s {
            "addr" => Ok(Addr),
            "addi" => Ok(Addi),
            "mulr" => Ok(Mulr),
            "muli" => Ok(Muli),
            "banr" => Ok(Banr),
            "bani" => Ok(Bani),
            "borr" => Ok(Borr),
            "bori" => Ok(Bori),
            "setr" => Ok(Setr),
            "seti" => Ok(Seti),
            "gtir" => Ok(Gtir),
            "gtri" => Ok(Gtri),
            "gtrr" => Ok(Gtrr),
            "eqir" => Ok(Eqir),
            "eqri" => Ok(Eqri),
            "eqrr" => Ok(Eqrr),
            _ => Err(()),
        }
    }
}

impl Opcode {
    fn all() -> &'static [Opcode] {
        &[
            Opcode::Addr,
            Opcode::Addi,
            Opcode::Mulr,
            Opcode::Muli,
            Opcode::Banr,
            Opcode::Bani,
            Opcode::Borr,
            Opcode::Bori,
            Opcode::Setr,
            Opcode::Seti,
            Opcode::Gtir,
            Opcode::Gtri,
            Opcode::Gtrr,
            Opcode::Eqir,
            Opcode::Eqri,
            Opcode::Eqrr,
        ]
    }

    fn apply(&self, i: &[usize], r: &mut [usize]) {
        match self {
            Opcode::Addr => r[i[2]] = r[i[0]] + r[i[1]],
            Opcode::Addi => r[i[2]] = r[i[0]] + i[1],
            Opcode::Mulr => r[i[2]] = r[i[0]] * r[i[1]],
            Opcode::Muli => r[i[2]] = r[i[0]] * i[1],
            Opcode::Banr => r[i[2]] = r[i[0]] & r[i[1]],
            Opcode::Bani => r[i[2]] = r[i[0]] & i[1],
            Opcode::Borr => r[i[2]] = r[i[0]] | r[i[1]],
            Opcode::Bori => r[i[2]] = r[i[0]] | i[1],
            Opcode::Setr => r[i[2]] = r[i[0]],
            Opcode::Seti => r[i[2]] = i[0],
            Opcode::Gtir => r[i[2]] = if i[0] > r[i[1]] { 1 } else { 0 },
            Opcode::Gtri => r[i[2]] = if r[i[0]] > i[1] { 1 } else { 0 },
            Opcode::Gtrr => r[i[2]] = if r[i[0]] > r[i[1]] { 1 } else { 0 },
            Opcode::Eqir => r[i[2]] = if i[0] == r[i[1]] { 1 } else { 0 },
            Opcode::Eqri => r[i[2]] = if r[i[0]] == i[1] { 1 } else { 0 },
            Opcode::Eqrr => r[i[2]] = if r[i[0]] == r[i[1]] { 1 } else { 0 },
        }
    }
}

#[derive(Debug)]
struct Instruction {
    op: Opcode,
    args: [usize; 3],
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"([a-z]{4}) (\d+) (\d+) (\d+)").unwrap();
        }
        let c = RE
            .captures(s)
            .ok_or_else(|| format!("cannot parse string {:?}", s))
            .unwrap();

        let op: Opcode = Opcode::from_str(&c[1]).unwrap();
        let args_1: usize = c[2].parse().unwrap();
        let args_2: usize = c[3].parse().unwrap();
        let args_3: usize = c[4].parse().unwrap();

        let args = [args_1, args_2, args_3];
        Ok(Instruction { op, args })
    }
}

pub fn answer1() {
    let s = std::fs::read_to_string("input/input19.txt").expect("cannot read file");
    let mut reg = [0; 4];

    println!("Go With The Flow (1/2): {:?}", reg[0]);
}

#[test]
fn test_parse_instruction() {
    let i = Instruction::from_str(r#"seti 5 0 1"#).unwrap();
    println!("{:?}", i);

    assert_eq!(Opcode::Seti, i.op);
    assert_eq!(5, i.args[0]);
    assert_eq!(0, i.args[1]);
    assert_eq!(1, i.args[2]);
}
