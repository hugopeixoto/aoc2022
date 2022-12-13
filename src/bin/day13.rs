#![feature(test)]
#![feature(iter_array_chunks)]

extern crate test;
extern crate scan_fmt;
extern crate serde_json;

use serde_json::Value;

fn compare(a: &Value, b: &Value) -> std::cmp::Ordering {
    match (a, b) {
        (Value::Number(a), Value::Number(b)) => a.as_u64().unwrap().cmp(&b.as_u64().unwrap()),
        (Value::Array(a), Value::Array(b)) => {
            for i in 0..a.len().min(b.len()) {
                let cmp = compare(&a[i], &b[i]);
                if cmp != std::cmp::Ordering::Equal {
                    return cmp;
                }
            }
            a.len().cmp(&b.len())
        },
        (Value::Number(a), Value::Array(b)) => compare(&Value::Array(vec![Value::Number(a.clone())]), &Value::Array(b.clone())),
        (Value::Array(a), Value::Number(b)) => compare(&Value::Array(a.clone()), &Value::Array(vec![Value::Number(b.clone())])),
        _ => { panic!("unhandled comparison: {} vs {}", a, b); }
    }
}

pub fn day13(input: String) -> (usize, usize) {
    let packet_pairs = input
        .split("\n\n")
        .map(|poop|
             poop
                 .lines()
                 .map(|line| serde_json::from_str::<Value>(line).unwrap())
                 .collect::<Vec<_>>()
         )
        .collect::<Vec<_>>();

    let p1 = packet_pairs
        .iter()
        .enumerate()
        .filter(|(_, pp)| compare(&pp[0], &pp[1]) != std::cmp::Ordering::Greater)
        .map(|(i, _)| i + 1)
        .sum();

    let mut packets = packet_pairs.iter().flatten().cloned().collect::<Vec<_>>();

    let packet2 = serde_json::from_str::<Value>("[[2]]").unwrap();
    let packet6 = serde_json::from_str::<Value>("[[6]]").unwrap();

    packets.push(packet2.clone());
    packets.push(packet6.clone());

    packets.sort_by(|a,b| compare(a, b));

    let p2 = (1 + packets.iter().position(|p| p == &packet2).unwrap()) *
              (1 + packets.iter().position(|p| p == &packet6).unwrap());

    (p1, p2)
}

aoc2022::day!(day13, bench_day13);
