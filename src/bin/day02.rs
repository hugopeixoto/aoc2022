#![feature(test)]
extern crate test;

fn score1(line: &str) -> usize {
    match line {
        "A X" => 1 + 3,
        "A Y" => 2 + 6,
        "A Z" => 3 + 0,
        "B X" => 1 + 0,
        "B Y" => 2 + 3,
        "B Z" => 3 + 6,
        "C X" => 1 + 6,
        "C Y" => 2 + 0,
        "C Z" => 3 + 3,
        _ => { panic!("unknown matchup: {}", line); }
    }
}

fn score2(line: &str) -> usize {
    match line {
        "A X" => 3 + 0,
        "A Y" => 1 + 3,
        "A Z" => 2 + 6,
        "B X" => 1 + 0,
        "B Y" => 2 + 3,
        "B Z" => 3 + 6,
        "C X" => 2 + 0,
        "C Y" => 3 + 3,
        "C Z" => 1 + 6,
        _ => { panic!("unknown matchup: {}", line); }
    }
}

pub fn day02(input: String) -> (usize, usize) {
    let p1 = input.lines().map(score1).sum();
    let p2 = input.lines().map(score2).sum();

    (p1, p2)
}

aoc2022::day!(day02, bench_day02);
