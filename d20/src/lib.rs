use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

use maze::{Maze, Point};

pub mod maze;

pub struct Setup {
    pub maze: Maze,
    pub start: Point,
    pub end: Point,
}

pub fn read_input(path: impl AsRef<Path>) -> Setup {
    let lines = BufReader::new(File::open(path).unwrap()).lines();
    let mut start = None;
    let mut end = None;
    let layout = lines
        .enumerate()
        .map(|(ridx, line)| {
            line.unwrap()
                .char_indices()
                .inspect(|&(cidx, c)| match c {
                    'S' => {
                        start = Some((ridx, cidx));
                    }
                    'E' => {
                        end = Some((ridx, cidx));
                    }
                    _ => {}
                })
                .map(|(_, c)| c.into())
                .collect()
        })
        .collect();
    Setup {
        maze: Maze::new(layout),
        start: start.unwrap().into(),
        end: end.unwrap().into(),
    }
}
