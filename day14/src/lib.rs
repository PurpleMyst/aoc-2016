use std::fmt::Display;

use rayon::prelude::*;

fn do_solve(hasher: impl Fn(u32) -> String + Send + Sync) -> usize {

    let hashes = (0..65_356)
        .into_par_iter()
        .map(hasher)
        .collect::<Vec<_>>();

    let mut keys = hashes.iter().enumerate().filter(|(i, hash)| {
        let Some(window) = hash.as_bytes()
            .windows(3)
            .find(|window| window[0] == window[1] && window[1] == window[2]) else { return false };
    
                hashes
                    .iter()
                    .skip(*i + 1)
                    .take(1000)
                    .any(|h| h.as_bytes().windows(5).any(|w| w.iter().all(|&c| c == window[0])))
    });

     keys.nth(63).unwrap().0
}

#[inline]
pub fn solve() -> (impl Display, impl Display) {
    let salt = include_str!("input.txt").trim();
    let part1 = do_solve(|n| format!("{:x}", md5::compute(format!("{salt}{n}"))));
    let part2 = do_solve(|n| {
        let mut state = format!("{:x}", md5::compute(format!("{salt}{n}")));
        for _ in 0..2016 {
            state = format!("{:x}", md5::compute(&state));
        }
        state
    });
    (part1, part2)
}
