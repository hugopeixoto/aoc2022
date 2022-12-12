#![feature(test)]
extern crate test;
extern crate scan_fmt;

use std::collections::VecDeque;

struct GridIterator {
    h: usize,
    w: usize,
    y: usize,
    x: usize,
}

impl GridIterator {
    pub fn from(h: usize, w: usize) -> Self {
        Self { h, w, y: 0, x: 0 }
    }
}

impl Iterator for GridIterator {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        let item = (self.y, self.x);

        if self.x + 1 == self.w {
            self.y += 1;
            self.x = 0;
        } else {
            self.x += 1;
        }

        if item.0 == self.h || item.1 == self.w {
            None
        } else {
            Some(item)
        }
    }
}


pub fn neighbors(p: (usize, usize), h: usize, w: usize) -> Vec<(usize, usize)> {
    let mut qs = vec![];

    if p.0 > 0 { qs.push((p.0 - 1, p.1)); }
    if p.1 > 0 { qs.push((p.0, p.1 - 1)); }
    if p.0 + 1 < h { qs.push((p.0 + 1, p.1)); }
    if p.1 + 1 < w { qs.push((p.0, p.1 + 1)); }

    qs
}

pub fn day12(input: String) -> (usize, usize) {
    let mut s = (0, 0);
    let mut e = (0, 0);
    let mut grid: Vec<Vec<(usize, Option<usize>)>> = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line
                .chars()
                .enumerate()
                .map(|(x, c)| {
                    if c == 'S' { s = (y, x); 0 }
                    else if c == 'E' { e = (y, x); 25 }
                    else { (c as usize) - ('a' as usize) }
                })
                .map(|v| (v, None))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let h = grid.len();
    let w = grid[0].len();

    let mut queue = VecDeque::new();
    grid[e.0][e.1].1 = Some(0);
    queue.push_back(e);

    while !queue.is_empty() {
        let p = queue.pop_front().unwrap();
        let height = grid[p.0][p.1].0;
        let distance = grid[p.0][p.1].1.unwrap();

        for q in neighbors(p, h, w) {
            let height2 = grid[q.0][q.1].0;
            if height2 + 1 >= height && grid[q.0][q.1].1.is_none() {
                grid[q.0][q.1].1 = Some(distance + 1);
                queue.push_back(q);
            }
        }
    }

    let p1 = grid[s.0][s.1].1.unwrap();

    let p2 = GridIterator::from(h, w)
        .filter(|(y, x)| grid[*y][*x].0 == 0)
        .map(|(y, x)| grid[y][x].1.unwrap_or(9000))
        .min()
        .unwrap();

    (p1, p2)
}

aoc2022::day!(day12, bench_day12);
