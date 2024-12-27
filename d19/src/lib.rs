use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
    str::FromStr,
};

use ascii::{AsciiStr, AsciiString};
use rustc_hash::{FxHashMap as HashMap, FxHashSet as HashSet};

#[derive(Debug, Default)]
pub struct SearchEngine {
    memo: HashMap<AsciiString, usize>,
    tokens: HashSet<AsciiString>,
}

impl SearchEngine {
    pub fn new(base: Vec<AsciiString>) -> Self {
        Self {
            tokens: base.into_iter().collect(),
            ..Default::default()
        }
    }

    pub fn solve_count(&mut self, lookup: &AsciiStr) -> usize {
        if let Some(memoed) = self.memo.get(lookup) {
            return *memoed;
        }

        // this is a dp problem - build substring, and recurse when we find a fit
        // never consider entire string. that will get caught in the base case above.
        // if we considered it here, then we would have an infinite loop.
        let mut total_solves = if self.tokens.contains(lookup) { 1 } else { 0 };
        for (before, after) in (1..lookup.len()).map(|split| (&lookup[..split], &lookup[split..])) {
            if self.tokens.contains(before) {
                total_solves += self.solve_count(after);
            }
        }
        self.memo.insert(lookup.to_owned(), total_solves);
        total_solves
    }
}

pub fn read_input(path: impl AsRef<Path>) -> (SearchEngine, impl Iterator<Item = AsciiString>) {
    let mut lines = BufReader::new(File::open(path).unwrap())
        .lines()
        .map(|line| line.unwrap());
    let towels = lines.next().unwrap();
    let towels = {
        towels
            .split(", ")
            .map(|towel| AsciiString::from_str(towel).unwrap())
    };
    let search_engine = SearchEngine::new(towels.collect());

    lines.next().unwrap();

    let inputs = lines.map(|line| AsciiString::from_str(&line).unwrap());
    (search_engine, inputs)
}
