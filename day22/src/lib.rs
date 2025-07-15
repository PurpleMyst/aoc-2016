use std::fmt::Display;

const WIDTH: u8 = 38;
const HEIGHT: u8 = 28;

const GOAL: (u8, u8) = (0, 0);
const START: (u8, u8) = (WIDTH - 1, 0);

const WALL_THRESHOLD: u8 = 99;

fn pos2idx(pos: (u8, u8)) -> usize {
    (pos.0 as usize * HEIGHT as usize) + pos.1 as usize
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Node {
    pos: (u8, u8),
    size: u8,
    used: u8,
}

type Nodes = [Node; WIDTH as usize * HEIGHT as usize];

impl Node {
    fn parse(s: &str) -> Self {
        let mut parts = s.split_ascii_whitespace();
        let pos_str = parts.next().unwrap();
        let x = pos_str
            .strip_prefix("/dev/grid/node-x")
            .unwrap()
            .split_once('-')
            .unwrap()
            .0
            .parse()
            .unwrap();
        let y = pos_str.rsplit_once("-y").unwrap().1.parse().unwrap();
        let pos = (x, y);
        // ASSUMPTION: >99 is always a wall
        let size = parts.next().unwrap().strip_suffix('T').unwrap().parse().unwrap_or(u8::MAX);
        let used = parts.next().unwrap().strip_suffix('T').unwrap().parse().unwrap_or(u8::MAX);
        Self { pos, size, used }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct State {
    target_pos: (u8, u8),
    empty_pos: (u8, u8),
}

impl State {
    fn done(&self) -> bool {
        self.target_pos == GOAL
    }

    fn advance(self, nodes: &'static Nodes) -> impl IntoIterator<Item = Self> {
        [(0, 1), (1, 0), (0, -1), (-1, 0)]
            .into_iter()
            .filter_map(move |(dy, dx)| {
                let (x, y) = self
                    .empty_pos
                    .0
                    .checked_add_signed(dx)
                    .zip(self.empty_pos.1.checked_add_signed(dy))?;
                if x >= WIDTH || y >= HEIGHT {
                    return None;
                }

                if nodes[pos2idx(self.empty_pos)].size > WALL_THRESHOLD {
                    return None;
                }

                Some(Self {
                    target_pos: if self.target_pos == (x, y) {
                        self.empty_pos
                    } else {
                        self.target_pos
                    },
                    empty_pos: (x, y),
                })
            })
    }

    fn heuristic(&self) -> usize {
        (self.target_pos.0.abs_diff(GOAL.0) as usize + self.target_pos.1.abs_diff(GOAL.1) as usize)
            + (self.target_pos.0.abs_diff(self.empty_pos.0) as usize
                + self.target_pos.1.abs_diff(self.empty_pos.1) as usize)
    }
}

#[inline]
pub fn solve() -> (impl Display, impl Display) {
    let nodes: &'static Nodes = Box::leak(Box::new(
        Nodes::try_from(
            include_str!("input.txt")
                .lines()
                .skip(2)
                .map(Node::parse)
                .collect::<Vec<_>>(),
        )
        .unwrap(),
    ));

    let empty_node = nodes.iter().find(|n| n.used == 0).unwrap();
    let part1 = nodes.iter().filter(|n| n.used != 0 && n.used < empty_node.size).count();

    let (_, part2) = pathfinding::prelude::astar(
        &State {
            target_pos: START,
            empty_pos: empty_node.pos,
        },
        |state| state.advance(nodes).into_iter().map(|s| (s, 1)),
        |state| state.heuristic(),
        |state| state.done(),
    )
    .unwrap();

    (part1, part2)
}
