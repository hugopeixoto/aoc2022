#![feature(test)]
extern crate test;

pub fn prio(c: char) -> usize {
    if c.is_uppercase() {
        c as usize + 26 + 1 - ('A' as usize)
    } else {
        c as usize + 1 - ('a' as usize)
    }
}

pub fn day03(input: String) -> (usize, usize) {
    let p1 = input
        .lines()
        .map(|line| {
            let chars = line.chars().collect::<Vec<_>>();

            for i in 0..chars.len()/2 {
                if chars[chars.len()/2..].contains(&chars[i]) {
                    return chars[i];
                }
            }

            panic!();
        })
        .map(prio)
    .sum();

    let p2 = input.lines().collect::<Vec<_>>();

    let mut p2s = 0;
    for i in 0..p2.len()/3 {
        for c in p2[i*3].chars() {
            if p2[i*3+1].contains(c) && p2[i*3+2].contains(c) {
                p2s += prio(c);
                break;
            }
        }
    }

    (p1, p2s)
}

aoc2022::day!(day03, bench_day03);
