use std::fmt::Display;

use rayon::prelude::*;

#[inline]
pub fn solve() -> (impl Display, impl Display) {
    let mut ranges = include_str!("input.txt")
        .lines()
        .map(|line| line.split_once('-').unwrap())
        .map(|(a, b)| (a.parse::<u32>().unwrap(),b.parse::<u32>().unwrap()))
        .collect::<Vec<_>>();
    ranges.sort_unstable();

    let mut new_ranges = Vec::with_capacity(ranges.len());

    let mut it = ranges.clone().into_iter();
    new_ranges.push(it.next().unwrap());

    for (mut a1, b1) in it {
        let (_, b0) = new_ranges.last().copied().unwrap();

        if b1 <= b0 {
            // In this case:
            // a0 <= a1
            // b0 >= b1
            // The new range would be contained in the old one.
            continue;
        }
        
        // we know that a0 <= a1 due to the sorting
        // if b0 >= a1 we can start at b0
        if b0 >= a1 {
            a1 = b0;
        }
        new_ranges.push((a1, b1));
    }

    while let Some(idx) = new_ranges.windows(2)
            .position(|w| w[0].1 == w[1].0 || w[0].1 +1 == w[1].0) {

        new_ranges[idx].1 = new_ranges[idx + 1].1;
        new_ranges.remove(idx + 1);
    }

    let ranges = new_ranges;

    let part1 = ranges[0].1 + 1;

    let part2 = (1usize << 32) - ranges.into_iter()
        .map(|(a, b)| (b - a + 1) as usize)
        .sum::<usize>();

    (part1, part2)
}
