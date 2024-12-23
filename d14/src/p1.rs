use d14::CountQuadrants;
use d14::from_input;

const INPUT: &str = "d14/input/input.txt";
const STEPS: isize = 100;

fn main() {
    let mut state = vec![];
    let robots = from_input(INPUT);
    let quad_counts = robots
        .into_iter()
        .map(|robot| robot.step(STEPS))
        .inspect(|loc| {
            state.push(*loc);
        })
        .count_quadrants();

    // display_state(&state);

    println!("The answer is {}", quad_counts);
}
