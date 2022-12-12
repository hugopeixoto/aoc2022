#![feature(test)]
extern crate test;
extern crate scan_fmt;

use scan_fmt::scan_fmt;

pub fn day04(input: String) -> (usize, usize) {
    let p1 = input
        .lines()
        .map(|line| scan_fmt!(&line, "{}-{},{}-{}", usize, usize, usize, usize).unwrap())
        .map(|(a,b,c,d)| (a..=b, c..=d))
        .filter(|(a,b)|
                (a.contains(&b.start()) && a.contains(&b.end())) ||
                (b.contains(&a.start()) && b.contains(&a.end())))
        .count();

    let p2 = input
        .lines()
        .map(|line| scan_fmt!(&line, "{}-{},{}-{}", usize, usize, usize, usize).unwrap())
        .map(|(a,b,c,d)| (a..=b, c..=d))
        .filter(|(a,b)|
                (a.contains(&b.start()) || a.contains(&b.end())) ||
                (b.contains(&a.start()) || b.contains(&a.end())))
        .count();

    (p1, p2)
}

aoc2022::day!(day04, bench_day04);
