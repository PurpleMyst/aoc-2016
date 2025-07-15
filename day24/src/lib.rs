use std::fmt::Display;

use itertools::Itertools;

const WIDTH: usize = 179;
// const HEIGHT: usize = 43;

const NUMBERS: usize = 8;


#[inline]
pub fn solve() -> (impl Display, impl Display) {
    let mut walls = [0u64; WIDTH];
    let mut numbers = [(0u8, 0u8); NUMBERS];

    include_str!("input.txt")
        .lines()
        .enumerate()
        .for_each(|(y, row)| {
            row.chars().enumerate().for_each(|(x, c)| {
                if c == '#' {
                    walls[x] |= 1 << y;
                } else if c.is_ascii_digit() {
                    let idx = c.to_digit(10).unwrap() as usize;
                    numbers[idx].0 = x as u8;
                    numbers[idx].1 = y as u8;
                }
            });
        });

    let mut dist = [[u64::MAX; NUMBERS]; NUMBERS];

    for n in 0..NUMBERS {
        dist[n][n] = 0;

        for m in n + 1..NUMBERS {
            let (_, d) = pathfinding::prelude::dijkstra(
                &numbers[n],
                |&(x, y)| {
                    [
                        (x.wrapping_sub(1), y),
                        (x + 1, y),
                        (x, y.wrapping_sub(1)),
                        (x, y + 1),
                    ]
                    .into_iter()
                    .filter(|&(x, y)| walls[x as usize] & (1 << y) == 0)
                    .map(|pos| (pos, 1))
                },
                |&pos| pos == numbers[m],
            ).unwrap();
            dist[n][m] = d;
            dist[m][n] = d;
        }
    }

    (1..NUMBERS)
        .permutations(NUMBERS - 1)
        .map(|p| {
            let part1 = dist[0][p[0]]
            + p.windows(2)
                .map(|w| dist[w[0]][w[1]])
                .sum::<u64>();
            let part2 = part1 + dist[p[NUMBERS - 2]][0];
            (part1, part2)
        })
        .fold((u64::MAX, u64::MAX), |(a, b), (c, d)| (a.min(c), b.min(d)))
}
