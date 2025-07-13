use std::fmt::Display;

#[inline]
pub fn solve() -> (impl Display, impl Display) {
    // https://en.wikipedia.org/wiki/Josephus_problem
    let n = include_str!("input.txt").trim().parse::<u64>().unwrap();
    let l = n - (1 << (64 - n.leading_zeros() - 1));
    let part1 = 2 * l + 1;

    (part1, "TODO")
}
