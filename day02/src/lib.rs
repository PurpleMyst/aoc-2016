use std::fmt::Display;

use Option::{None as NNNNNN, Some as S};

const P1_SIDE: u8 = 3;
const P2_SIDE: u8 = 5;

#[rustfmt::skip]
const P1_MATRIX: [Option<char>; (P1_SIDE * P1_SIDE) as usize] = [
    S('1'), S('2'), S('3'),
    S('4'), S('5'), S('6'),
    S('7'), S('8'), S('9'),
];

#[rustfmt::skip]
const P2_MATRIX: [Option<char>; (P2_SIDE * P2_SIDE) as usize] = [
    NNNNNN, NNNNNN, S('1'), NNNNNN, NNNNNN,
    NNNNNN, S('2'), S('3'), S('4'), NNNNNN,
    S('5'), S('6'), S('7'), S('8'), S('9'),
    NNNNNN, S('A'), S('B'), S('C'), NNNNNN,
    NNNNNN, NNNNNN, S('D'), NNNNNN, NNNNNN,
];

fn next_pos<const S: u8>((x, y): (u8, u8), ch: char) -> Option<(u8, u8)> {
    Some(match ch {
        'U' if y > 0 => (x, y - 1),
        'D' if y < S - 1 => (x, y + 1),
        'L' if x > 0 => (x - 1, y),
        'R' if x < S - 1 => (x + 1, y),
        _ => return None,
    })
}

fn solve_part<const S: u8, const T: usize>(mut pos: (u8, u8), matrix: [Option<char>; T]) -> String {
    debug_assert_eq!((S * S) as usize, T);
    include_str!("input.txt")
        .trim()
        .lines()
        .map(|line| {
            line.chars().for_each(|ch| {
                if let Some(next_pos) = next_pos::<{ S }>(pos, ch) {
                    if matrix
                        .get(usize::from(S * next_pos.1 + next_pos.0))
                        .unwrap()
                        .is_some()
                    {
                        pos = next_pos;
                    }
                }
            });

            matrix[usize::from(pos.1 * S + pos.0)].unwrap()
        })
        .collect::<String>()
}

#[inline]
pub fn solve() -> (impl Display, impl Display) {
    (
        solve_part::<P1_SIDE, { (P1_SIDE * P1_SIDE) as usize }>((1, 1), P1_MATRIX),
        solve_part::<P2_SIDE, { (P2_SIDE * P2_SIDE) as usize }>((0, 2), P2_MATRIX),
    )
}
