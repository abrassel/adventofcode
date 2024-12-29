use d23::{
    find_3_cliques,
    input::{make_graph, read_input},
};

const PATH: &str = "d23/input/input.txt";

fn main() {
    let input = read_input(PATH);
    let graph = make_graph(&input);
    let count = find_3_cliques(graph);
    println!("Answer: {}", count);
}
