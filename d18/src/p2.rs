use d18::{Bfs, Maze, Point, read_input};

const PATH: &str = "d18/input/input.txt";
const DIM: Point = Point(71, 71);
const START: Point = Point(0, 0);
const END: Point = Point(70, 70);

fn main() {
    let bytes = read_input(PATH);
    let mut maze = Maze::new(DIM);
    for byte in bytes {
        maze.place_byte(byte);
        if maze.bfs(START, END).is_none() {
            println!("The answer is {}", byte);
            break;
        }
    }
}
