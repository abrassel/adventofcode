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

fn gradually_ascending(left: i32, right: i32) -> bool {
    RANGE_INCLUSIVE.contains(&(right - left))
}

fn gradually_descending(left: i32, right: i32) -> bool {
    RANGE_INCLUSIVE.contains(&(left - right))
}

fn do_test(record: &[i32], test: impl Fn(i32, i32) -> bool) -> bool {
    record
        .into_iter()
        .tuple_windows()
        .all(|(&left, &right)| test(left, right))
}

fn brute_force_check(record: &[i32]) -> bool {
    for i in 0..record.len() {
        let removed = [&record[..i], &record[i + 1..]].concat();
        if do_test(&removed, gradually_descending) || do_test(&removed, gradually_ascending) {
            return true;
        }
    }
    false
}

fn main() {
    let safe = records().filter(|record| brute_force_check(record)).count();

    println!("Safe count: {:?}", safe);
}
