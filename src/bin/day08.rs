#![feature(test)]
extern crate test;
extern crate scan_fmt;

pub fn day08(input: String) -> (usize, usize) {
    let mut grid = input
        .lines()
        .map(|line|
             line
                 .chars()
                 .map(|c| ((c as i32) - ('0' as i32), false))
                 .collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let h = grid.len();
    let w = grid[0].len();

    for y in 0..h {
        let mut v = -1;
        let mut u = -1;
        for x in 0..w {
            if grid[y][x].0 > v {
                grid[y][x].1 = true;
                v = grid[y][x].0;
            }
            if grid[y][w-x-1].0 > u {
                grid[y][w-x-1].1 = true;
                u = grid[y][w-x-1].0;
            }
        }
    }
    for x in 0..w {
        let mut v = -1;
        let mut u = -1;
        for y in 0..h {
            if grid[y][x].0 > v {
                grid[y][x].1 = true;
                v = grid[y][x].0;
            }
            if grid[h-y-1][x].0 > u {
                grid[h-y-1][x].1 = true;
                u = grid[h-y-1][x].0;
            }
        }
    }

    let p1 = grid
        .iter()
        .map(|row| row.iter().filter(|(_, v)| *v).count())
        .sum();

    let mut p2 = 0;
    for y in 0..h {
        for x in 0..w {
            let v = grid[y][x].0;

            let right = (x+1..w).position(|x1| grid[y][x1].0 >= v).map(|v| v + 1).unwrap_or(w-x-1);
            let left = (0..x).position(|x1| grid[y][x-x1-1].0 >= v).map(|v| v + 1).unwrap_or(x);

            let down = (y+1..h).position(|y1| grid[y1][x].0 >= v).map(|v| v + 1).unwrap_or(h-y-1);
            let up = (0..y).position(|y1| grid[y-y1-1][x].0 >= v).map(|v| v + 1).unwrap_or(y);

            let score = right*left*down*up;

            p2 = p2.max(score);
        }
    }

    (p1, p2)
}

aoc2022::day!(day08, bench_day08);
