use d20::{Setup, cheats::Cheats, read_input};

const PATH: &str = "d20/input/input.txt";
const MAX_PHANTOM_DISTANCE: usize = 20;

fn main() {
    let Setup { maze, start, end } = read_input(PATH);
    let cheat_scores = maze.cheat_scores(start, end, MAX_PHANTOM_DISTANCE);
    let score = cheat_scores
        .into_iter()
        .filter(|(_, score)| *score >= 100)
        .count();
    println!("The number of such cheats is {}", score);
}
