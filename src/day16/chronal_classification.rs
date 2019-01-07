use crate::common::error::AocError;
use crate::common::response::AocResponse;

use std::collections::HashMap;

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

struct Sample {
    before: [usize; 4],
    input: [usize; 4],
    after: [usize; 4],
}

impl Sample {
    fn from_slice(v: &[usize]) -> [usize; 4] {
        let mut array = [0; 4];
        array.copy_from_slice(v);
        array
    }

    fn parse(s: &str) -> Vec<Sample> {
        let mut lines = s.split('\n');

        let mut samples = vec![];
        while let Some(before_line) = &lines.next() {
            // skip blank lines
            if *before_line == "" {
                continue;
            }
            let before_vec: Vec<usize> = before_line[9..]
                .trim_right_matches(']')
                .split(", ")
                .map(|t| t.parse::<usize>().unwrap())
                .collect();

            let input_line = &lines.next().unwrap();
            let input_vec: Vec<usize> = input_line[..]
                .split(' ')
                .map(|t| t.parse::<usize>().unwrap())
                .collect();

            let after_line = &lines.next().unwrap();
            let after_vec: Vec<usize> = after_line[9..]
                .trim_right_matches(']')
                .split(", ")
                .map(|t| t.parse::<usize>().unwrap())
                .collect();

            samples.push(Sample {
                before: Sample::from_slice(&before_vec),
                input: Sample::from_slice(&input_vec),
                after: Sample::from_slice(&after_vec),
            });
        }

        samples
    }

    fn probe_ops(&self, ops: impl Iterator<Item = Opcode>) -> Vec<Opcode> {
        ops.filter(|op| {
            let mut regs = self.before;
            op.apply(&self.input[1..4] as _, &mut regs);
            regs == self.after
        })
        .collect()
    }

    fn opcode(&self) -> usize {
        self.input[0]
    }
}

pub fn answer1() -> Result<AocResponse<i32>, AocError> {
    let s = std::fs::read_to_string("input/input16_q1.txt")?;
    let samples = Sample::parse(&s);

    let mut triple_candidates = 0;
    for sample in samples.iter() {
        let candidates = sample.probe_ops(Opcode::all().to_vec().iter().cloned());
        if candidates.len() >= 3 {
            triple_candidates += 1;
        }
    }

    Ok(AocResponse::new(
        16,
        1,
        "Chronal Classification",
        triple_candidates,
    ))
}

pub fn answer2() -> Result<AocResponse<usize>, AocError> {
    let s = std::fs::read_to_string("input/input16_q1.txt")?;
    let samples = Sample::parse(&s);

    let mut to_map = Opcode::all().to_vec();
    let mut mapped: HashMap<usize, Opcode> = HashMap::new();
    for sample in samples.iter().cycle() {
        if mapped.contains_key(&sample.opcode()) {
            continue;
        }
        let candidates = sample.probe_ops(to_map.iter().cloned());
        if candidates.len() == 1 {
            let op = candidates[0];
            to_map.retain(|&tm| tm != op);
            mapped.insert(sample.opcode(), op);
        }
        if to_map.is_empty() {
            break;
        }
    }

    let prog = std::fs::read_to_string("input/input16_q2.txt")?;
    let mut reg = [0; 4];
    for inst in prog.split('\n').filter(|l| !l.is_empty()) {
        let mut tokens = inst.split(' ').map(|s| s.parse::<usize>().unwrap());
        let opcode = tokens.next().unwrap() as usize;
        let op = mapped[&opcode];
        let args = [
            tokens.next().unwrap(),
            tokens.next().unwrap(),
            tokens.next().unwrap(),
        ];
        op.apply(&args, &mut reg);
    }

    Ok(AocResponse::new(16, 2, "Chronal Classification", reg[0]))
}

#[test]
fn test_addi() {
    let inputs = vec![2, 1, 2];
    let mut registers = vec![3, 2, 1, 1];

    Opcode::Addi.apply(&inputs, &mut registers);
    let expected = vec![3, 2, 2, 1];
    assert_eq!(expected, registers);
}

#[test]
fn test_mulr() {
    let inputs = vec![2, 1, 2];
    let mut registers = vec![3, 2, 1, 1];

    Opcode::Mulr.apply(&inputs, &mut registers);
    let expected = vec![3, 2, 2, 1];
    assert_eq!(expected, registers);
}

#[test]
fn test_seti() {
    let inputs = vec![2, 1, 2];
    let mut registers = vec![3, 2, 1, 1];

    Opcode::Seti.apply(&inputs, &mut registers);
    let expected = vec![3, 2, 2, 1];
    assert_eq!(expected, registers);
}
