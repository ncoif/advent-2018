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
#[derive(Debug)]
struct Prog {
    ip: usize,
    instructions: Vec<Instruction>,
}

impl FromStr for Prog {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.split("\n");
        let ip_s = lines.next().unwrap();
        let re_ip = Regex::new(r"#ip (\d)").unwrap();
        let ip_a = re_ip
            .captures(ip_s)
            .ok_or_else(|| format!("cannot parse ip {:?}", s))
            .unwrap();

        let ip: usize = ip_a[1].parse().unwrap();

        let mut instructions = vec![];
        for line in lines.filter(|l| *l != "") {
            instructions.push(Instruction::from_str(line).unwrap());
        }

        Ok(Prog { ip, instructions })
    }
}

impl Prog {
    fn run(&self, reg: &mut [usize]) {
        let ip = self.ip;

        loop {
            if reg[ip] == 3 {
                self.fast(reg);
            } else {
                let cur = &self.instructions[reg[ip]];
                cur.op.apply(&cur.args, reg);
                reg[ip] += 1;

                if reg[ip] >= self.instructions.len() {
                    break;
                }
            }
        }
    }

    fn fast(&self, reg: &mut [usize]) {
        // From lines 3 to 11 apreas to be a be a very inefficient way of determining whether R3 divides R2

        if reg[2] % reg[3] == 0 {
            reg[0] = reg[0] + reg[3];
        }
        reg[1] = reg[2];
        reg[4] = 0;
        reg[5] = 12;
    }
}

pub fn answer1() {
    let s = std::fs::read_to_string("input/input19.txt").expect("cannot read file");
    let prog = Prog::from_str(&s).unwrap();

    let mut reg = [0; 6];
    prog.run(&mut reg);

    println!("Day 19: Go With The Flow (1/2): {:?}", reg[0]);
}

pub fn answer2() {
    let s = std::fs::read_to_string("input/input19.txt").expect("cannot read file");
    let prog = Prog::from_str(&s).unwrap();

    let mut reg = [0; 6];
    reg[0] = 1;
    prog.run(&mut reg);

    println!("Day 19: Go With The Flow (2/2): {:?}", reg[0]);
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

#[test]
fn test_parse_prog() {
    let p = Prog::from_str(
        r#"#ip 0
seti 5 0 1
seti 6 0 2
addi 0 1 0
addr 1 2 3
setr 1 0 0
seti 8 0 4
seti 9 0 5"#,
    )
    .unwrap();
    println!("{:?}", p);

    assert_eq!(0, p.ip);
    assert_eq!(7, p.instructions.len());
}

#[test]
fn test_run_prog() {
    let p = Prog::from_str(
        r#"#ip 0
seti 5 0 1
seti 6 0 2
addi 0 1 0
addr 1 2 3
setr 1 0 0
seti 8 0 4
seti 9 0 5"#,
    )
    .unwrap();
    println!("{:?}", p);

    let mut reg = [0; 6];
    p.run(&mut reg);

    assert_eq!(6 + 1, reg[0]);
}
