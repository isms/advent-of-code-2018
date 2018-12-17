use crate::utils;
use std::collections::HashMap;

#[derive(Debug, Copy, Clone)]
pub enum Opcode {
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

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct CPU {
    r: [usize; 4],
}

impl CPU {
    pub fn from_vec(rs: &Vec<usize>) -> Self {
        if rs.len() != 4 {
            panic!("CPUs must have exactly 4 registers");
        }
        let array: [usize; 4] = [rs[0], rs[1], rs[2], rs[3]];
        CPU { r: array }
    }

    // addr (add register) stores into register C the result of adding register A and register B.
    pub fn addr(self, inst: Instruction) -> CPU {
        let mut next = self.clone();
        next.r[inst.c] = next.r[inst.a] + next.r[inst.b];
        next
    }
    // addi (add immediate) stores into register C the result of adding register A and value B.
    pub fn addi(self, inst: Instruction) -> CPU {
        let mut next = self.clone();
        next.r[inst.c] = next.r[inst.a] + inst.b;
        next
    }
    // mulr (multiply register) stores into register C the result of multiplying register A and
    // register B.
    pub fn mulr(self, inst: Instruction) -> CPU {
        let mut next = self.clone();
        next.r[inst.c] = next.r[inst.a] * next.r[inst.b];
        next
    }
    // muli (multiply immediate) stores into register C the result of multiplying register A and
    // value B.
    pub fn muli(self, inst: Instruction) -> CPU {
        let mut next = self.clone();
        next.r[inst.c] = next.r[inst.a] * inst.b;
        next
    }
    // banr (bitwise AND register) stores into register C the result of the bitwise AND of register
    // A and register B.
    pub fn banr(self, inst: Instruction) -> CPU {
        let mut next = self.clone();
        next.r[inst.c] = next.r[inst.a] & next.r[inst.b];
        next
    }
    // bani (bitwise AND immediate) stores into register C the result of the bitwise AND of register
    // A and value B.
    pub fn bani(self, inst: Instruction) -> CPU {
        let mut next = self.clone();
        next.r[inst.c] = next.r[inst.a] & inst.b;
        next
    }
    // borr (bitwise OR register) stores into register C the result of the bitwise OR of register A
    // and register B.
    pub fn borr(self, inst: Instruction) -> CPU {
        let mut next = self.clone();
        next.r[inst.c] = next.r[inst.a] | next.r[inst.b];
        next
    }
    // bori (bitwise OR immediate) stores into register C the result of the bitwise OR of register A
    // and value B.
    pub fn bori(self, inst: Instruction) -> CPU {
        let mut next = self.clone();
        next.r[inst.c] = next.r[inst.a] | inst.b;
        next
    }
    // setr (set register) copies the contents of register A into register C. (Input B is ignored.)
    pub fn setr(self, inst: Instruction) -> CPU {
        let mut next = self.clone();
        next.r[inst.c] = next.r[inst.a];
        next
    }
    // seti (set immediate) stores value A into register C. (Input B is ignored.)
    pub fn seti(self, inst: Instruction) -> CPU {
        let mut next = self.clone();
        next.r[inst.c] = inst.a;
        next
    }
    // gtir (greater-than immediate/register) sets register C to 1 if value A is greater than
    // register B. Otherwise, register C is set to 0.
    pub fn gtir(self, inst: Instruction) -> CPU {
        let mut next = self.clone();
        next.r[inst.c] = match inst.a > next.r[inst.b] {
            true => 1,
            false => 0,
        };
        next
    }
    // gtri (greater-than register/immediate) sets register C to 1 if register A is greater than
    // value B. Otherwise, register C is set to 0.
    pub fn gtri(self, inst: Instruction) -> CPU {
        let mut next = self.clone();
        next.r[inst.c] = match next.r[inst.a] > inst.b {
            true => 1,
            false => 0,
        };
        next
    }
    // gtrr (greater-than register/register) sets register C to 1 if register A is greater than
    // register B. Otherwise, register C is set to 0.
    pub fn gtrr(self, inst: Instruction) -> CPU {
        let mut next = self.clone();
        next.r[inst.c] = match next.r[inst.a] > next.r[inst.b] {
            true => 1,
            false => 0,
        };
        next
    }
    // eqir (equal immediate/register) sets register C to 1 if value A is equal to register B.
    // Otherwise, register C is set to 0.
    pub fn eqir(self, inst: Instruction) -> CPU {
        let mut next = self.clone();
        next.r[inst.c] = match inst.a == next.r[inst.b] {
            true => 1,
            false => 0,
        };
        next
    }
    // eqri (equal register/immediate) sets register C to 1 if register A is equal to value B.
    // Otherwise, register C is set to 0.
    pub fn eqri(self, inst: Instruction) -> CPU {
        let mut next = self.clone();
        next.r[inst.c] = match next.r[inst.a] == inst.b {
            true => 1,
            false => 0,
        };
        next
    }
    // eqrr (equal register/register) sets register C to 1 if register A is equal to register B.
    // Otherwise, register C is set to 0.
    pub fn eqrr(self, inst: Instruction) -> CPU {
        let mut next = self.clone();
        next.r[inst.c] = match next.r[inst.a] == next.r[inst.b] {
            true => 1,
            false => 0,
        };
        next
    }

