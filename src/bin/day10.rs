#![feature(test)]
extern crate test;

#[derive(Debug)]
enum Inst {
    Noop,
    Add(i32),
}

pub fn day10(input: String) -> (usize, usize) {
    let signals = input
        .lines()
        .map(|line|
             if line == "noop" {
                Inst::Noop
            } else {
                Inst::Add(line.split(" ").nth(1).unwrap().parse::<i32>().unwrap())
            }
        )
        .flat_map(|inst| match inst {
            Inst::Noop => vec![Inst::Noop],
            Inst::Add(x) => vec![Inst::Noop, Inst::Add(x)],
        })
        .scan(1, |prev, inst| {
              let v = *prev;
              match inst {
                  Inst::Noop => {},
                  Inst::Add(x) => { *prev += x; },
              }

              Some(v)
        })
        .collect::<Vec<_>>();


    let p1 = signals
        .iter()
        .enumerate()
        .filter(|(i, _)| (i + 21) % 40 == 0)
        .map(|(i, s)| (i as i32 + 1) * s)
        .sum::<i32>();

    let crt = signals
        .iter()
        .enumerate()
        .map(|(i, s)| (s-1 ..= s+1).contains(&((i as i32) % 40)))
        .collect::<Vec<_>>();

    for i in 0..6 {
        for j in 0..40 {
            print!("{}", if crt[i*40 + j] { '#' } else { '.' });
        }
        println!("");
    }


    (p1 as usize, 0)
}

aoc2022::day!(day10, bench_day10);
