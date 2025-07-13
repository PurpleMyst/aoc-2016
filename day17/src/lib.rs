use std::fmt::Display;

use arrayvec::ArrayVec;
use pathfinding::prelude::*;

const PASSCODE: &str = include_str!("input.txt");
// const PASSCODE: &str = "hijkl";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Into<char> for Direction {
    fn into(self) -> char {
        match self {
            Direction::Up => 'U',
            Direction::Down => 'D',
            Direction::Left => 'L',
            Direction::Right => 'R',
        }
    }
}

impl Into<usize> for Direction {
    fn into(self) -> usize {
        match self {
            Direction::Up => 0,
            Direction::Down => 1,
            Direction::Left => 2,
            Direction::Right => 3,
        }
    }
}

#[derive(Clone, Hash, PartialEq, Eq, Debug)]
struct State {
    pos: (usize, usize),
    path: String,
}

impl Default for State {
    fn default() -> Self {
        Self { path: PASSCODE.trim().to_string(), pos: (0, 0) }
    }
}

impl State {
    fn move_in(mut self, direction: Direction) -> Option<Self> {
        match direction {
            Direction::Up if self.pos.1 > 0 => self.pos.1 -= 1,
            Direction::Down if self.pos.1 < 3 => self.pos.1 += 1,
            Direction::Left if self.pos.0 > 0 => self.pos.0 -= 1,
            Direction::Right if self.pos.0 < 3 => self.pos.0 += 1,
            _ => return None,
        }

        let hash = format!("{:x}", md5::compute(self.path.as_bytes()));
        self.path.push(direction.into());
        let determinant = hash.chars().nth(direction.into()).unwrap();
        let can_move = matches!(determinant, 'b'..='f');
        can_move.then_some(self)
    }
}

#[inline]
pub fn solve() -> (impl Display, impl Display) {
    let (states, _) = astar(
        &State::default(),
        |state| {
            [
                Direction::Up,
                Direction::Down,
                Direction::Left,
                Direction::Right,
            ]
            .iter()
            .filter_map(|&dir| state.clone().move_in(dir)) .map(|s| (s, 1))
            .collect::<ArrayVec<_, 4>>()
        },
        |_state| 0,
        |state| state.pos == (3, 3),
    ).unwrap();

    let binding = states.into_iter().last().unwrap();
    let part1 = binding.path.strip_prefix(PASSCODE.trim()).unwrap().to_owned();

    let part2_state = bfs_reach(
        State::default(),
        |state| {
            if state.pos == (3, 3) {
                return ArrayVec::<State, 4>::default();
            }
            [
                Direction::Up,
                Direction::Down,
                Direction::Left,
                Direction::Right,
            ]
            .iter()
            .filter_map(|&dir| state.clone().move_in(dir)) 
            .collect::<ArrayVec<_, 4>>()
        },
    ).filter(|state| state.pos == (3, 3))
    .max_by_key(|state| state.path.len()).unwrap();

    (part1, part2_state.path.len() - PASSCODE.trim().len())
}
