use std::fmt::Display;

#[derive(Debug)]
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

#[derive(Debug)]
enum Instruction {
    Cpy(Operand, Operand),
    Inc(Operand),
    Dec(Operand),
    Jnz(Operand, Operand),
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

    fn dst(&mut self, operand: &Operand) -> &mut i64 {
        match operand {
            Operand::Register(index) => &mut self.registers[*index as usize],
            _ => panic!("Destination must be a register"),
        }
    }

    fn run(&mut self, instructions: &[Instruction]) {
        while self.pc < instructions.len() {
            match &instructions[self.pc] {
                Instruction::Cpy(x, y) => *self.dst(y) = self.eval(x),
                Instruction::Inc(x) => {
                    let reg = self.dst(x);
                    *reg += 1;
                }
                Instruction::Dec(x) => {
                    let reg = self.dst(x);
                    *reg -= 1;
                }
                Instruction::Jnz(x, y) => {
                    if self.eval(x) != 0 {
                        self.pc = self.pc.checked_add_signed(self.eval(y) as isize).unwrap_or(usize::MAX);
                        continue;
                    }
                }
            }

            self.pc += 1;
        }
    }
}

#[inline]
pub fn solve() -> (impl Display, impl Display) {
    let instructions = include_bytes!("input.txt")
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
        registers: [0; 4],
        pc: 0,
    };
    part1.run(&instructions);

    let mut part2 = Interpreter {
        registers: [0, 0, 1, 0],
        pc: 0,
    };
    part2.run(&instructions);

    (part1.registers[0], part2.registers[0])
}
