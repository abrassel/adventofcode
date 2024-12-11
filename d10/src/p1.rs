use d10::Graph;

fn main() {
    let graph = Graph::read_input();
    let tot: usize = graph.find(0).map(|start| graph.end_counts(start)).sum();

    println!("The answer is {}", tot);
}
