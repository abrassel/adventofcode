use d16::read_input;

const PATH: &str = "d16/input/input.txt";

fn main() {
    let maze = read_input(PATH);
    let path_len = maze.djikstra_all_paths();
    println!("The score is: {}", path_len);
}
