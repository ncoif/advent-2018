use crate::common::error::AocError;
use crate::common::response::AocResponse;

use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashSet;
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
    type Err = AocError;

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
            _ => Err(AocError::InvalidToken(s.to_string())),
        }
    }
}

impl Opcode {
    fn apply(self, i: &[usize], r: &mut [usize]) {
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
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"([a-z]{4}) (\d+) (\d+) (\d+)").unwrap();
        }
        let c = RE
            .captures(s)
            .ok_or_else(|| format!("cannot parse string {:?}", s))?;

        let op: Opcode = Opcode::from_str(&c[1])?;
        let args_1: usize = c[2].parse()?;
        let args_2: usize = c[3].parse()?;
        let args_3: usize = c[4].parse()?;

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
        let mut lines = s.split('\n');
        let ip_s = lines.next().unwrap();
        let re_ip = Regex::new(r"#ip (\d)").unwrap();
        let ip_a = re_ip
            .captures(ip_s)
            .ok_or_else(|| format!("cannot parse ip {:?}", s))
            .unwrap();

        let ip: usize = ip_a[1].parse().unwrap();

        let mut instructions = vec![];
        for line in lines.filter(|l| !l.is_empty()) {
            instructions.push(Instruction::from_str(line).unwrap());
        }

        Ok(Prog { ip, instructions })
    }
}

impl Prog {
    fn run(&self, reg: &mut [usize]) {
        let ip = self.ip;

        loop {
            let cur = &self.instructions[reg[ip]];
            cur.op.apply(&cur.args, reg);
            reg[ip] += 1;

            if reg[ip] >= self.instructions.len() {
                break;
            }
        }
    }

    fn run_get_first_ip30(&self, reg: &mut [usize]) -> usize {
        let ip = self.ip;

        loop {
            if reg[ip] == 30 {
                break;
            }

            let cur = &self.instructions[reg[ip]];
            cur.op.apply(&cur.args, reg);
            reg[ip] += 1;

            if reg[ip] >= self.instructions.len() {
                break;
            }
        }

        reg[4]
    }

    // run the program in a loop, and collect all values in R4 for instructions 30, trying to find a cycle
    // return the max if a cycle is found
    fn run_with_ip30_cycle(&self, reg: &mut [usize]) -> usize {
        let mut candidates = HashSet::new();
        let mut last = None;
        let ip = self.ip;

        loop {
            if reg[ip] == 30 {
                let r4 = reg[4];
                if candidates.contains(&r4) {
                    break;
                } else {
                    candidates.insert(r4);
                    last = Some(r4);
                }
            }

            let cur = &self.instructions[reg[ip]];
            cur.op.apply(&cur.args, reg);
            reg[ip] += 1;

            if reg[ip] >= self.instructions.len() {
                break;
            }
        }

        last.unwrap()
    }
}

pub fn answer1() -> Result<AocResponse<usize>, AocError> {
    let s = std::fs::read_to_string("input/input21.txt")?;
    let prog = Prog::from_str(&s).unwrap();

    // seems that ip 30 "eqrr 4 0 5" is the only one interacting with R0
    //ip=30, reg = [0, 30, 1, 1, 15823996, 0]
    // so trying to see if using R0 15823996 halts
    let mut reg = [0; 6];
    let r0 = prog.run_get_first_ip30(&mut reg); //15823996

    let mut reg = [0; 6];
    reg[0] = r0;
    prog.run(&mut reg);

    Ok(AocResponse::new(21, 1, "Chronal Conversion", reg[0]))
}

pub fn answer2() -> Result<AocResponse<usize>, AocError> {
    let s = std::fs::read_to_string("input/input21.txt")?;
    let prog = Prog::from_str(&s).unwrap();

    // look for all possible values of R4 for ip30 when running a program, and break once we found a cycle
    let mut reg = [0; 6];
    let r0 = prog.run_with_ip30_cycle(&mut reg);

    // using this r0, run the program to confirm that it halts
    // let mut reg = [0; 6];
    // reg[0] = r0;
    // prog.run(&mut reg);

    Ok(AocResponse::new(21, 2, "Chronal Conversion", r0))
}
