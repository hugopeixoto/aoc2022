#![feature(test)]
extern crate test;

pub fn day01(input: String) -> (usize, usize) {
    let mut elves = vec![0];
    for line in input.lines() {
        if line == "" {
            elves.push(0);
        } else {
            *elves.last_mut().unwrap() += line.parse::<usize>().unwrap();
        }
    }

    elves.sort_by(|a, b| b.cmp(a));

    (elves[0], elves.iter().take(3).sum::<usize>())
}

aoc2022::day!(day01, bench_day01);
