#![feature(try_blocks)]

use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

pub struct Graph(Vec<Vec<u32>>);

const INPUT: &str = "d10/input/input.txt";
const DIRS: [(isize, isize); 4] = [(-1, 0), (0, 1), (0, -1), (1, 0)];

impl Graph {
    pub fn read_input() -> Self {
        let lines = BufReader::new(File::open(INPUT).unwrap()).lines();
        let map = lines
            .map(|line| {
                line.unwrap()
                    .chars()
                    .map(|c| c.to_digit(10).unwrap())
                    .collect()
            })
            .collect();
        Self(map)
    }

    pub fn end_counts(&self, start: (usize, usize)) -> usize {
        let mut nines = HashSet::new();
        self.bfs_generic(start, |loc| {
            nines.insert(loc);
        });
        nines.len()
    }

    pub fn path_counts(&self, start: (usize, usize)) -> usize {
        let mut count = 0;
        self.bfs_generic(start, |_| {
            count += 1;
        });
        count
    }

    pub fn find(&self, target: u32) -> impl Iterator<Item = (usize, usize)> {
        self.0
            .iter()
            .enumerate()
            .map(move |(ridx, row)| {
                row.iter()
                    .enumerate()
                    .filter_map(move |(cidx, &val)| (target == val).then_some((ridx, cidx)))
            })
            .flatten()
    }

    fn at(&self, loc: (usize, usize)) -> u32 {
        self.0[loc.0][loc.1]
    }

    fn neighbors(&self, loc: (usize, usize)) -> Vec<(usize, usize)> {
        DIRS.iter()
            .filter_map(|dir| {
                Some((
                    loc.0.checked_add_signed(dir.0)?,
                    loc.1.checked_add_signed(dir.1)?,
                ))
            })
            .filter(|&(x, y)| {
                let cur = self.at(loc);
                let new: Option<u32> = try { *self.0.get(x)?.get(y)? };
                new.is_some_and(|new| new == cur + 1)
            })
            .collect()
    }

    fn bfs_generic(&self, start: (usize, usize), mut hit_fn: impl FnMut((usize, usize))) {
        let mut to_visit = vec![(start, 0)];

        while let Some((cur, len)) = to_visit.pop() {
            if self.at(cur) == 9 {
                hit_fn(cur)
            }
            to_visit.extend(self.neighbors(cur).into_iter().map(|cur| (cur, len + 1)));
        }
    }
}
