use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

use petgraph::prelude::*;

pub fn read_input(path: impl AsRef<Path>) -> Vec<String> {
    BufReader::new(File::open(path).unwrap())
        .lines()
        .map(|line| line.unwrap())
        .collect()
}

pub fn make_graph_backing(edges: &[String]) -> Vec<(&str, &str)> {
    edges
        .into_iter()
        .map(|x| x.split_once("-").unwrap())
        .collect()
}

pub fn make_graph<'a>(edges: &'a [String]) -> UnGraphMap<&'a str, ()> {
    let edges = make_graph_backing(edges);
    edges.into_iter().collect()
}
