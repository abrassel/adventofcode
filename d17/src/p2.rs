use d17::{ProgramState, read_input};
use itertools::iproduct;

const PATH: &str = "d17/input/input.txt";

fn main() {
    let program: ProgramState = read_input(PATH).collect();
    let goal_state: Vec<_> = program.instructions.iter().map(|x| *x as u64).collect();

    // decompiled input program:
    // A % 8 -> B
    // .....
    // (use top 10 digits of A) -> B
    // ...
    // print B % 8
    // A // 2^3 -> A
    // repeat unless A == 0

    // in the above program, only the bottom A octant influences the output
    // so we read octants off of A from right to left.
    // the only caveat is that there is a line where top digits from A get used.

    // therefore, if we start with the biggest A octant,
    // we can test against the last program instruction,
    // since the top digits are all 0 at that point.

    // i figured this out during testing, but after fixing the top digits
    // they will influence the next block's choice.
    // this is relevant because there are sometimes multiple answers.
    // to this end, need a search / pruning strategy.

    // seed with 0, and then expand with all possible answers.

    // work backward from the final output
    let all_answers = goal_state.into_iter().rev().fold(
        // the frontier starts with no options (just the seed)
        vec![0],
        |so_far, goal| {
            // figure out which octant options, when appended to `so_far`
            // meet the current octant goal
            iproduct!(so_far, 0b0..=0b111)
                .map(|(acc, octant)| (acc << 3) | octant)
                .filter(|frontier| {
                    // seed program with new register
                    let mut program = ProgramState {
                        a: *frontier,
                        ..program.clone()
                    };
                    program.run();
                    // check that the we have added a new output earlier in the program
                    program.output[0] == goal
                })
                .collect()
        },
    );

    // when we build the frontier, we build it from high to low,
    // and we try the lowest value first, always.
    // therefore, the first element in the vector is the lowest solution
    let answer = all_answers[0];
    println!("The answer is {}", answer);
}
