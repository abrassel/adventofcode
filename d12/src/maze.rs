use std::{collections::HashSet, fs::File, io::BufRead, path::Path};

use crate::{perimeter::Region, util};

fn add_border(region: &mut Vec<Vec<char>>) {
    // add a boundary of "-"
    for row in region.iter_mut() {
        row.insert(0, '-');
        row.push('-');
    }
    let horizontal_border = vec!['-'; region[0].len()];
    region.insert(0, horizontal_border.clone());
    region.push(horizontal_border);
}

pub struct Maze(Vec<Vec<char>>);

impl Maze {
    pub fn new(path: impl AsRef<Path>) -> Self {
        let maze = File::open_buffered(path).unwrap().lines();
        let mut region: Vec<Vec<char>> = maze
            .into_iter()
            .map(|line| line.unwrap().chars().collect())
            .collect();
        add_border(&mut region);
        Self(region)
    }

    fn tiles(&self) -> impl Iterator<Item = ((usize, usize), char)> {
        self.0.iter().enumerate().flat_map(|(ridx, row)| {
            row.iter()
                .enumerate()
                .map(move |(cidx, &c)| ((ridx, cidx), c))
        })
    }

    fn neighbors(&self, loc: (usize, usize)) -> impl Iterator<Item = ((usize, usize), char)> {
        util::neighbors(loc).filter_map(|new: (usize, usize)| {
            let plant = self.0.get(new.0)?.get(new.1)?;
            Some((new, *plant))
        })
    }

    pub fn union_find_with_perimeter(&self) -> Vec<Region> {
        let mut visited = HashSet::new();
        let mut regions = vec![];

        for (loc, tile) in self.tiles() {
            if visited.contains(&loc) {
                continue;
            }
            let mut to_visit = vec![loc];
            let mut region = HashSet::new();
            while let Some(loc) = to_visit.pop() {
                if visited.contains(&loc) {
                    continue;
                }
                region.insert(loc);
                visited.insert(loc);
                for (nloc, ntile) in self.neighbors(loc) {
                    if ntile == tile {
                        to_visit.push(nloc);
                    }
                }
            }
            regions.push(Region::new(tile, region));
        }
        regions
    }
}
