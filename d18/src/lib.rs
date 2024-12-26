#![feature(try_blocks)]

use std::{
    fs::File,
    io::{BufRead, BufReader},
    ops::{Add, Index, IndexMut},
    path::Path,
};

use rustc_hash::FxHashSet as HashSet;

pub struct Maze {
    maze: Vec<Vec<Cell>>,
}

pub fn read_input(path: impl AsRef<Path>) -> impl Iterator<Item = Point> {
    let lines = BufReader::new(File::open(path).unwrap())
        .lines()
        .map(|line| line.unwrap());
    lines.map(Point::from)
}

#[derive(Copy, Clone, derive_more::Display)]
pub enum Cell {
    #[display(".")]
    Open,
    #[display("#")]
    Closed,
}

#[derive(Clone, Copy, Eq, PartialEq, Hash, derive_more::Display)]
#[display("{},{}", _1, _0)]
pub struct Point(pub usize, pub usize);

impl From<String> for Point {
    fn from(value: String) -> Self {
        let (x, y) = value.split_once(",").unwrap();
        Self(y.parse().unwrap(), x.parse().unwrap())
    }
}

impl Add<(isize, isize)> for Point {
    type Output = Option<Self>;

    fn add(self, rhs: (isize, isize)) -> Self::Output {
        Some(Point(
            self.0.checked_add_signed(rhs.0)?,
            self.1.checked_add_signed(rhs.1)?,
        ))
    }
}

pub trait Bfs {
    fn neighbor(&self, loc: Point) -> impl Iterator<Item = Point>;

    fn bfs(&self, start: Point, end: Point) -> Option<usize> {
        let mut frontier = vec![start];
        let mut level = 0;
        let mut visited = HashSet::default();
        while !frontier.is_empty() {
            let mut new_frontier = vec![];
            for cur in frontier {
                if cur == end {
                    return Some(level);
                }
                if visited.contains(&cur) {
                    continue;
                }
                visited.insert(cur);

                // expand frontier
                new_frontier.extend(self.neighbor(cur));
            }
            frontier = new_frontier;
            level += 1;
        }
        None
    }

    fn print_with_path(&self, _path: Vec<Point>) {}
}

impl Bfs for Maze {
    fn neighbor(&self, loc: Point) -> impl Iterator<Item = Point> {
        const DIRS: [(isize, isize); 4] = [(0, -1), (0, 1), (-1, 0), (1, 0)];
        DIRS.into_iter()
            .filter_map(move |dir| loc + dir)
            .filter(|&nbor| matches!(self.get(nbor), Some(Cell::Open)))
    }

    fn print_with_path(&self, path: Vec<Point>) {
        for (ridx, row) in self.maze.iter().enumerate() {
            for (cidx, cell) in row.iter().enumerate() {
                if path.contains(&Point(ridx, cidx)) {
                    print!("O");
                } else {
                    print!("{}", cell);
                }
            }
            println!();
        }
    }
}

impl Index<Point> for Maze {
    type Output = Cell;

    fn index(&self, index: Point) -> &Self::Output {
        &self.maze[index.0][index.1]
    }
}

impl IndexMut<Point> for Maze {
    fn index_mut(&mut self, index: Point) -> &mut Self::Output {
        &mut self.maze[index.0][index.1]
    }
}

impl Maze {
    pub fn get(&self, loc: Point) -> Option<Cell> {
        self.maze.get(loc.0)?.get(loc.1).copied()
    }

    pub fn new(dims: Point) -> Self {
        Self {
            maze: vec![vec![Cell::Open; dims.1]; dims.0],
        }
    }

    pub fn place_byte(&mut self, pt: Point) {
        self[pt] = Cell::Closed
    }
}
