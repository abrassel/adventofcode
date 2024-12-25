#![feature(impl_trait_in_assoc_type)]

use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

mod maze;
mod point;
pub use maze::Maze;

pub fn read_input(path: impl AsRef<Path>) -> Maze {
    let lines = BufReader::new(File::open(path).unwrap())
        .lines()
        .map(|line| line.unwrap());
    lines.collect()
}
