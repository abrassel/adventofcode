use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

pub fn read_input(path: impl AsRef<Path>) -> Vec<u32> {
    BufReader::new(File::open(path).unwrap())
        .lines()
        .map(|line| line.unwrap().parse().unwrap())
        .collect()
}
