#![feature(test)]
extern crate test;
extern crate scan_fmt;

use scan_fmt::scan_fmt;

type St = Vec<(i32, i32)>;

pub fn is_touching(h: (i32, i32), t: (i32, i32)) -> bool {
    h.0.abs_diff(t.0) < 2 && h.1.abs_diff(t.1) < 2
}

pub fn move_tail(head: (i32, i32), tail: (i32, i32)) -> (i32, i32) {
    if is_touching(head, tail) {
        tail
    } else {
        (
            if head.0 == tail.0 {
                tail.0
            } else {
                tail.0 + (head.0 - tail.0) / (head.0.abs_diff(tail.0) as i32)
            },
            if head.1 == tail.1 {
                tail.1
            } else {
                tail.1 + (head.1 - tail.1) / (head.1.abs_diff(tail.1) as i32)
            },
        )
    }
}

pub fn apply(state: &St, direction: char) -> St {
    let head = state[0];
    let head = match direction {
        'U' => (head.0, head.1 - 1),
        'D' => (head.0, head.1 + 1),
        'L' => (head.0 - 1, head.1),
        'R' => (head.0 + 1, head.1),
        '#' => head,
        _ => { panic!("unknown direction: {}", direction); }
    };

    let mut s2 = vec![head];

    for next in &state[1..] {
        s2.push(move_tail(*s2.last().unwrap(), *next));
    }

    s2
}

pub fn day09(input: String) -> (usize, usize) {
    let mut instructions = input
        .lines()
        .map(|line| scan_fmt!(&line, "{} {}", char, usize).unwrap())
        .flat_map(|(dir, count)| (0..count).map(move |_| dir))
        .collect::<Vec<_>>();
    instructions.insert(0, '#');

    let p1 = instructions
        .iter()
        .scan(vec![(0, 0), (0, 0)], |prev, &direction| {
            *prev = apply(prev, direction);

            Some(prev.clone())
        })
        .map(|rope| *rope.last().unwrap())
        .collect::<std::collections::HashSet<_>>()
        .len();

    let p2 = instructions
        .iter()
        .scan(vec![(0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0)], |prev, &direction| {
            *prev = apply(prev, direction);

            Some(prev.clone())
        })
        .map(|rope| *rope.last().unwrap())
        .collect::<std::collections::HashSet<_>>()
        .len();

    (p1, p2)
}

aoc2022::day!(day09, bench_day09);
