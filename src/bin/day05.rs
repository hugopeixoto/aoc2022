#![feature(test)]
extern crate test;
extern crate scan_fmt;

use scan_fmt::scan_fmt;

pub fn day05(input: String) -> (usize, usize) {
    let x = input
        .split("\n\n")
        .collect::<Vec<_>>();

    let stacks = (input.lines().next().unwrap().len() - 3) / 4 + 1;

    let initial_state = (0..stacks)
        .map(|s|
            x[0] 
                 .lines()
                 .map(|line| line.chars().nth(s * 4+1).unwrap())
                 .filter(char::is_ascii_alphabetic)
                 .collect::<std::collections::VecDeque<_>>())
        .collect::<Vec<_>>();

    let moves = x[1]
        .lines()
        .map(|line| scan_fmt!(&line, "move {} from {} to {}", usize, usize, usize).unwrap())
        .collect::<Vec<_>>();

    let mut state = initial_state.clone();
    for (c, from, to) in moves.iter() {
        let from = from-1;
        let to = to-1;

        for _ in 1..=*c {
            let v = state[from].pop_front().unwrap();
            state[to].push_front(v);
        }
    }
    let p1 = state.iter().map(|v| v[0]).collect::<String>();
    println!("{}", p1);

    let mut state = initial_state.clone();
    for (c, from, to) in moves.iter() {
        let from = from-1;
        let to = to-1;

        let mut tmp = vec![];
        for _ in 1..=*c {
            let v = state[from].pop_front().unwrap();
            tmp.push(v);
        }
        tmp.reverse();
        for v in tmp {
            state[to].push_front(v);
        }
    }
    let p1 = state.iter().map(|v| v[0]).collect::<String>();
    println!("{}", p1);


    (0, 0)
}

aoc2022::day!(day05, bench_day05);
