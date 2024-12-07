use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

use derive_more::derive::From;
use itertools::Itertools;

use crate::{
    INPUT,
    path::{MapEither, Path},
};

#[derive(Clone, Debug)]
pub struct Maze {
    row_major: Vec<Vec<usize>>,
    col_major: Vec<Vec<usize>>,
}

#[derive(Copy, Clone, Eq, Hash, PartialEq, From, Debug)]
pub struct Pos(pub usize, pub usize, pub Direction);

impl Pos {
    pub fn loc(self) -> (usize, usize) {
        (self.0, self.1)
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Direction {
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

    pub fn dir(self) -> (i64, i64) {
        match self {
            Direction::Up => (-1, 0),
            Direction::Left => (0, -1),
            Direction::Down => (1, 0),
            Direction::Right => (0, 1),
        }
    }
}

impl Maze {
    pub fn from_path() -> (Self, Pos) {
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
        (
            Self {
                row_major,
                col_major,
            },
            start.unwrap().into(),
        )
    }

    pub fn guard_path(&self, mut guard_start: Pos) -> Option<Path> {
        let mut segments = vec![guard_start];
        let mut ready_to_break = false;
        let mut seen = HashSet::new();
        while !ready_to_break {
            if seen.contains(&guard_start) {
                return None;
            }
            seen.insert(guard_start);
            guard_start = match self.next_loc(guard_start) {
                Ok(next_guard) => next_guard,
                Err(next_guard) => {
                    ready_to_break = true;
                    next_guard
                }
            };
            segments.push(guard_start);
        }
        Some(segments.into())
    }

    // compute the next path segment, noting if there are no more with an Err varient
    fn next_loc(&self, mut guard_start: Pos) -> Result<Pos, Pos> {
        // project along the search plane
        let (search_slice, guard_loc, search_len) = match guard_start.2 {
            Direction::Up | Direction::Down => (
                &self.col_major[guard_start.1],
                &mut guard_start.0,
                self.row_major.len(),
            ),
            Direction::Left | Direction::Right => (
                &self.row_major[guard_start.0],
                &mut guard_start.1,
                self.col_major.len(),
            ),
        };
        let new_slice_loc: Result<usize, usize> = try {
            // binary search to find the next obstacle
            let next_obstacle_idx = {
                let rounded_up = search_slice.binary_search(&guard_loc).unwrap_err();
                // unfortunately, we may "overestimate" if we're coming from the positive side of the vector
                let adjustment = match guard_start.2 {
                    Direction::Up | Direction::Left => -1,
                    _ => 0,
                };
                usize::try_from(rounded_up as i64 + adjustment).map_err(|_| 0usize)?
            };

            // get the actual location of the obstacle, handling overflow -> going off the side of the map
            let obstacle_loc = *search_slice.get(next_obstacle_idx).ok_or(search_len)?;

            // we need to place ourselves on the "near" side of the obstacle.
            let adjustment = match guard_start.2 {
                Direction::Up | Direction::Left => 1,
                Direction::Right | Direction::Down => -1,
            };
            (obstacle_loc as i64 + adjustment) as usize
        };
        match new_slice_loc {
            Ok(end) | Err(end) => {
                *guard_loc = end;
                guard_start.2 = guard_start.2.rotate();
            }
        };
        new_slice_loc.map_either(|_| guard_start)
    }

    pub fn insert(&mut self, new_barrier: (usize, usize)) {
        let loc = self.col_major[new_barrier.1]
            .binary_search(&new_barrier.0)
            .unwrap_err();
        self.col_major[new_barrier.1].insert(loc, new_barrier.0);
        let loc = self.row_major[new_barrier.0]
            .binary_search(&new_barrier.1)
            .unwrap_err();
        self.row_major[new_barrier.0].insert(loc, new_barrier.1);
    }

    pub fn print_with_path(&self, path: &Path) {
        let mut entire = vec![vec!['.'; self.col_major.len()]; self.row_major.len()];
        for (ridx, row) in self.row_major.iter().enumerate() {
            for cidx in row {
                entire[ridx][*cidx] = '#';
            }
        }

        for (row, col) in path.iter() {
            entire[row][col] = 'O';
        }

        for row in entire {
            println!("{}", row.iter().join(""));
        }
        println!();
    }

    pub fn print(&self) {
        let mut entire = vec![vec!['.'; self.col_major.len()]; self.row_major.len()];
        for (ridx, row) in self.row_major.iter().enumerate() {
            for cidx in row {
                entire[ridx][*cidx] = '#';
            }
        }

        for row in entire {
            println!("{}", row.iter().join(""));
        }
        println!();
    }
}
