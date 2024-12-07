#![feature(try_blocks)]

use itertools::Itertools;
use maze::Maze;
use rayon::prelude::*;

mod maze;
mod path;

pub const INPUT: &str = "d6p1/input/input.txt";

fn main() {
    let (maze, guard_start) = Maze::from_path();
    let standard_guard_path = maze.guard_path(guard_start).unwrap();

    let cycle_count = standard_guard_path
        .iter()
        .iter()
        .unique()
        .collect_vec()
        .par_iter()
        .filter(|start_loc| {
            let mut new_maze = maze.clone();
            new_maze.insert(***start_loc);
            new_maze.guard_path(guard_start).is_none()
        })
        .count();
    println!("cycle count: {}", cycle_count);
}
