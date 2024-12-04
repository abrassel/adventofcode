use std::{
    fs::File,
    io::{BufRead, BufReader},
};

const INPUT: &str = "d4p1/input/input.txt";

const WORD: &str = "XMAS";

const DIRS: [(i32, i32); 8] = [
    (1, 1),   // diagonal down-right
    (1, -1),  // diagonal down-left
    (1, 0),   // vertical down
    (-1, 0),  // vertical up
    (-1, -1), // diagonal up-left
    (-1, 1),  // diagonal up-right
    (0, -1),  // horizontal left
    (0, 1),   // horizontal right
];

struct WordSearch(Vec<Vec<char>>);

impl WordSearch {
    pub fn from_input() -> Self {
        Self(
            BufReader::new(File::open(INPUT).unwrap())
                .lines()
                .map(|line| line.unwrap().chars().collect())
                .collect(),
        )
    }

    pub fn find_xs(&self) -> impl Iterator<Item = (usize, usize)> + use<'_> {
        self.0.iter().enumerate().flat_map(|(ridx, row)| {
            row.iter()
                .enumerate()
                .filter_map(move |(cidx, &val)| (val == 'X').then_some((ridx, cidx)))
        })
    }

    pub fn search_dir(&self, start: (usize, usize), dir: (i32, i32)) -> bool {
        let mut cur = (start.0 as i32, start.1 as i32);
        WORD.chars().all(|char| {
            let idx = match (usize::try_from(cur.0), usize::try_from(cur.1 as i32)) {
                (Ok(ridx), Ok(cidx)) => (ridx, cidx),
                _ => {
                    return false;
                }
            };

            let was_match = self
                .0
                .get(idx.0)
                .and_then(|row| row.get(idx.1))
                .map_or(false, |&val| val == char);
            cur = (cur.0 + dir.0, cur.1 + dir.1);
            was_match
        })
    }
}

fn main() {
    let board = WordSearch::from_input();
    let xs = board.find_xs();
    let count: usize = xs
        .map(|loc| {
            DIRS.into_iter()
                .filter(|&dir| board.search_dir(loc, dir))
                .count()
        })
        .sum();
    println!("The count: {}", count)
}
