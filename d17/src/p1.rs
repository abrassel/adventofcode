use d17::{ProgramState, read_input};

const PATH: &str = "d17/input/input.txt";

fn main() {
    let mut program: ProgramState = read_input(PATH).collect();
    program.run();
    program.print_output();
}
