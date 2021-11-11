use std::collections::HashSet;
use std::iter::repeat;

use std::fmt::Display;

type Vec2D = (i64, i64);

const DIRECTIONS: [Vec2D; 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

#[inline]
pub fn solve() -> (impl Display, impl Display) {
    let mut facing = 0usize;

    let mut positions = include_str!("input.txt")
        .trim()
        .split(", ")
        .flat_map(|instr| {
            let mut cs = instr.chars();
            facing = match cs.next() {
                Some('R') => facing + 1,
                Some('L') => facing.wrapping_sub(1),
                _ => unreachable!(),
            } % DIRECTIONS.len();

            let count: usize = cs.as_str().parse().unwrap();

            repeat(DIRECTIONS[facing]).take(count)
        })
        .scan((0i64, 0i64), |(x, y), (dx, dy)| {
            *x += dx;
            *y += dy;
            Some((*x, *y))
        });

    let mut visited = HashSet::new();
    let (x2, y2) = positions.find(|&pos| !visited.insert(pos)).unwrap();

    let (x1, y1) = positions.last().unwrap();

    (x1.abs() + y1.abs(), x2.abs() + y2.abs())
}
