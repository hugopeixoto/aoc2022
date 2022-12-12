#![feature(test)]
extern crate test;
extern crate scan_fmt;

use scan_fmt::scan_fmt;

fn bits(a: &[char]) -> u32 {
    a.iter().map(|c| 1 << ((*c as u32) - ('a' as u32))).reduce(|a,b| a | b).unwrap()
}

pub fn day06(input: String) -> (usize, usize) {
    let c = input
        .chars()
        .collect::<Vec<_>>();
    let p1 = c
        .windows(4)
        .enumerate()
        .filter(|(_, a)| bits(a).count_ones() == 4)
        .next()
        .unwrap()
        .0;
    let p2 = c
        .windows(14)
        .enumerate()
        .filter(|(_, a)| bits(a).count_ones() == 14)
        .next()
        .unwrap()
        .0;

    (p1 + 4, p2 + 14)
}

aoc2022::day!(day06, bench_day06);
