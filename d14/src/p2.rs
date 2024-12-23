use d14::{from_input, robot::Robot};
use itertools::Itertools;

const INPUT: &str = "d14/input/input.txt";

fn main() {
    let robots = from_input(INPUT);
    let minimum_score = find_tree(&robots);

    println!("The answer is {}", minimum_score);
}

fn find_tree(robots: &[Robot]) -> isize {
    for t in 0.. {
        if robots.into_iter().map(|robot| robot.step(t)).all_unique() {
            return t;
        }
    }

    unreachable!()
}
