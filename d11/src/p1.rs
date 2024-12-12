use d11::RockSet;

const ITERS: usize = 25;
fn main() {
    let mut init_state = RockSet::read_input();
    for _ in 0..ITERS {
        init_state = init_state.evolve();
    }
    println!("The answer is {}", init_state.len());
}
