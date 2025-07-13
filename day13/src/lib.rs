use std::{collections::{hash_map::Entry, VecDeque}, fmt::Display};

use pathfinding::prelude::*;
use arrayvec::ArrayVec;
use rustc_hash::{FxHashMap, FxHashSet};

const PART1_TARGET: (u8, u8) = (31, 39);
const PART2_STEPS: u8 = 50;

fn manhattan_distance(a: (u8, u8), b: (u8, u8)) -> u64 {
    a.0.abs_diff(b.0) as u64 + a.1.abs_diff(b.1) as u64
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
struct State {
    pos: (u8, u8),
}

impl State {
    fn advance(&self) -> ArrayVec<Self, 4> {
        let mut next_states = ArrayVec::new();
        let (x, y) = self.pos;

        next_states.push(State { pos: (x + 1, y) });
        next_states.push(State { pos: (x, y + 1) });
        if x != 0 {
            next_states.push(State { pos: (x - 1, y) });
        }
        if y != 0 {
            next_states.push(State { pos: (x, y - 1) });
        }

        next_states
    }
}

fn is_wall(designer_number: u64, pos: (u8, u8)) -> bool {
    let x = pos.0 as u64;
    let y = pos.1 as u64;

    let n = x * x + 3 * x + 2 * x * y + y + y * y + designer_number;

    n.count_ones() % 2 != 0
}

#[inline]
pub fn solve() -> (impl Display, impl Display) {
    let designer_number = atoi::atoi(include_bytes!("input.txt")).unwrap();

    let (_, part1) = astar(
        &State { pos: (1, 1) },
        |state| {
            state.advance()
                .into_iter()
                .filter(|s| !is_wall(designer_number, s.pos))
                .map(|s| (s, 1))
        },
        |state| manhattan_distance(state.pos, PART1_TARGET),
        |state| state.pos == PART1_TARGET,
    ).unwrap();

    let mut q = VecDeque::new();
    q.push_back((State {pos:(1, 1)}, 0));
    let mut reachable = FxHashSet::default();
    while let Some((pos, depth)) = q.pop_front() {
        if depth > PART2_STEPS || !reachable.insert(pos) {
            continue;
        }

        q.extend(
        pos.advance()
            .into_iter()
            .filter(|s| !is_wall(designer_number, s.pos))
            .map(|s| (s, depth + 1))
        );
    }

    (part1, reachable.len())
}
