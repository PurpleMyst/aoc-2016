use std::fmt::Display;

#[derive(Clone, Copy)]
struct Robot {
    low_to: Destination,
    high_to: Destination,
}

#[derive(Clone, Copy)]
struct Destination {
    ty: &'static str,
    n: u8,
}

fn add(hands: (Option<u8>, Option<u8>), chip: u8) -> (Option<u8>, Option<u8>) {
    match hands {
        (Some(l), None) => (Some(l), Some(chip)),
        (None, None) => (Some(chip), None),
        _ => unreachable!(),
    }
}

#[inline]
pub fn solve() -> (impl Display, impl Display) {
    let mut robots = [None; 256];
    let mut values = [(None, None); 256];
    for line in include_str!("input.txt").lines() {
        let mut parts = line.split(' ');
        if line.starts_with("bot") {
            let robot_id = parts.nth(1).unwrap().parse::<usize>().unwrap();
            let low_to_ty = parts.nth(3).unwrap();
            let low_to_n = parts.nth(0).unwrap().parse::<u8>().unwrap();
            let low_to = Destination { ty: low_to_ty, n: low_to_n };
            let high_to_ty = parts.nth(3).unwrap();
            let high_to_n = parts.nth(0).unwrap().parse::<u8>().unwrap();
            let high_to = Destination { ty: high_to_ty, n: high_to_n };
            robots[robot_id] = Some(Robot { low_to, high_to });
        } else {
            let value = parts.nth(1).unwrap().parse::<u8>().unwrap();
            let robot_id = parts.nth(3).unwrap().parse::<usize>().unwrap();

            if values[robot_id].0.is_none() {
                values[robot_id].0 = Some(value);
            } else {
                values[robot_id].1 = Some(value);
            }
        }
    }

    let mut part1 = 0;
    let mut bin0 = 0;
    let mut bin1 = 0;
    let mut bin2 = 0;

    while bin0 == 0 || bin1 == 0 || bin2 == 0 {
        let robot_id = values.iter()
            .position(|(a, b)| a.is_some() && b.is_some())
            .unwrap();

        let robot = robots[robot_id].unwrap();
        let (Some(a), Some(b)) = values[robot_id] else { unreachable!() };
        values[robot_id] = (None, None);

        let low = a.min(b);
        let high = a.max(b);

        match robot.low_to.ty {
            "bot" => values[robot.low_to.n as usize] = add(values[robot.low_to.n as usize], low),
            "output" => {
                if robot.low_to.n <= 2 {
                let mut bins = [&mut bin0, &mut bin1, &mut bin2];
                let bin = &mut bins[robot.low_to.n as usize];
                if **bin == 0 { **bin = low; }
                }
            }
            ty => unreachable!("{ty:?}"),
        }
        match robot.high_to.ty {
            "bot" => values[robot.high_to.n as usize] = add(values[robot.high_to.n as usize], high),
            "output" => {
                if robot.high_to.n <= 2 {
                let mut bins = [&mut bin0, &mut bin1, &mut bin2];
                let bin = &mut bins[robot.high_to.n as usize];
                if **bin == 0 { **bin = high; }
                }
            }
            _ => unreachable!(),
        }

        if low == 17 && high == 61 {
            part1 = robot_id;
        }

    }
    let part2 = u16::from(bin0) * u16::from(bin1) * u16::from(bin2);

    (part1, part2)
}
