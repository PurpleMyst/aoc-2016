use std::fmt::Display;

use rustc_hash::FxHashSet as HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
struct ItemSet(u32);

impl ItemSet {
    #[must_use]
    fn add(self, element: u8) -> Self {
        Self(self.0 | (1 << (element - b'A')))
    }

    #[must_use]
    fn sub(self, element: u8) -> Self {
        Self(self.0 & !(1 << (element - b'A')))
    }

    fn has(self, element: u8) -> bool {
        (self.0 & (1 << (element - b'A'))) != 0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
struct Floor {
    generators: ItemSet,
    microchips: ItemSet,
}

impl Floor {
    #[must_use]
    fn add_microchip(self, element: u8) -> Self {
        Self {
            microchips: self.microchips.add(element),
            ..self
        }
    }

    #[must_use]
    fn add_generator(self, element: u8) -> Self {
        Self {
            generators: self.generators.add(element),
            ..self
        }
    }

    #[must_use]
    fn sub_microchip(self, element: u8) -> Self {
        Self {
            microchips: self.microchips.sub(element),
            ..self
        }
    }

    #[must_use]
    fn sub_generator(self, element: u8) -> Self {
        Self {
            generators: self.generators.sub(element),
            ..self
        }
    }

    fn is_safe(self) -> bool {
        self.generators.0 == 0 || (self.generators.0 & self.microchips.0) == self.microchips.0
    }
}

const FLOORS: usize = 4;
type Floors = [Floor; FLOORS];

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
struct State {
    floors: Floors,
    pos: usize,
}

impl State {
    fn advance(self) -> impl IntoIterator<Item = Self> {
        let mut next_states = HashSet::default();
        let current_floor = self.floors[self.pos];

        for element in b'A'..=b'Z' {
            // Try to bring this chip up and down
            if current_floor.microchips.has(element) {
                for dir in [-1, 1] {
                    let Some(next_pos) = self.pos.checked_add_signed(dir).filter(|&p| p < FLOORS) else {
                        continue;
                    };
                    let next_floor = self.floors[next_pos];

                    let new_current_floor = current_floor.sub_microchip(element);
                    let new_next_floor = next_floor.add_microchip(element);

                    if new_current_floor.is_safe() && new_next_floor.is_safe() {
                        let mut new_floors = self.floors;
                        new_floors[self.pos] = new_current_floor;
                        new_floors[next_pos] = new_next_floor;

                        next_states.insert(State {
                            floors: new_floors,
                            pos: next_pos,
                        });
                    }
                }
            }

            if current_floor.generators.has(element) {
                for dir in [-1, 1] {
                    let Some(next_pos) = self.pos.checked_add_signed(dir).filter(|&p| p < FLOORS) else {
                        continue;
                    };
                    let next_floor = self.floors[next_pos];

                    let new_current_floor = current_floor.sub_generator(element);
                    let new_next_floor = next_floor.add_generator(element);

                    if new_current_floor.is_safe() && new_next_floor.is_safe() {
                        let mut new_floors = self.floors;
                        new_floors[self.pos] = new_current_floor;
                        new_floors[next_pos] = new_next_floor;

                        next_states.insert(State {
                            floors: new_floors,
                            pos: next_pos,
                        });
                    }
                }
            }

            // if current_floor.microchips.has(element)
            // && current_floor.generators.has(element) {
            //     for dir in [-1, 1] {
            //         let Some(next_pos) = self.pos.checked_add_signed(dir).filter(|&p| p < FLOORS) else {
            //             continue;
            //         };
            //         let next_floor = self.floors[next_pos];
            //
            //         let new_current_floor = current_floor.sub_generator(element).sub_microchip(element);
            //         let new_next_floor = next_floor.add_generator(element).add_microchip(element);
            //
            //         if new_current_floor.is_safe() && new_next_floor.is_safe() {
            //             let mut new_floors = self.floors;
            //             new_floors[self.pos] = new_current_floor;
            //             new_floors[next_pos] = new_next_floor;
            //
            //             next_states.push(State {
            //                 floors: new_floors,
            //                 pos: next_pos,
            //             })
            //         }
            //     }
            //
            // }
        }

        for element1 in b'A'..=b'Z' {
            for element2 in b'A'..=b'Z' {
                if current_floor.microchips.has(element1) && current_floor.microchips.has(element2) {
                    for dir in [-1, 1] {
                        let Some(next_pos) = self.pos.checked_add_signed(dir).filter(|&p| p < FLOORS) else {
                            continue;
                        };
                        let next_floor = self.floors[next_pos];

                        let new_current_floor = current_floor.sub_microchip(element1).sub_microchip(element2);
                        let new_next_floor = next_floor.add_microchip(element1).add_microchip(element2);

                        if new_current_floor.is_safe() && new_next_floor.is_safe() {
                            let mut new_floors = self.floors;
                            new_floors[self.pos] = new_current_floor;
                            new_floors[next_pos] = new_next_floor;

                            next_states.insert(State {
                                floors: new_floors,
                                pos: next_pos,
                            });
                        }
                    }
                }

                if current_floor.generators.has(element1) && current_floor.generators.has(element2) {
                    for dir in [-1, 1] {
                        let Some(next_pos) = self.pos.checked_add_signed(dir).filter(|&p| p < FLOORS) else {
                            continue;
                        };
                        let next_floor = self.floors[next_pos];

                        let new_current_floor = current_floor.sub_generator(element1).sub_generator(element2);
                        let new_next_floor = next_floor.add_generator(element1).add_generator(element2);

                        if new_current_floor.is_safe() && new_next_floor.is_safe() {
                            let mut new_floors = self.floors;
                            new_floors[self.pos] = new_current_floor;
                            new_floors[next_pos] = new_next_floor;

                            next_states.insert(State {
                                floors: new_floors,
                                pos: next_pos,
                            });
                        }
                    }
                }

                if current_floor.generators.has(element1) && current_floor.microchips.has(element2) {
                    for dir in [-1, 1] {
                        let Some(next_pos) = self.pos.checked_add_signed(dir).filter(|&p| p < FLOORS) else {
                            continue;
                        };
                        let next_floor = self.floors[next_pos];

                        let new_current_floor = current_floor.sub_generator(element1).sub_microchip(element2);
                        let new_next_floor = next_floor.add_generator(element1).add_microchip(element2);

                        if new_current_floor.is_safe() && new_next_floor.is_safe() {
                            let mut new_floors = self.floors;
                            new_floors[self.pos] = new_current_floor;
                            new_floors[next_pos] = new_next_floor;

                            next_states.insert(State {
                                floors: new_floors,
                                pos: next_pos,
                            });
                        }
                    }
                }
            }
        }

        next_states
    }

    fn target(&self) -> Self {
        let have = self.floors
            .iter()
            .fold(0, |acc, floor| acc | floor.generators.0 | floor.microchips.0);

        Self {
            floors: [Floor::default(), Floor::default(), Floor::default(), Floor { generators: ItemSet(have), microchips: ItemSet(have) }],
            pos: FLOORS - 1,
        }
    }
}

fn show(State { floors, pos }: &State) {
    let have = floors
        .iter()
        .fold(0, |acc, floor| acc | floor.generators.0 | floor.microchips.0);

    for (i, floors) in floors.iter().enumerate().rev() {
        print!("\x1b[{}mF{} ", 31 + i, i + 1);
        print!("{} ", if i == *pos { "E" } else { "." });
        for i in b'A'..=b'Z' {
            if floors.generators.has(i) {
                print!("{}G ", i as char);
            } else if have & (1 << (i - b'A')) != 0 {
                print!(".  ");
            }
            if floors.microchips.has(i) {
                print!("{}M ", i as char);
            } else if have & (1 << (i - b'A')) != 0 {
                print!(".  ");
            }
        }
        println!("\x1b[0m");
    }
}

#[inline]
pub fn solve() -> (impl Display, impl Display) {
    let mut floors = Floors::default();

    include_str!("input.txt")
        .lines()
        .zip(floors.iter_mut())
        .for_each(|(description, floor)| {
            regex::Regex::new(r"([a-z]+)(-compatible)? (microchip|generator)")
                .unwrap()
                .captures_iter(description)
                .for_each(|cap| {
                    let element = cap[1].as_bytes()[0].to_ascii_uppercase();
                    if &cap[3] == "microchip" {
                        floor.microchips = floor.microchips.add(element);
                    } else {
                        floor.generators = floor.generators.add(element);
                    }
                });
        });

    let mut initial_state = State { floors, pos: 0 };
    let mut target_state = initial_state.target();

    let (_, part1) = pathfinding::prelude::dijkstra(
        &initial_state,
        |state| state.advance().into_iter().map(move |state| (state, 1)),
        |state| *state == target_state,
    ).unwrap();

    initial_state.floors[0] = initial_state.floors[0].add_microchip(b'E');
    initial_state.floors[0] = initial_state.floors[0].add_generator(b'E');
    initial_state.floors[0] = initial_state.floors[0].add_microchip(b'D');
    initial_state.floors[0] = initial_state.floors[0].add_generator(b'D');
    target_state = initial_state.target();

    let (_, part2) = pathfinding::prelude::dijkstra(
        &initial_state,
        |state| state.advance().into_iter().map(move |state| (state, 1)),
        |state| *state == target_state,
    ).unwrap();

    (part1, part2)
}
