#![feature(try_blocks)]

use std::{
    fs::File,
    io::{BufRead, BufReader},
};

mod oracle;

use itertools::Itertools;

pub const INPUT: &str = "d6p1/input/input.txt";

#[derive(Clone, Copy, Debug)]
enum Direction {
    Up,
    Left,
    Down,
    Right,
}

impl TryFrom<char> for Direction {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        use Direction::*;
        Ok(match value {
            '^' => Up,
            '>' => Right,
            '<' => Left,
            'v' => Down,
            _ => return Err(anyhow::anyhow!("invalid dir")),
        })
    }
}

impl Direction {
    fn rotate(self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Left => Direction::Up,
            Direction::Down => Direction::Left,
            Direction::Right => Direction::Down,
        }
    }
}

struct Maze {
    row_major: Vec<Vec<usize>>,
    col_major: Vec<Vec<usize>>,
    pos: (usize, usize, Direction),
}

impl Maze {
    pub fn from_file() -> Self {
        let lines = BufReader::new(File::open(INPUT).unwrap()).lines();
        let mut col_len = None;
        let mut start = None;
        let row_major: Vec<Vec<usize>> = lines
            .enumerate()
            .map(|(ridx, line)| {
                let line = line.unwrap();
                col_len = Some(line.len());
                line.char_indices()
                    .filter(|&(i, c)| {
                        if let Ok(dir) = Direction::try_from(c) {
                            start = Some((ridx, i, dir))
                        }
                        c == '#'
                    })
                    .map(|(i, _)| i)
                    .collect()
            })
            .collect();
        let mut col_major = vec![vec![]; col_len.unwrap()];
        for (ridx, row) in row_major.iter().enumerate() {
            for &cidx in row {
                col_major[cidx].push(ridx);
            }
        }
        Self {
            row_major,
            col_major,
            pos: start.unwrap(),
        }
    }

    /// move `cur` and return length of move
    pub fn next_loc(&mut self) -> Result<usize, usize> {
        let (search_slice, loc, uncompressed_len) = match self.pos.2 {
            Direction::Up | Direction::Down => (
                &self.col_major[self.pos.1],
                &mut self.pos.0,
                self.row_major.len(),
            ),
            Direction::Left | Direction::Right => (
                &self.row_major[self.pos.0],
                &mut self.pos.1,
                self.col_major.len(),
            ),
        };
        let next_obstacle_idx = search_slice.binary_search(&loc).unwrap_err();
        // have to round down if hitting obstacle from pos direction
        let barrier_round_dir: i64 = match self.pos.2 {
            Direction::Up | Direction::Left => -1,
            Direction::Right | Direction::Down => 0,
        };
        let moveme_round_dir: i64 = match self.pos.2 {
            Direction::Up | Direction::Left => 1,
            Direction::Down | Direction::Right => -1,
        };
        // handle underflow
        match usize::try_from(next_obstacle_idx as i64 + barrier_round_dir) {
            Ok(next_idx) => {
                match search_slice.get(next_idx) {
                    Some(&next_barrier) => {
                        // valid case - update loc and then return distance traveled
                        let dist = (next_barrier as i64 - (*loc as i64)).abs() as usize - 1;
                        // move loc to be the opposite dir of traveled.
                        *loc = (next_barrier as i64 + moveme_round_dir) as usize;
                        self.pos.2 = self.pos.2.rotate();
                        Ok(dist)
                    }
                    // overflow goes out the top of the vector
                    None => Err(uncompressed_len - *loc - 1),
                }
            }
            // underflow goes out the beginning of the vector
            Err(_) => Err(*loc - 1),
        }
    }
}

fn main() {
    oracle::do_problem();

    // non-oracle solution is broken because it does not consider uniqueness.
    let mut maze = Maze::from_file();
    let count: usize = std::iter::repeat_with(|| maze.next_loc())
        .take_while_inclusive(Result::is_ok)
        .map(|res| match res {
            Ok(val) | Err(val) => val,
        })
        .sum();
    println!("The total distance traveled is: {}", count + 1);
}
