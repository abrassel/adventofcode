use itertools::Itertools;
use regex::{self, Regex};
use std::{
    cmp::Ordering,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};
use vec2d::Vec2D;

use robot::Robot;

pub mod robot;
pub mod vec2d;

pub(crate) type NumSize = isize;
// pub const ROW: NumSize = 7;
// pub const COL: NumSize = 11;
pub const ROW: NumSize = 103;
pub const COL: NumSize = 101;
const MAZE: Vec2D = Vec2D(ROW, COL);

pub fn from_input(path: impl AsRef<Path>) -> Vec<Robot> {
    let re = Regex::new(r"-?\d+,-?\d+").unwrap();
    let input = BufReader::new(File::open(path).unwrap()).lines();
    input
        .map(|line| {
            let line = line.unwrap();
            let res: Vec<_> = re
                .find_iter(&line)
                .map(|m| {
                    let (lhs, rhs) = m.as_str().split_once(',').unwrap();
                    Vec2D(rhs.parse().unwrap(), lhs.parse().unwrap())
                })
                .collect();
            Robot {
                pos: res[0],
                v: res[1],
            }
        })
        .collect()
}

pub fn quadrant(pos: Vec2D) -> Option<usize> {
    Some(match (pos.0.cmp(&(ROW / 2)), pos.1.cmp(&(COL / 2))) {
        (Ordering::Less, Ordering::Less) => 0,
        (Ordering::Greater, Ordering::Less) => 1,
        (Ordering::Less, Ordering::Greater) => 2,
        (Ordering::Greater, Ordering::Greater) => 3,
        _ => return None,
    })
}

pub trait CountQuadrants: Iterator<Item = Vec2D> {
    fn count_quadrants(self) -> usize
    where
        Self: Sized,
    {
        self.filter_map(quadrant)
            .counts()
            .values()
            .copied()
            .reduce(|a, b| a * b)
            .unwrap()
    }
}

impl<T> CountQuadrants for T where T: Iterator<Item = Vec2D> + ?Sized {}

pub fn display_state(state: &Vec<Vec2D>) {
    let mut output = vec![vec![0; COL as usize]; ROW as usize];

    for state in state {
        output[state.0 as usize][state.1 as usize] += 1;
    }

    for row in output {
        println!(
            "{}",
            row.into_iter()
                .map(|t| if t == 0 {
                    ".".to_owned()
                } else {
                    t.to_string()
                })
                .collect::<String>()
        );
    }
}
