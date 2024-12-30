use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

use rustc_hash::FxHashMap as HashMap;

use crate::GateType;

pub fn read_input(
    path: impl AsRef<Path>,
) -> (HashMap<String, bool>, Vec<(String, GateType, String)>) {
    let mut lines = BufReader::new(File::open(path).unwrap())
        .lines()
        .map(|line| line.unwrap());
    let init_state = (&mut lines)
        .take_while(|x| !x.is_empty())
        .map(|line| {
            let (name, state) = line.split_once(": ").unwrap();
            (name.to_owned(), if state == "1" { true } else { false })
        })
        .collect();
    let edges = lines
        .flat_map(|line| {
            // ntg XOR fgs -> mjb
            let (gate, out) = line.split_once(" -> ").unwrap();
            let [in1, gate, in2] = gate
                .split_whitespace()
                .collect::<Vec<_>>()
                .try_into()
                .unwrap();
            [
                (in1.to_owned(), gate.into(), out.to_owned()),
                (in2.to_owned(), gate.into(), out.to_owned()),
            ]
        })
        .collect();

    (init_state, edges)
}
