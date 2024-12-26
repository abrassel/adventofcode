use d18::{Bfs, Maze, Point, read_input};

const PATH: &str = "d18/input/input.txt";
const DIM: Point = Point(71, 71);
const BYTE_LIMIT: usize = 1024;
const START: Point = Point(0, 0);
const END: Point = Point(70, 70);

fn main() {
    let bytes = read_input(PATH);
    let mut maze = Maze::new(DIM);
    for byte in bytes.take(BYTE_LIMIT) {
        maze.place_byte(byte);
    }
    let len = maze.bfs(START, END);
    println!("The answer is {}", len.unwrap());
}
