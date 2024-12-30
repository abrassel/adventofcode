use std::{collections::HashMap, fmt::Debug};

#[derive(Debug)]
pub struct Trie {
    root: Box<Node>,
}

#[derive(Clone)]
struct Node {
    children: Vec<Option<Box<Node>>>,
    count: usize,
}

impl Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Node")
            .field(
                "children",
                &self
                    .children
                    .iter()
                    .enumerate()
                    .flat_map(|(i, child)| child.as_ref().map(|child| (i, child)))
                    .collect::<HashMap<_, _>>(),
            )
            .field("count", &self.count)
            .finish()
    }
}

impl Trie {
    pub fn new() -> Self {
        Self {
            root: Box::new(Node::new()),
        }
    }

    pub fn insert(&mut self, lock: Vec<usize>) {
        let mut cur = &mut self.root;
        for tumbler in lock {
            cur = cur.children[tumbler].get_or_insert_default();
        }
        cur.count += 1;
    }

    /// any lock where all heights are <= this cutoff will do
    pub fn less_than(&self, lock: &[usize]) -> usize {
        // recursively prune away trie
        let mut opts = vec![&self.root];
        for tumbler in lock {
            opts = opts
                .into_iter()
                .flat_map(|opt| {
                    // grow the search frontier by all lock heights less than equal cutoff height
                    let cutoff = &opt.children[..=*tumbler];
                    cutoff
                })
                .flat_map(|x| x)
                .collect();
        }
        opts.into_iter().map(|opt| opt.count).sum()
    }
}

impl Node {
    pub fn new() -> Self {
        Self {
            children: vec![None; 10],
            count: 0,
        }
    }
}
impl Default for Node {
    fn default() -> Self {
        Self::new()
    }
}
