use std::{collections::VecDeque, fmt::Display};

// https://www.reddit.com/r/adventofcode/comments/5j4lp1/2016_day_19_solutions/dbdf9mn/
fn step(left: &mut VecDeque<u32>, right: &mut VecDeque<u32>) {
    if left.len() > right.len() {
        left.pop_back();
    } else {
        right.pop_back();
    }

    unsafe {
        right.push_front(left.pop_front().unwrap_unchecked());
        left.push_back(right.pop_back().unwrap_unchecked());
    }
}

#[inline]
pub fn solve() -> (impl Display, impl Display) {
    // https://en.wikipedia.org/wiki/Josephus_problem
    let n = include_str!("input.txt").trim().parse::<u32>().unwrap();
    let l = n - (1 << (32 - n.leading_zeros() - 1));
    let part1 = 2 * l + 1;

    let mut left: VecDeque<u32> = (1..=n / 2).collect();
    let mut right: VecDeque<u32> = (n/2 + 1..=n).rev().collect();
    for _ in 0..(n - 1) {
        step(&mut left, &mut right);
    }
    let part2 = left.pop_front().unwrap();

    (part1, part2)
}
