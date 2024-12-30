pub mod input;
pub mod trie;

#[derive(derive_more::Constructor, Debug)]
pub struct Key {
    tumblers: Vec<usize>,
    height: usize,
}

impl Key {
    pub fn max_lock(&self) -> Vec<usize> {
        self.tumblers
            .iter()
            .map(|tumbler| self.height - tumbler)
            .collect()
    }
}
