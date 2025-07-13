use std::fmt::Display;

const WIDTH: usize = 100;

// bit 1 = trap
type Row = u128;

fn next_row(row: Row) -> Row {
    let mut new_row = Row::default();

    for x in 0..WIDTH {
        let left = if x == 0 {
            false
        } else {
            row & (1 << (x - 1)) != 0
        };
        let center = row & (1 << x) != 0;
        let right = if x == WIDTH - 1 {
            false
        } else {
            row & (1 << (x + 1)) != 0
        };

        let is_trap = matches!((left, center, right), 
        (true, true, false) |
        (false, true, true) |
        (true, false, false) |
        (false, false, true)
        );

        if is_trap {
            new_row |= 1 << x;
        }
    }

    new_row
}

fn load(s: &str) -> Row {
    let mut row = Row::default();
    for (i, c) in s.chars().enumerate() {
        if c == '^' {
            row |= 1 << i;
        }
    }
    row
}

fn store(row: Row) -> String {
    let mut s = String::new();
    for x in 0..WIDTH {
        if row & (1 << x) != 0 {
            s.push('^');
        } else {
            s.push('.');
        }
    }
    s
}

#[inline]
pub fn solve() -> (impl Display, impl Display) {
    let mut row = load(include_str!("input.txt"));
    let mut part1 = 0;
    for _ in 0..40 {
        part1 += row.count_zeros() as usize - (128 - WIDTH);
        row = next_row(row);
    }

    let mut part2 = part1;
    for _ in 0..400000 - 40 {
        part2 += row.count_zeros() as usize - (128 - WIDTH);
        row = next_row(row);
    }

    (part1, part2)
}
