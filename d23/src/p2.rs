use d23::{
    input::{make_graph, read_input},
    maximal_clique,
};
use itertools::Itertools;

const PATH: &str = "d23/input/input.txt";

fn main() {
    let input = read_input(PATH);
    let graph = make_graph(&input);
    let max_clique = maximal_clique(graph);
    let sol_key = max_clique.into_iter().sorted().join(",");
    println!("Answer: {}", sol_key);
}
