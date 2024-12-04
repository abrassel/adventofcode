use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

use itertools::Itertools;

const INPUT: &str = "d4p1/input/input.txt";

struct WordSearch(pub Vec<Vec<char>>);

impl WordSearch {
    pub fn from_input() -> Self {
        Self(
            BufReader::new(File::open(INPUT).unwrap())
                .lines()
                .map(|line| line.unwrap().chars().collect())
                .collect(),
        )
    }

    pub fn is_word(&self, start: (usize, usize), dir: (i32, i32)) -> Option<(usize, usize)> {
        let mut loc = (start.0 as i32, start.1 as i32);
        let letters = (0..3)
            .map_while(|_| match (usize::try_from(loc.0), usize::try_from(loc.1)) {
                (Ok(rloc), Ok(cloc)) => {
                    loc.0 += dir.0;
                    loc.1 += dir.1;
                    self.0.get(rloc).and_then(|row| row.get(cloc).cloned())
                }
                _ => None,
            })
            .collect_vec();
        (&letters == &['M', 'A', 'S'] || &letters == &['S', 'A', 'M']).then_some((
            (start.0 as i32 + dir.0) as usize,
            (start.1 as i32 + dir.1) as usize,
        ))
    }
}

fn main() {
    let board = WordSearch::from_input();

    // get a list of valid "a" locations by diagonal, horizontal, vertical, and anti-diagonal
    let all_coords = (0..board.0.len()).cartesian_product(0..board.0[0].len());

    let pairs = [((1, 1), (1, -1))];

    let count: usize = pairs
        .into_iter()
        .map(|(left, right)| {
            let left_hits: HashSet<_> = all_coords
                .clone()
                .filter_map(|start| board.is_word(start, left))
                .collect();
            let right_hits: HashSet<_> = all_coords
                .clone()
                .filter_map(|start| board.is_word(start, right))
                .collect();

            left_hits.intersection(&right_hits).count()
        })
        .sum();

    println!("The count: {}", count)
}
