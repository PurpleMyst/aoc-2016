macro_rules! doit {
    ($($day:ident: $solve:ident),+$(,)?) => {
        $(use $day::solve as $solve;)+
        iai::main!($($solve),+);
    };
}

doit!(
    day01: day01_solve,
    day02: day02_solve,
    day03: day03_solve,
    day05: day05_solve,
    day09: day09_solve,
    day10: day10_solve,
    day11: day11_solve,
    day12: day12_solve,
    day13: day13_solve,
    day14: day14_solve,
);
