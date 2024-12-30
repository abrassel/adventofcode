use itertools::Itertools;
use petgraph::{algo::toposort, prelude::DiGraphMap};
use rustc_hash::FxHashMap as HashMap;

pub mod input;

#[derive(Clone, Copy, Debug, derive_more::Display, PartialEq, Eq)]
pub enum GateType {
    Xor,
    And,
    Or,
}

impl From<&str> for GateType {
    fn from(value: &str) -> Self {
        match value {
            "XOR" => Self::Xor,
            "AND" => Self::And,
            "OR" => Self::Or,
            _ => panic!(),
        }
    }
}

impl GateType {
    pub fn eval(&self, in1: &str, in2: &str, state: &HashMap<&str, bool>) -> bool {
        let in1_val = state[in1];
        let in2_val = state[in2];
        match self {
            GateType::Xor => in1_val ^ in2_val,
            GateType::And => in1_val & in2_val,
            GateType::Or => in1_val | in2_val,
        }
    }
}

pub fn drive_circuit<'a>(
    mut state: HashMap<&'a str, bool>,
    graph: &DiGraphMap<&'a str, GateType>,
) -> HashMap<&'a str, bool> {
    let nodes = toposort(&graph, None).unwrap();

    // visit circuit in order of resolution
    // nodes appear in the order that they depend on gate transitions
    for node in nodes {
        // we will be passed in init states, too.
        if state.contains_key(node) {
            continue;
        }

        // get gate inputs
        let ((in1, _, _), (in2, _, gate)) = graph
            .edges_directed(node, petgraph::Direction::Incoming)
            .collect_tuple()
            .unwrap();

        // update state with newly evaluated circuit
        state.insert(node, gate.eval(in1, in2, &state));
    }

    state
}
