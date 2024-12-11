use d10::Graph;

fn main() {
    let graph = Graph::read_input();
    let paths: usize = graph.find(0).map(|start| graph.path_counts(start)).sum();

    println!("The path count is {}", paths);
}
