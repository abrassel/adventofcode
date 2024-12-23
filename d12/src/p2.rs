#![feature(file_buffered)]
#![feature(try_blocks)]
// #![feature(mixed_integer_ops_unsigned_sub)]

use maze::Maze;

mod maze;
mod perimeter;
mod util;

pub const INPUT: &str = "d12/input/input.txt";

fn main() {
    let maze = Maze::new(INPUT);
    let regions = maze.union_find_with_perimeter();
    let answer: usize = regions
        .into_iter()
        .filter(|x| x.label() != '-')
        .map(|region| {
            // println!("Region: {}, count: {:?}", region.label(), region.corners());
            region.len() * region.corners()
        })
        .sum();
    println!("The answer is {answer}");
}
