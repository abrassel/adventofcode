use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

use crate::{Move, Warehouse};

pub fn read_input<ObjectKind: Copy + From<char>>(
    path: impl AsRef<Path>,
) -> (Warehouse<ObjectKind>, Vec<Move>) {
    let mut lines = BufReader::new(File::open(path).unwrap())
        .lines()
        .map(|line| line.unwrap());
    let maze = (&mut lines).take_while(|line| !line.is_empty()).collect();
    let moves = lines
        .flat_map(|line| line.chars().map(Move::from).collect::<Vec<_>>())
        .collect();
    (maze, moves)
}
