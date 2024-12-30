use d25::input::read_input;

const PATH: &str = "d25/input/input.txt";

fn main() {
    let (locks, keys) = read_input(PATH);

    let ans: usize = keys
        .iter()
        .map(|key| locks.less_than(&key.max_lock()))
        .sum();
    println!("The answer is {}", ans);
}
