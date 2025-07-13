use std::fmt::Display;

#[derive(Debug, Clone, Copy)]
struct Disc {
    positions: u8,
    initial_pos: u8,
}

impl Disc {
    fn parse(s: &str) -> Self {
        let mut parts = s.split_whitespace();
        let positions = parts.nth(3).unwrap().parse().unwrap();
        let initial_pos = parts.last().unwrap().strip_suffix('.').unwrap().parse().unwrap();
        Self { positions, initial_pos }
    }
}

#[inline]
pub fn solve() -> (impl Display, impl Display) {
    let input = include_str!("input.txt");
    let discs: Vec<Disc> = input.lines().map(Disc::parse).collect();

    let mut u = Vec::new();
    let mut m = Vec::new();

    for (i, disc) in discs.iter().enumerate() {
        u.push(-(disc.initial_pos as i128) - (i + 1) as i128);
        m.push(disc.positions as i128);
    }
    let part1 = ring_algorithm::chinese_remainder_theorem(&u, &m).unwrap();
    u.push(-(discs.len() as i128 + 1));
    m.push(11);
    let part2 = ring_algorithm::chinese_remainder_theorem(&u, &m).unwrap() + m.iter().product::<i128>();

    (part1, part2)
}
