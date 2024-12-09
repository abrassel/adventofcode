use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use itertools::Itertools;
use line::Line;
use vec::Vec2D;

mod line;
mod vec;

const INPUT: &str = "d8p1/input/input.txt";

fn is_antenna(c: char) -> bool {
    c.is_ascii_alphanumeric()
}

fn main() {
    let lines = BufReader::new(File::open(INPUT).unwrap()).lines();
    let area = lines
        .map(|line| line.unwrap().chars().collect_vec())
        .collect_vec();
    let bounds = Vec2D(area.len() as i64 - 1, area[0].len() as i64 - 1);
    let antennas = area
        .into_iter()
        .enumerate()
        .map(|(ridx, row)| {
            row.into_iter().enumerate().filter_map(move |(cidx, val)| {
                is_antenna(val).then_some((val, Vec2D(ridx as i64, cidx as i64)))
            })
        })
        .flatten()
        .into_group_map();
    let unique_antinodes = antennas
        .values()
        .map(|same_freq| {
            same_freq
                .into_iter()
                .tuple_combinations()
                .map(|(&left, &right)| Line::through(left, right).antinodes(bounds))
                .flatten()
        })
        .flatten()
        .unique()
        .count();

    println!("The answer is {}", unique_antinodes);
}
