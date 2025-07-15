use std::fmt::Display;

#[derive(Debug, Clone, Copy)]
enum Operation {
    SwapPositions(usize, usize),
    SwapLetters(u8, u8),
    RotateLeft(usize),
    RotateRight(usize),
    RotateBasedOnPosition(u8),
    Reverse(usize, usize),
    Move(usize, usize),
}

impl Operation {
    fn parse(input: &str) -> Self {
        let mut parts = input.split(' ');
        match parts.next().unwrap() {
            "swap" => match parts.next().unwrap() {
                "position" => {
                    let x: usize = parts.next().unwrap().parse().unwrap();
                    let y: usize = parts.next_back().unwrap().parse().unwrap();
                    Operation::SwapPositions(x, y)
                }
                "letter" => {
                    let x = parts.next().unwrap().as_bytes()[0];
                    let y = parts.next_back().unwrap().as_bytes()[0];
                    Operation::SwapLetters(x, y)
                }
                _ => unreachable!(),
            },
            "rotate" => match parts.next().unwrap() {
                "left" => {
                    let x: usize = parts.next().unwrap().parse().unwrap();
                    Operation::RotateLeft(x)
                }
                "right" => {
                    let x: usize = parts.next().unwrap().parse().unwrap();
                    Operation::RotateRight(x)
                }
                "based" => {
                    let x = parts.next_back().unwrap().as_bytes()[0];
                    Operation::RotateBasedOnPosition(x)
                }
                _ => unreachable!(),
            },
            "reverse" => {
                let x: usize = parts.nth(1).unwrap().parse().unwrap();
                let y: usize = parts.next_back().unwrap().parse().unwrap();
                Operation::Reverse(x, y)
            }
            "move" => {
                let x: usize = parts.nth(1).unwrap().parse().unwrap();
                let y: usize = parts.next_back().unwrap().parse().unwrap();
                Operation::Move(x, y)
            }
            _ => unreachable!(),
        }
    }

    fn apply(&self, buf: &mut Vec<u8>) {
        match self {
            Operation::SwapPositions(x, y) => buf.swap(*x, *y),
            Operation::SwapLetters(x, y) => {
                let i = buf.iter().position(|b| b == x).unwrap();
                let j = buf.iter().position(|b| b == y).unwrap();
                buf.swap(i, j);
            }
            Operation::RotateLeft(x) => buf.rotate_left(*x),
            Operation::RotateRight(x) => buf.rotate_right(*x),
            Operation::RotateBasedOnPosition(x) => {
                let i = buf.iter().position(|b| b == x).unwrap();
                let n = 1 + i + if i >= 4 { 1 } else { 0 };
                let l = buf.len();
                buf.rotate_right(n % l);
            }
            Operation::Reverse(x, y) => {
                let slice = &mut buf[*x..*y + 1];
                slice.reverse();
            }
            Operation::Move(x, y) => {
                let b = buf.remove(*x);
                buf.insert(*y, b);
            }
        }
    }
}

fn apply_all(ops: &[Operation], buf: &mut Vec<u8>) {
    for op in ops {
        op.apply(buf);
    }
}

#[inline]
pub fn solve() -> (impl Display, impl Display) {
    let operations = include_str!("input.txt")
        .lines()
        .map(Operation::parse)
        .collect::<Vec<_>>();
    let mut part1 = b"abcdefgh".to_vec();
    apply_all(&operations, &mut part1);
    let part1 = String::from_utf8(part1).unwrap();

    let mut part2 = b"fbgdceah".to_vec();
    loop {
        let prev = part2.clone();
        apply_all(&operations, &mut part2);
        if part2 == b"fbgdceah" {
            break (part1, String::from_utf8(prev).unwrap());
        }
    }
}