    pub fn execute(self, inst: Instruction) -> CPU {
        // do nothing
        match inst.opcode {
            Opcode::Addr => self.addr(inst),
            Opcode::Addi => self.addi(inst),
            Opcode::Mulr => self.mulr(inst),
            Opcode::Muli => self.muli(inst),
            Opcode::Banr => self.banr(inst),
            Opcode::Bani => self.bani(inst),
            Opcode::Borr => self.borr(inst),
            Opcode::Bori => self.bori(inst),
            Opcode::Setr => self.setr(inst),
            Opcode::Seti => self.seti(inst),
            Opcode::Gtir => self.gtir(inst),
            Opcode::Gtri => self.gtri(inst),
            Opcode::Gtrr => self.gtrr(inst),
            Opcode::Eqir => self.eqir(inst),
            Opcode::Eqri => self.eqri(inst),
            Opcode::Eqrr => self.eqrr(inst),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Instruction {
    opcode: Opcode,
    a: usize,
    b: usize,
    c: usize,
}

pub fn parse_fixtures(text: &str) -> Vec<(CPU, Vec<usize>, CPU)> {
    let mut fixtures = vec![];
    for case in text.split("\n\n") {
        let lines: Vec<&str> = case.split("\n").collect();

        let before: Vec<usize> = lines[0][9..19]
            .split(", ")
            .map(|x| x.parse::<usize>().expect("Bad before case"))
            .collect();
        let instructions: Vec<usize> = lines[1]
            .split(" ")
            .map(|x| x.parse::<usize>().expect("Bad instructions"))
            .collect();
        let after: Vec<usize> = lines[2][9..19]
            .split(", ")
            .map(|x| x.parse::<usize>().expect("Bad after case"))
            .collect();
        fixtures.push((CPU::from_vec(&before), instructions, CPU::from_vec(&after)));
    }
    fixtures
}

pub fn parse_program(text: &str) -> Vec<Vec<usize>> {
    let flat_codes: Vec<usize> = text
        .split_whitespace()
        .map(|c| c.parse::<usize>().unwrap())
        .collect();
    let groups4: Vec<_> = flat_codes.chunks_exact(4).map(|x| x.to_vec()).collect();
    groups4
}

pub fn part1(fixtures: Vec<(CPU, Vec<usize>, CPU)>) -> usize {
    let mut possibilities: HashMap<usize, Vec<Opcode>> = HashMap::new();
    let all_opcodes = vec![
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
    ];
    for (before, instr, after) in fixtures {
        let (i, a, b, c) = (instr[0], instr[1], instr[2], instr[3]);
        let mut narrowed: Vec<Opcode> = vec![];
        let curr = possibilities
            .entry(i)
            .or_insert(all_opcodes.clone())
            .to_vec();
        for opcode in curr.into_iter() {
            let instruction = Instruction { opcode, a, b, c };
            if before.execute(instruction) == after {
                narrowed.push(opcode);
            }
        }
        possibilities.insert(i, narrowed);
    }
    possibilities
        .values()
        .into_iter()
        .map(|c| {
            println!("{:?}", c);
            c.len()
        })
        .filter(|l| *l >= 3)
        .count()
}

pub fn run() {
    let filename = "inputs/16/input.txt";
    let contents: String = utils::read_input(&filename);

    let split: Vec<&str> = contents.split("\n\n\n").collect();
    let fixtures = parse_fixtures(split[0]);
    let _program = parse_program(split[1]);

    println!("part 1: {:?}", part1(fixtures));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let fixtures = parse_fixtures("Before: [3, 2, 1, 1]\n9 2 1 2\nAfter:  [3, 2, 2, 1]");
        assert_eq!(fixtures.len(), 1);
        part1(fixtures);
    }

    #[test]
    fn test_addr() {
        let before = CPU::from_vec(&vec![0, 1, 2, 3]);
        let instruction = Instruction {
            opcode: Opcode::Addr,
            a: 1,
            b: 3,
            c: 0,
        };
        let after = CPU::from_vec(&vec![4, 1, 2, 3]);
        assert_eq!(before.execute(instruction), after);
    }

    #[test]
    fn test_addi() {
        let before = CPU::from_vec(&vec![0, 1, 0, 0]);
        let instruction = Instruction {
            opcode: Opcode::Addi,
            a: 1,
            b: 3,
            c: 0,
        };
        let after = CPU::from_vec(&vec![4, 1, 0, 0]);
        assert_eq!(before.execute(instruction), after);
    }

    #[test]
    fn test_mulr() {
        let before = CPU::from_vec(&vec![2, 2, 0, 0]);
        let instruction = Instruction {
            opcode: Opcode::Mulr,
            a: 0,
            b: 1,
            c: 3,
        };
        let after = CPU::from_vec(&vec![2, 2, 0, 4]);
        assert_eq!(before.execute(instruction), after);
    }

    #[test]
    fn test_muli() {
        let before = CPU::from_vec(&vec![2, 2, 0, 0]);
        let instruction = Instruction {
            opcode: Opcode::Muli,
            a: 0,
            b: 1,
            c: 3,
        };
        let after = CPU::from_vec(&vec![2, 2, 0, 2]);
        assert_eq!(before.execute(instruction), after);
    }

    #[test]
    fn test_banr() {
        let before = CPU::from_vec(&vec![5, 2, 0, 0]);
        let instruction = Instruction {
            opcode: Opcode::Banr,
            a: 0,
            b: 1,
            c: 3,
        };
        let after = CPU::from_vec(&vec![5, 2, 0, 0]);
        assert_eq!(before.execute(instruction), after);
    }

    #[test]
    fn test_bani() {
        let before = CPU::from_vec(&vec![5, 2, 0, 7]);
        let instruction = Instruction {
            opcode: Opcode::Bani,
            a: 0,
            b: 1,
            c: 3,
        };
        let after = CPU::from_vec(&vec![5, 2, 0, 1]);
        assert_eq!(before.execute(instruction), after);
    }

    #[test]
    fn test_borr() {
        let before = CPU::from_vec(&vec![5, 2, 0, 0]);
        let instruction = Instruction {
            opcode: Opcode::Borr,
            a: 0,
            b: 1,
            c: 3,
        };
        let after = CPU::from_vec(&vec![5, 2, 0, 7]);
        assert_eq!(before.execute(instruction), after);
    }

    #[test]
    fn test_bori() {
        let before = CPU::from_vec(&vec![5, 2, 0, 7]);
        let instruction = Instruction {
            opcode: Opcode::Bori,
            a: 0,
            b: 1,
            c: 3,
        };
        let after = CPU::from_vec(&vec![5, 2, 0, 5]);
        assert_eq!(before.execute(instruction), after);
    }

    #[test]
    fn test_setr() {
        let before = CPU::from_vec(&vec![0, 1, 0, 0]);
        let instruction = Instruction {
            opcode: Opcode::Setr,
            a: 1,
            b: 0,
            c: 3,
        };
        let after = CPU::from_vec(&vec![0, 1, 0, 1]);
        assert_eq!(before.execute(instruction), after);
    }

    #[test]
    fn test_seti() {
        let before = CPU::from_vec(&vec![0, 1, 0, 0]);
        let instruction = Instruction {
            opcode: Opcode::Seti,
            a: 2,
            b: 0,
            c: 3,
        };
        let after = CPU::from_vec(&vec![0, 1, 0, 2]);
        assert_eq!(before.execute(instruction), after);
    }

    #[test]
    fn test_gtir() {
        let before = CPU::from_vec(&vec![0, 0, 0, 0]);
        let instruction = Instruction {
            opcode: Opcode::Gtir,
            a: 1,
            b: 0,
            c: 3,
        };
        let after = CPU::from_vec(&vec![0, 0, 0, 1]);
        assert_eq!(before.execute(instruction), after);
    }

    #[test]
    fn test_gtri() {
        let before = CPU::from_vec(&vec![0, 1, 0, 0]);
        let instruction = Instruction {
            opcode: Opcode::Gtri,
            a: 1,
            b: 0,
            c: 3,
        };
        let after = CPU::from_vec(&vec![0, 1, 0, 1]);
        assert_eq!(before.execute(instruction), after);
    }

    #[test]
    fn test_gtrr() {
        let before = CPU::from_vec(&vec![0, 0, 0, 0]);
        let instruction = Instruction {
            opcode: Opcode::Gtrr,
            a: 0,
            b: 1,
            c: 3,
        };
        let after = CPU::from_vec(&vec![0, 0, 0, 0]);
        assert_eq!(before.execute(instruction), after);
    }

    #[test]
    fn test_eqir() {
        let before = CPU::from_vec(&vec![0, 0, 0, 0]);
        let instruction = Instruction {
            opcode: Opcode::Eqir,
            a: 0,
            b: 0,
            c: 3,
        };
        let after = CPU::from_vec(&vec![0, 0, 0, 1]);
        assert_eq!(before.execute(instruction), after);
    }

    #[test]
    fn test_eqri() {
        let before = CPU::from_vec(&vec![0, 2, 0, 0]);
        let instruction = Instruction {
            opcode: Opcode::Eqri,
            a: 1,
            b: 2,
            c: 3,
        };
        let after = CPU::from_vec(&vec![0, 2, 0, 1]);
        assert_eq!(before.execute(instruction), after);
    }

    #[test]
    fn test_eqrr() {
        let before = CPU::from_vec(&vec![1, 0, 1, 0]);
        let instruction = Instruction {
            opcode: Opcode::Eqrr,
            a: 0,
            b: 2,
            c: 3,
        };
        let after = CPU::from_vec(&vec![1, 0, 1, 1]);
        assert_eq!(before.execute(instruction), after);
    }
}
