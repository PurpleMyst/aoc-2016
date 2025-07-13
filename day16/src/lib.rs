use std::fmt::Display;

const TARGET_LEN_PART1: usize = 272;
const TARGET_LEN_PART2: usize = 35_651_584;

fn step(a: &[u8]) -> Vec<u8> {
    let mut output = Vec::with_capacity(a.len() * 2 + 1);
    output.extend_from_slice(a);
    output.push(b'0');
    output.extend(
    a.iter()
        .copied()
        .rev()
        .map(|b| if b == b'0' { b'1' } else { b'0' })
    );
    output
}

fn checksum(b: Vec<u8>) -> Vec<u8> {
    if b.len() % 2 == 1 {
        return b;
    }
    let mut output = Vec::with_capacity(b.len() / 2);
    for i in (0..b.len()).step_by(2) {
        output.push(if b[i] == b[i + 1] { b'1' } else { b'0' });
    }
    checksum(output)
}

fn do_solve(len: usize) -> String {
    let mut state = include_str!("input.txt").trim().as_bytes().to_vec();
    while state.len() < len {
        state = step(&state);
    }
    state.truncate(len);
    String::from_utf8(checksum(state)).unwrap()
}

#[inline]
pub fn solve() -> (impl Display, impl Display) {
    (
        do_solve(TARGET_LEN_PART1),
        do_solve(TARGET_LEN_PART2),
    )
}
