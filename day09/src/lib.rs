use std::fmt::Display;

fn part1_length(input: &[u8]) -> usize {
    let mut idx = 0;
    let mut len = 0;

    while idx < input.len() {
        if input[idx] == b'(' {
            let j = idx + input[idx..].iter().position(|&b| b == b'x').unwrap();
            let a: usize = atoi::atoi(&input[idx + 1..j]).unwrap();
            let k = j + 1 + input[j + 1..].iter().position(|&b| b == b')').unwrap();
            let b: usize = atoi::atoi(&input[j + 1..k]).unwrap();
            len += a * b;
            idx = k + 1 + a;
        } else {
            idx += 1;
            len += 1;
        }
    }
    len
}

fn part2_length(input: &[u8]) -> usize {
    let mut idx = 0;
    let mut len = 0;

    while idx < input.len() {
        if input[idx] == b'(' {
            let j = idx + input[idx..].iter().position(|&b| b == b'x').unwrap();
            let chars: usize = atoi::atoi(&input[idx + 1..j]).unwrap();
            let k = j + 1 + input[j + 1..].iter().position(|&b| b == b')').unwrap();
            let reps: usize = atoi::atoi(&input[j + 1..k]).unwrap();
            len += part2_length(&input[k + 1..k + 1+chars]) * reps;
            idx = k + 1 + chars;
        } else {
            idx += 1;
            len += 1;
        }
    }

    len
}

#[inline]
pub fn solve() -> (impl Display, impl Display) {
    let input = include_str!("input.txt").trim().as_bytes();
    (part1_length(input), part2_length(input))
}

