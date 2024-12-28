use std::ops::Add;

use derive_more::Constructor;
use rustc_hash::{FxHashMap as HashMap, FxHashSet as HashSet};

#[derive(Clone, Copy, derive_more::Display)]
pub enum Cell {
    #[display(".")]
    Open,
    #[display("#")]
    Closed,
}

impl From<char> for Cell {
    fn from(value: char) -> Self {
        match value {
            '.' | 'S' | 'E' => Self::Open,
            '#' => Self::Closed,
            _ => unreachable!(),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, derive_more::From, Debug)]
pub struct Point(pub usize, pub usize);

impl Point {
    pub fn neighbors(self) -> impl Iterator<Item = Self> {
        const DIRS: [(isize, isize); 4] = [(0, -1), (0, 1), (1, 0), (-1, 0)];
        DIRS.into_iter().filter_map(move |dir| self + dir)
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

#[derive(Constructor)]

pub struct Maze {
    layout: Vec<Vec<Cell>>,
}

impl Maze {
    /// Filter is a function that takes `to`, `from`, and `depth`, and decides whether to visit `to`
    pub fn bfs<F>(&self, start: Point, mut filter: F) -> HashMap<Point, usize>
    where
        F: FnMut(Point, Point, usize) -> bool,
    {
        let mut level: Vec<Point> = vec![start];
        let mut depth = 0;
        let mut map = HashMap::default();
        let mut visited = HashSet::default();
        while !level.is_empty() {
            let mut new_level = vec![];
            for from in level {
                if visited.contains(&from) {
                    continue;
                }
                visited.insert(from);

                map.insert(from, depth);
                let neighbors = from.neighbors().filter(|&to| filter(to, from, depth + 1));
                new_level.extend(neighbors);
            }
            level = new_level;
            depth += 1;
        }

        map
    }

    pub fn get(&self, pt: Point) -> Option<Cell> {
        self.layout.get(pt.0)?.get(pt.1).copied()
    }

    fn iter_internal(&self) -> impl Iterator<Item = impl Iterator<Item = (Point, Cell)>> {
        self.layout.iter().enumerate().map(|(ridx, row)| {
            row.iter()
                .enumerate()
                .map(move |(cidx, cell)| (Point(ridx, cidx), *cell))
        })
    }

    pub fn iter(&self) -> impl Iterator<Item = (Point, Cell)> {
        self.iter_internal().flatten()
    }

    pub fn print_with_bfs_res(&self, bfs_res: &HashMap<Point, usize>) {
        for row in self.iter_internal() {
            for (point, cell) in row {
                match bfs_res.get(&point) {
                    Some(dist) => print!("{}", dist % 10),
                    None => print!("{}", cell),
                }
            }
            println!();
        }
    }
}
