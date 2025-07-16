use std::{collections::VecDeque, fmt::Display};

fn step(n: &mut VecDeque<u32>) {
    n.remove(n.len() / 2);
    let Some(m) = n.pop_front() else { unsafe { std::hint::unreachable_unchecked() } };
    n.push_back(m);
}

#[inline]
pub fn solve() -> (impl Display, impl Display) {
    // https://en.wikipedia.org/wiki/Josephus_problem
    let n = include_str!("input.txt").trim().parse::<u32>().unwrap();
    let l = n - (1 << (32 - n.leading_zeros() - 1));
    let part1 = 2 * l + 1;

    let mut xs: VecDeque<u32> = (1..=n).collect();
    let mut pbar = tqdm::pbar(Some(n as usize - 1));
    for _ in 0..(n - 1) {
        step(&mut xs);
        pbar.update(1).unwrap();
    }
    let part2 = xs[0];

    (part1, part2)
}
