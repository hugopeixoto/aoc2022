#![feature(test)]
extern crate test;

fn b_score(a: usize, b: usize) -> usize {
  [3, 6, 0][(b+3-a)%3]
}

fn other(a: usize, score: usize) -> usize {
    (a + score/3 + 3 - 2) % 3 + 1
}

pub fn day02(input: String) -> (usize, usize) {
    input
        .lines()
        .map(|line| (
            match &line[0..1] { "A" => 1, "B" => 2, "C" => 3, _ => { panic!(); } },
            match &line[2..] { "X" => 1, "Y" => 2, "Z" => 3, _ => { panic!(); } }
        ))
        .map(|(a,b)| (b_score(a,b) + b, (b-1)*3 + other(a, (b-1)*3)))
        .reduce(|(a,b), (c,d)| (a+c, b+d))
        .unwrap()
}

aoc2022::day!(day02, bench_day02);
