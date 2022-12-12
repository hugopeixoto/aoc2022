#![feature(test)]
extern crate test;
extern crate scan_fmt;

use std::path::PathBuf;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Dir {
    children: Vec<PathBuf>,
    file_size: usize,
    total_size: Option<usize>,
}

fn df(dir: &PathBuf, dirs: &mut HashMap<PathBuf, Dir>) -> usize {
    let info = (*dirs.get(dir).unwrap()).clone();

    if let Some(x) = info.total_size {
        return x;
    }

    let dir_size = info.children.iter().map(|c| df(c, dirs)).sum::<usize>();

    let total_size = dir_size + info.file_size;

    dirs.get_mut(dir).unwrap().total_size = Some(total_size);
    return total_size;
}

pub fn day07(input: String) -> (usize, usize) {
    let mut dirs = HashMap::new();

    let mut cwd = PathBuf::new();
    for line in input.lines() {
        if line == "$ cd .." {
            cwd.pop();
        } else if line == "$ cd /" {
            cwd = PathBuf::from("/");
            dirs.insert(cwd.clone(), Dir { children: vec![], file_size: 0, total_size: None });
        } else if line.starts_with("$ cd") {
            cwd.push(line.split(" ").nth(2).unwrap().to_string());
            dirs.insert(cwd.clone(), Dir { children: vec![], file_size: 0, total_size: None });
        } else if line == "$ ls" {
            // lol nothing
        } else if line.starts_with("dir") {
            let mut subdir = cwd.clone();
            subdir.push(line.split(" ").nth(1).unwrap().to_string());
            dirs.get_mut(&cwd).unwrap().children.push(subdir);
        } else {
            let size = line.split(" ").nth(0).unwrap().parse::<usize>().unwrap();
            dirs.get_mut(&cwd).unwrap().file_size += size;
        }
    }

    df(&PathBuf::from("/"), &mut dirs);

    let p1 = dirs
        .values()
        .map(|dir| dir.total_size.unwrap())
        .filter(|size| *size <= 100000)
        .sum();

    let available = 70000000 - dirs[&PathBuf::from("/")].total_size.unwrap();
    let must_delete = 30000000 - available;

    let p2 = dirs
        .values()
        .map(|dir| dir.total_size.unwrap())
        .filter(|size| *size >= must_delete)
        .min()
        .unwrap();

    (p1, p2)
}

aoc2022::day!(day07, bench_day07);
