use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

pub fn read_input(path: impl AsRef<Path>) -> impl Iterator<Item = Vec<char>> {
    let lines = BufReader::new(File::open(path).unwrap()).lines();
    lines.map(|line| line.unwrap().chars().collect())
}
