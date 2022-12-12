#![feature(test)]
extern crate test;
extern crate scan_fmt;

use scan_fmt::scan_fmt;
use std::collections::VecDeque;

#[derive(Debug, Clone)]
enum Operand { Old, N(usize) }

#[derive(Debug, Clone)]
enum Operation { Add(Operand), Mul(Operand) }
impl Operation {
    pub fn apply(&self, old: usize) -> usize {
        match self {
            Self::Add(Operand::Old) => old + old,
            Self::Add(Operand::N(x)) => old + x,
            Self::Mul(Operand::Old) => old * old,
            Self::Mul(Operand::N(x)) => old * x,
        }
    }
    pub fn from(expr: &str) -> Self {
        if expr.contains("*") {
            let op = expr.split("*").nth(1).unwrap();

            Self::Mul(if op == "old" { Operand::Old } else { Operand::N(op.parse::<usize>().unwrap()) })
        } else if expr.contains("+") {
            let op = expr.split("+").nth(1).unwrap();

            Self::Add(if op == "old" { Operand::Old } else { Operand::N(op.parse::<usize>().unwrap()) })
        } else {
            panic!("unknown operation: {}", expr);
        }
    }
}

#[derive(Debug, Clone)]
struct Monkey {
    items: VecDeque<usize>,
    operation: Operation,
    test: usize,
    if_true: usize,
    if_false: usize,
    inspections: usize,
}
impl Monkey {
    pub fn handle(&self, item: usize) -> (usize, usize) {
        let item = self.operation.apply(item);
        let item = item / 3;
        (item, if item % self.test == 0 {
            self.if_true
        } else {
            self.if_false
        })
    }

    pub fn handle2(&self, item: usize, poop: usize) -> (usize, usize) {
        let item = self.operation.apply(item);
        let item = item % poop;
        (item, if item % self.test == 0 {
            self.if_true
        } else {
            self.if_false
        })
    }
}

pub fn day11(input: String) -> (usize, usize) {
    let input = input
        .replace("\n\n", "\n")
        .replace("\n  Starting items: ", "|")
        .replace("\n  Operation: new = ", "|")
        .replace("\n  Test: divisible by ", "|")
        .replace("\n    If true: throw to monkey ", "|")
        .replace("\n    If false: throw to monkey ", "|")
        .replace(" ", "")
        .lines()
        .map(|line| scan_fmt!(&line, "Monkey{}:|{}|{}|{}|{}|{}", usize, String, String, usize, usize, usize).unwrap())
        .map(|(_id, items, op, test, if_true, if_false)| {
            Monkey {
                items: items.split(",").map(|x| x.parse::<usize>().unwrap()).collect(),
                operation: Operation::from(&op),
                test,
                if_true,
                if_false,
                inspections: 0,
            }
        })
        .collect::<Vec<_>>();

    let mut monkeys = input.clone();
    for _round in 0..20 {
        for m in 0..monkeys.len() {
            while let Some(item) = monkeys[m].items.pop_front() {
                monkeys[m].inspections += 1;
                let (item, t) = monkeys[m].handle(item);
                monkeys[t].items.push_back(item);
            }
        }
    }
    monkeys.sort_by_key(|m| m.inspections);
    monkeys.reverse();

    let p1 = monkeys.iter().take(2).map(|m| m.inspections).reduce(|a,b| a*b).unwrap();


    let mut monkeys = input.clone();
    let poop = monkeys.iter().map(|m| m.test).reduce(|a,b| a*b).unwrap();
    for _round in 0..10000 {
        for m in 0..monkeys.len() {
            while let Some(item) = monkeys[m].items.pop_front() {
                monkeys[m].inspections += 1;
                let (item, t) = monkeys[m].handle2(item, poop);
                monkeys[t].items.push_back(item);
            }
        }
    }
    monkeys.sort_by_key(|m| m.inspections);
    monkeys.reverse();

    let p2 = monkeys.iter().take(2).map(|m| m.inspections).reduce(|a,b| a*b).unwrap();

    (p1, p2)
}

aoc2022::day!(day11, bench_day11);
