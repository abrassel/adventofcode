use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

use crate::{Key, trie::Trie};

pub fn read_input(path: impl AsRef<Path>) -> (Trie, Vec<Key>) {
    let mut lines = BufReader::new(File::open(path).unwrap())
        .lines()
        .map(|line| line.unwrap());

    // extract all locks and keys
    let mut keys = vec![];
    let mut locks = Trie::new();
    loop {
        let next_block: Vec<_> = (&mut lines).take_while(|x| !x.is_empty()).collect();
        if next_block.is_empty() {
            break;
        }
        let height = next_block.len();
        let is_key = next_block[0].starts_with(".");
        let mut tumblers = vec![0; next_block[0].len()];
        for row in next_block {
            for (col, c) in row.char_indices() {
                if c == '#' {
                    tumblers[col] += 1;
                }
            }
        }
        if is_key {
            keys.push(Key::new(tumblers, height));
        } else {
            locks.insert(tumblers);
        }
    }
    (locks, keys)
}
