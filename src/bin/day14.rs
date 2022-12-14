#![feature(test)]
#![feature(iter_array_chunks)]

extern crate test;
extern crate scan_fmt;
extern crate serde_json;

use scan_fmt::scan_fmt;

pub fn day14(input: String) -> (usize, usize) {
    let paths = input
        .lines()
        .map(|line|
             line
             .split(" -> ")
             .map(|p| scan_fmt!(&p, "{},{}", i32, i32).unwrap())
             .collect::<Vec<_>>())
        .collect::<Vec<_>>();

    println!("{:?}", paths);

    let mut grid = std::collections::HashSet::new();

    for path in paths.iter() {
        for i in 1..path.len() {
            let p = path[i - 1];
            let q = path[i];
            let di = (q.0 - p.0).signum();
            let dj = (q.1 - p.1).signum();
            let distance = (q.0 - p.0).abs().max((q.1 - p.1).abs());

            for d in 0..= distance {
                let r = (p.0 + d*di, p.1 + d*dj);
                grid.insert(r);
            }
        }
    }

    let bounds = grid.iter().fold((500, 0, 0, 0), |(a,b,c,d), (i,j)| {
        (a.min(*i), b.min(*j), c.max(*i), d.max(*j))
    });

    for y in bounds.1 ..= bounds.3 {
        for x in bounds.0 ..= bounds.2 {
            print!("{}", if grid.contains(&(x,y)) { '#' } else { '.' });
        }
        println!("");
    }

    let mut p1 = 0;
    let mut p1grid = grid.clone();
    while drop_sand(&mut p1grid, 500, 0, bounds.3) {
        p1 += 1;
    }

    let mut p2 = 0;
    let mut p2grid = grid.clone();
    while drop_sand2(&mut p2grid, 500, 0, bounds.3) {
        p2 += 1;
    }

    (p1, p2 + 1)
}

fn drop_sand(grid: &mut std::collections::HashSet<(i32, i32)>, x: i32, y: i32, maxy: i32) -> bool {
    if y > maxy {
        return false;
    }

    for p in [(x,y+1), (x-1,y+1), (x+1,y+1)] {
        if !grid.contains(&p) {
            return drop_sand(grid, p.0, p.1, maxy);
        }
    }

    grid.insert((x,y));
    true
}

fn drop_sand2(grid: &mut std::collections::HashSet<(i32, i32)>, x: i32, y: i32, maxy: i32) -> bool {
    if y == maxy + 1 {
        grid.insert((x,y));
        return true;
    }

    for p in [(x,y+1), (x-1,y+1), (x+1,y+1)] {
        if !grid.contains(&p) {
            return drop_sand2(grid, p.0, p.1, maxy);
        }
    }

    grid.insert((x,y));
    (x,y) != (500, 0)
}

aoc2022::day!(day14, bench_day14);
