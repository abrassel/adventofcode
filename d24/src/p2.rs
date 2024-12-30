#![feature(try_blocks)]

use std::{
    fs::OpenOptions,
    io::{BufWriter, Write},
};

use anyhow::Result;
use d24::{GateType, input::read_input};
use itertools::Itertools;
use lazy_static::lazy_static;
use petgraph::{dot::Dot, prelude::*};
use rustc_hash::{FxHashMap as HashMap, FxHashSet as HashSet};

const PATH: &str = "d24/input/input.txt";
lazy_static! {
    static ref SWAPS: HashMap<&'static str, &'static str> = {
        let mut map = HashMap::default();
        map.insert("z05", "dkr");
        map.insert("dkr", "z05");

        map.insert("z15", "htp");
        map.insert("htp", "z15");

        map.insert("z20", "hhh");
        map.insert("hhh", "z20");

        map.insert("rhv", "ggk");
        map.insert("ggk", "rhv");

        map
    };
}

type CircuitGraph<'a> = DiGraphMap<&'a str, GateType>;

fn key_num(key: impl AsRef<str>) -> Option<usize> {
    key.as_ref().strip_prefix(&['x', 'y', 'z'])?.parse().ok()
}

fn repair_swaps(expected_out: Vec<(String, GateType, String)>) -> Vec<(String, GateType, String)> {
    expected_out
        .into_iter()
        .map(|(from, gate, to)| {
            let to = do_swap(to);
            (from, gate, to)
        })
        .collect()
}

fn do_swap(from: String) -> String {
    match SWAPS.get(from.as_str()) {
        Some(corrected) => {
            println!("Correcting {} to {}", from, corrected);
            (*corrected).to_owned()
        }
        None => from,
    }
}

fn main() {
    let (state, expected_out) = read_input(PATH);
    let max_bits: usize = state.keys().filter_map(key_num).max().unwrap();
    let bit_len = max_bits.ilog10() as usize + 1; // 10 -> 1, but has two digits

    // updated `expected_out` with the node swaps
    let expected_out = repair_swaps(expected_out);

    let graph: CircuitGraph = DiGraphMap::from_edges(
        expected_out
            .iter()
            .map(|(from, gate, to)| (from.as_str(), to.as_str(), gate)),
    );

    let viz = Dot::new(&graph);
    let out = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open("d24/input/viz")
        .unwrap();
    let mut writer = BufWriter::new(&out);
    writeln!(writer, "{}", viz).unwrap();

    let key = |prefix, key| format!("{prefix}{:0>bit_len$}", key);
    let find_gate = |lin: &str, rin: &str, gate_type| -> Result<String> {
        let lin_candidates = graph
            .edges_directed(lin, petgraph::Direction::Outgoing)
            .filter(|(_, _, gate)| **gate == gate_type)
            .map(|(_, to, _)| to)
            .collect::<HashSet<_>>();
        let rin_candidates = graph
            .edges_directed(rin, petgraph::Direction::Outgoing)
            .filter(|(_, _, gate)| **gate == gate_type)
            .map(|(_, to, _)| to)
            .collect::<HashSet<_>>();
        lin_candidates
            .intersection(&rin_candidates)
            .next()
            .map(|res| (*res).to_owned())
            .ok_or(anyhow::anyhow!(
                "missing {} for {}, {}",
                gate_type,
                lin,
                rin
            ))
    };

    // starting from lowest  bits and working up, verify each adder structurally
    let mut carry_in: Option<String> = None;
    for bit in 0..=max_bits {
        let carry_out: Result<String> = try {
            let x_key = key('x', bit);
            let y_key = key('y', bit);

            let xor = find_gate(&x_key, &y_key, GateType::Xor)?;
            let z_key = match &carry_in {
                Some(carry_in) => find_gate(&xor, &carry_in, GateType::Xor)?,
                None => xor.clone(),
            };
            let actual_z_bit =
                key_num(&z_key).ok_or(anyhow::anyhow!("output {} not a z_bit", &z_key))?;
            if actual_z_bit != bit {
                println!("Found error at bit {}, swapped with {}", bit, actual_z_bit);
                return;
            }

            // now compute carry
            let and = find_gate(&x_key, &y_key, GateType::And)?;
            let carry_out = match &carry_in {
                Some(carry_in) => {
                    let carry_and = find_gate(&carry_in, &xor, GateType::And)?;
                    find_gate(&carry_and, &and, GateType::Or)?
                }
                None => and,
            };
            carry_out
        };
        carry_in = match carry_out {
            Ok(carry_out) => Some(carry_out),
            Err(failure) => {
                println!("Error in full adder {} {}", bit, failure);
                return;
            }
        };
    }
    println!("No errors detected");
    let ans = SWAPS.keys().sorted().join(",");
    println!("The answer is {}", ans);
}
