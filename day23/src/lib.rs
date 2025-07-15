use std::fmt::Display;

#[derive(Debug, Clone, Copy)]
enum Operand {
    Integer(i64),
    Register(u8),
}

impl Operand {
    fn parse(buf: &[u8]) -> Self {
        if buf.len() == 1 && buf[0].is_ascii_alphabetic() {
            Self::Register(buf[0] - b'a')
        } else {
            let value = atoi::atoi(buf).unwrap();
            Self::Integer(value)
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Cpy(Operand, Operand),
    Inc(Operand),
    Dec(Operand),
    Jnz(Operand, Operand),
    Tgl(Operand),
}

struct Interpreter {
    registers: [i64; 4],
    pc: usize,
}

impl Interpreter {
    fn eval(&self, operand: &Operand) -> i64 {
        match operand {
            Operand::Integer(value) => *value,
            Operand::Register(index) => self.registers[*index as usize],
        }
    }

    fn dst(&mut self, operand: &Operand) -> Option<&mut i64> {
        match operand {
            Operand::Register(index) => Some(&mut self.registers[*index as usize]),
            _ => None,
        }
    }

    fn run(&mut self, instructions: &mut [Instruction]) {
        while self.pc < instructions.len() {
        match &instructions[self.pc] {
            Instruction::Cpy(x, y) => {
                let x_val = self.eval(x);
                if let Some(dest) = self.dst(y) {
                    *dest = x_val;
                }
                self.pc += 1;
            }
            Instruction::Inc(x) => {
                if let Some(reg) = self.dst(x) {
                    *reg += 1;
                }
                self.pc += 1;
            }
            Instruction::Dec(x) => {
                if let Some(reg) = self.dst(x) {
                    *reg -= 1;
                }
                self.pc += 1;
            }
            Instruction::Jnz(x, y) => {
                if self.eval(x) != 0 {
                    self.pc = self.pc.checked_add_signed(self.eval(y) as isize).unwrap_or(usize::MAX);
                } else {
                    self.pc += 1;
                }
            }
            Instruction::Tgl(x) => {
                let offset = self.eval(x) as isize;
                if let Some(target_instr) = self.pc.checked_add_signed(offset).and_then(|pc| instructions.get_mut(pc)) {
                    let new_instr = match *target_instr {
                        Instruction::Cpy(operand, operand1) => Instruction::Jnz(operand, operand1),
                        Instruction::Inc(operand) => Instruction::Dec(operand),
                        Instruction::Dec(operand) |
                        Instruction::Tgl(operand) => Instruction::Inc(operand),
                        Instruction::Jnz(operand, operand1) => Instruction::Cpy(operand, operand1),
                    };
                    *target_instr = new_instr;
                }
                self.pc += 1;
            }
        }
        }
    }
}

#[inline]
pub fn solve() -> (impl Display, impl Display) {
    let mut instructions = include_bytes!("input.txt")
        .split(|&b| b == b'\n')
        .filter(|line| !line.is_empty())
        .map(|line| {
            let mut parts = line.split(|&b| b.is_ascii_whitespace());
            let opcode = parts.next().unwrap();
            match opcode {
                b"cpy" => {
                    let dest = Operand::parse(parts.next().unwrap());
                    let src = Operand::parse(parts.next().unwrap());
                    Instruction::Cpy(dest, src)
                }
                b"inc" => {
                    let reg = Operand::parse(parts.next().unwrap());
                    Instruction::Inc(reg)
                }
                b"dec" => {
                    let reg = Operand::parse(parts.next().unwrap());
                    Instruction::Dec(reg)
                }
                b"tgl" => {
                    let reg = Operand::parse(parts.next().unwrap());
                    Instruction::Tgl(reg)
                }
                b"jnz" => {
                    let cond = Operand::parse(parts.next().unwrap());
                    let offset = Operand::parse(parts.next().unwrap());
                    Instruction::Jnz(cond, offset)
                }
                _ => panic!("Unknown instruction: {}", String::from_utf8_lossy(opcode)),
            }
        })
        .collect::<Vec<_>>();

    let mut part1 = Interpreter {
        registers: [7,0,0,0],
        pc: 0,
    };
    part1.run(&mut instructions.clone());

    let mut part2 = Interpreter {
        registers: [12,0,0,0],
        pc: 0,
    };
    part2.run(&mut instructions);

    (part1.registers[0], part2.registers[0])
}
