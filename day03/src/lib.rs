use std::fmt::Display;

fn is_valid((a, b, c): (u16, u16, u16)) -> bool {
    a + b > c && a + c > b && b + c > a
}

#[inline]
pub fn solve() -> (impl Display, impl Display) {
    let triangles = include_str!("input.txt")
        .trim()
        .lines()
        .map(|line| {
            let mut sides = line
                .split_whitespace()
                .map(|side| side.parse::<u16>().unwrap());
            let a = sides.next().unwrap();
            let b = sides.next().unwrap();
            let c = sides.next().unwrap();
            (a, b, c)
        })
        .collect::<Vec<_>>();

    let p1 = triangles.iter().filter(|&&t| is_valid(t)).count();
    let p2 = triangles
        .chunks_exact(3)
        .map(|chunk| {
            let [(a1, a2, a3), (b1, b2, b3), (c1, c2, c3)]: [(u16, u16, u16); 3] =
                chunk.try_into().unwrap();
            [(a1, b1, c1), (a2, b2, c2), (a3, b3, c3)]
                .into_iter()
                .filter(|&t| is_valid(t))
                .count()
        })
        .sum::<usize>();

    (p1, p2)
}
