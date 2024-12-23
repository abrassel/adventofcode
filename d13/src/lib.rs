use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use derive_more::Constructor;
use itertools::Itertools;
use regex::Regex;
use vec2d::Vec2D;

pub mod vec2d;

pub const INPUT: &str = "d13/input/input.txt";
pub const A_COST: NumSize = 3;
pub const B_COST: NumSize = 1;
pub const OFFSET: NumSize = 10000000000000;

pub type NumSize = u128;

#[derive(Debug, Constructor)]
pub struct Problem {
    pub a: Vec2D,
    pub b: Vec2D,
    pub v: Vec2D,
}

pub fn read_input() -> Vec<Problem> {
    let regex = Regex::new(r"\w[+=](\d+)").unwrap();
    BufReader::new(File::open(INPUT).unwrap())
        .lines()
        .enumerate()
        .filter_map(|(offset, value)| (offset % 4 != 3).then_some(value))
        .map(|line| {
            let line = line.unwrap();
            regex
                .captures_iter(&line)
                .map(|c| c.get(1).unwrap().as_str().parse().unwrap())
                .collect()
        })
        .tuple_windows()
        .map(|(a, b, v)| Problem::new(a, b, v + Vec2D(OFFSET, OFFSET)))
        .collect()
}
