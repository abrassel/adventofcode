use d24::{GateType, drive_circuit, input::read_input};
use petgraph::prelude::DiGraphMap;

const PATH: &str = "d24/input/input.txt";

fn main() {
    let (init_state, circuits) = read_input(PATH);
    let init_state = init_state.iter().map(|(k, &v)| (k.as_ref(), v)).collect();
    let circuits: DiGraphMap<&str, GateType> = DiGraphMap::from_edges(
        circuits
            .iter()
            .map(|(from, weight, to)| (from.as_ref(), to.as_ref(), weight)),
    );
    let final_state = drive_circuit(init_state, &circuits);

    let count: u64 = final_state
        .into_iter()
        .filter(|(k, _)| k.starts_with("z"))
        .map(|(k, v)| {
            let offset: usize = k[1..].parse().unwrap();
            (v as u64) << offset
        })
        .sum();
    println!("The answer is {}", count);
}
