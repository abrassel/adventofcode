use d19::read_input;

const PATH: &str = "d19/input/input.txt";

fn main() {
    let (mut search_engine, searches) = read_input(PATH);
    let count: usize = searches
        .map(|search| search_engine.solve_count(&search))
        .sum();
    println!("The answer is {}", count);
}
