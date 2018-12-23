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
