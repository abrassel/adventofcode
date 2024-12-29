use d21::{int_value, read_input, score};

const PATH: &str = "d21/input/input.txt";
const ROBOTS: usize = 2;

fn main() {
    let input = read_input(PATH);
    let complexities = input.map(|code| int_value(&code) * score(code, ROBOTS));
    let total: u128 = complexities.sum();
    println!("The answer is {}", total);
}
