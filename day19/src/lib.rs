use std::fmt::Display;

#[inline]
pub fn solve() -> (impl Display, impl Display) {
    // https://en.wikipedia.org/wiki/Josephus_problem
    let n = include_str!("input.txt").trim().parse::<u32>().unwrap();
    let l = n - (1 << (32 - n.leading_zeros() - 1));
    let part1 = 2 * l + 1;

    // https://www.reddit.com/r/adventofcode/comments/5j4lp1/2016_day_19_solutions/dbdf50n/
    let mut part2 = 1;
    while part2 * 3 < n {
        part2 *= 3;
    }

    (part1, n - part2)
}
