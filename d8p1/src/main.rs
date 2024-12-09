use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

mod vec;

use itertools::Itertools;
use vec::Vec2D;

const INPUT: &str = "d8p1/input/input.txt";

fn is_antenna(c: char) -> bool {
    c.is_ascii_alphanumeric()
}

fn main() {
    let mut rows = 0;
    let mut cols = 0;
    let towers: HashMap<char, Vec<Vec2D>> = {
        let lines = BufReader::new(File::open(INPUT).unwrap()).lines();
        lines
            .enumerate()
            .map(|(ridx, line)| {
                rows += 1;
                cols = 0;
                line.unwrap()
                    .char_indices()
                    .filter_map(|(cidx, val)| {
                        cols += 1;
                        is_antenna(val).then_some((val, Vec2D::from((ridx as i64, cidx as i64))))
                    })
                    .collect_vec()
            })
            .flatten()
            .into_grouping_map()
            .collect()
    };
    let bounding_box = Vec2D(rows, cols);
    let pairs = towers
        .values()
        .map(|towers| {
            towers
                .into_iter()
                .tuple_combinations()
                .map(|(&left, &right)| [2 * right - left, 2 * left - right])
                .flatten()
        })
        .flatten()
        .filter(|candidate| bounding_box.contains(candidate))
        .unique()
        .count();
    println!("The total is {}", pairs);
}
