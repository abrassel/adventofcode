use derive_more::derive::Constructor;
use itertools::Itertools;
use std::collections::HashSet;

use crate::util::{DIRS, step};

#[derive(Debug, Constructor)]
pub struct Region(char, HashSet<(usize, usize)>);

impl Region {
    fn tiles(&self) -> impl Iterator<Item = (usize, usize)> {
        self.1.iter().copied()
    }

    pub fn corner_count(&self, loc: (usize, usize)) -> usize {
        let check = |dir| step(loc, dir).map_or(false, |loc| self.1.contains(&loc));
        let convex_corner_count = |dir1, dir2| {
            //  _ v
            // |_|_
            // |_|_|
            check(dir1) && check(dir2) && !check((dir1.0 + dir2.0, dir1.1 + dir2.1))
        };
        let concave_corner_count = |dir1, dir2| {
            //  _
            // |_|_
            // |_|_|
            //x
            !check(dir1) && !check(dir2)
        };

        let res = DIRS
            .into_iter()
            .circular_tuple_windows()
            .map(|(dir1, dir2)| {
                (if concave_corner_count(dir1, dir2) {
                    1
                } else {
                    0
                }) + (if convex_corner_count(dir1, dir2) {
                    1
                } else {
                    0
                })
            })
            .sum();
        // println!("{:?}: {}", loc, res);
        res
    }

    pub fn corners(&self) -> usize {
        self.tiles().map(|tile| self.corner_count(tile)).sum()
    }

    pub fn label(&self) -> char {
        self.0
    }

    pub fn len(&self) -> usize {
        self.1.len()
    }
}
