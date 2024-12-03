use std::{
    fs::File,
    io::{BufRead, BufReader},
    ops::RangeInclusive,
};

use itertools::Itertools;

const RANGE_INCLUSIVE: RangeInclusive<i32> = 1..=3;

fn records() -> impl Iterator<Item = Vec<i32>> {
    let reader = BufReader::new(File::open("d2p1/input/input.txt").unwrap());
    reader.lines().map(Result::unwrap).map(|line| {
        line.split_ascii_whitespace()
            .map(str::parse)
            .map(Result::unwrap)
            .collect()
    })
}

fn gradually_ascending(record: &[i32]) -> bool {
    record
        .into_iter()
        .tuple_windows()
        .all(|(left, right)| RANGE_INCLUSIVE.contains(&(right - left)))
}

fn gradually_descending(record: &[i32]) -> bool {
    record
        .into_iter()
        .tuple_windows()
        .all(|(left, right)| RANGE_INCLUSIVE.contains(&(left - right)))
}

fn main() {
    let safe = records()
        .filter(|record| gradually_ascending(record) || gradually_descending(record))
        .count();

    println!("Safe count: {}", safe);
}
