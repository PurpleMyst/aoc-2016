use std::fmt::Display;

fn password_char_part1(door_id: &'static str, n: u64) -> Option<char> {
    let hash = format!("{:x}", md5::compute(format!("{door_id}{n}")));
    hash.starts_with("00000").then(|| hash.chars().nth(5).unwrap())
}

fn password_char_part2(door_id: &'static str, n: u64) -> Option<(usize, char)> {
    let hash = format!("{:x}", md5::compute(format!("{door_id}{n}")));
    if !hash.starts_with("00000") {
        return None;
    }
    match hash.chars().nth(5).unwrap() {
        c @ '0'..='7' => Some(((c as u8 - b'0').into(), hash.chars().nth(6).unwrap())),
        _ => None,
    }
}

#[inline]
pub fn solve() -> (impl Display, impl Display) {
    let door_id = include_str!("input.txt").trim();

    let part1 = (0..)
        .filter_map(|n| password_char_part1(door_id, n))
        .take(8)
        .collect::<String>();

    let mut part2 = ['\0'; 8];
    let mut n = 0;
    while part2.iter().any(|&ch| ch == '\0') {
        if let Some((i, ch)) = password_char_part2(door_id, n) {
            if part2[i] == '\0' {
                part2[i] = ch;
            }
        }

        n += 1;
    }

    (part1, part2.into_iter().collect::<String>())
}
