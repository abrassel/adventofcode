pub mod input;

use petgraph::prelude::*;
use rustc_hash::FxHashSet as HashSet;

pub fn find_3_cliques(graph: UnGraphMap<&str, ()>) -> usize {
    // search every vertex for a third leg
    let mut cliques = HashSet::default();
    for first in graph.nodes().filter(|node| node.starts_with("t")) {
        for second in graph.neighbors(first) {
            // avoid infinite loop
            for third in graph.neighbors(second).filter(|third| third != &first) {
                if graph.contains_edge(first, third) {
                    let mut clique = [first, second, third];
                    clique.sort_unstable();
                    cliques.insert(clique);
                }
            }
        }
    }
    cliques.len()
}

/// algorithm BronKerbosch1(R, P, X) is
/// if P and X are both empty then
///     report R as a maximal clique
/// for each vertex v in P do
///     BronKerbosch1(R ⋃ {v}, P ⋂ N(v), X ⋂ N(v))
///     P := P \ {v}
///     X := X ⋃ {v}
pub fn bron_kerbosch<'a>(
    graph: &UnGraphMap<&'a str, ()>,
    acc: HashSet<&'a str>,
    mut maybe: HashSet<&'a str>,
    mut excluded: HashSet<&'a str>,
) -> Vec<HashSet<&'a str>> {
    if maybe.is_empty() && excluded.is_empty() {
        return vec![acc];
    }
    let mut cliques = vec![];
    while let Some(&vertex) = maybe.iter().next() {
        let neighbors = graph.neighbors(vertex).collect::<HashSet<_>>();
        let clique = bron_kerbosch(
            graph,
            &acc | &hash_set(vertex),
            &maybe & &neighbors,
            &excluded & &neighbors,
        );
        cliques.extend(clique);
        maybe.remove(vertex);
        excluded.insert(vertex);
    }
    cliques
}

fn hash_set(singleton: &str) -> HashSet<&str> {
    let mut set = HashSet::default();
    set.insert(singleton);
    set
}

pub fn maximal_clique(graph: UnGraphMap<&str, ()>) -> HashSet<&str> {
    let cliques = bron_kerbosch(
        &graph,
        HashSet::default(),
        graph.nodes().collect(),
        HashSet::default(),
    );
    cliques
        .into_iter()
        .max_by_key(|clique| clique.len())
        .unwrap()
}
