#![feature(try_blocks)]

mod input;

use std::{collections::HashMap, u128, usize};

type Memo = HashMap<(Vec<char>, usize), u128>;
type Paths = HashMap<(char, char), Vec<Vec<char>>>;

pub use input::read_input;
use itertools::{Itertools, iproduct};

const DIR_PAD: [[char; 3]; 2] = [[' ', '^', 'A'], ['<', 'v', '>']];
const NUM_PAD: [[char; 3]; 4] = [['7', '8', '9'], ['4', '5', '6'], ['1', '2', '3'], [
    ' ', '0', 'A',
]];

fn all_points<Matrix, Row>(pad: Matrix) -> Vec<(char, (usize, usize))>
where
    Matrix: AsRef<[Row]>,
    Row: AsRef<[char]>,
{
    let mut map = vec![];
    for (ridx, row) in pad.as_ref().into_iter().enumerate() {
        for (cidx, col) in row.as_ref().into_iter().enumerate() {
            if *col != ' ' {
                map.push((*col, (ridx, cidx)))
            }
        }
    }
    map
}

fn to_dirs(path: Vec<(usize, usize)>) -> Vec<char> {
    path.into_iter()
        .tuple_windows()
        .map(|(from, to)| {
            if from.0 < to.0 {
                'v'
            } else if from.0 > to.0 {
                '^'
            } else if from.1 < to.1 {
                '>'
            } else if from.1 > to.1 {
                '<'
            } else {
                unreachable!()
            }
        })
        .chain(vec!['A'])
        .collect()
}

fn shortest_paths<Matrix, Row>(
    left: (usize, usize),
    right: (usize, usize),
    pad: Matrix,
) -> Vec<Vec<char>>
where
    Matrix: AsRef<[Row]>,
    Row: AsRef<[char]>,
{
    let mut paths = vec![];
    let mut to_visit = vec![(left, vec![])];
    let mut final_loop = false;
    while !final_loop {
        let mut next_level = vec![];
        for (cur, mut path) in to_visit {
            if path.contains(&cur) {
                continue;
            }
            path.push(cur);
            if cur == right {
                paths.push(to_dirs(path));
                final_loop = true;
            } else {
                let dirs = [(0, 1), (0, -1), (1, 0), (-1, 0)];
                for dir in dirs {
                    let next = try {
                        let desired = (
                            cur.0.checked_add_signed(dir.0)?,
                            cur.1.checked_add_signed(dir.1)?,
                        );
                        let val = pad.as_ref().get(desired.0)?.as_ref().get(desired.1)?;
                        (desired, *val)
                    };
                    if let Some((next, val)) = next {
                        if val != ' ' {
                            next_level.push((next, path.clone()));
                        }
                    }
                }
            }
        }
        to_visit = next_level;
    }
    paths
}

fn all_paths<Matrix, Row>(pad: Matrix) -> Paths
where
    Matrix: AsRef<[Row]> + Copy,
    Row: AsRef<[char]>,
{
    // compute all pairwise paths
    let points = all_points(pad);
    iproduct!(&points, &points)
        .map(|((lcar, left), (rcar, right))| ((*lcar, *rcar), shortest_paths(*left, *right, pad)))
        .collect()
}

fn score_unmemod(
    code: Vec<char>,
    robots: usize,
    cur_paths: &Paths,
    dir_paths: &Paths,
    memo: &mut Memo,
) -> u128 {
    let code = [&['A'], code.as_slice()].concat();
    let mut tot = 0;
    for (x, y) in code.into_iter().tuple_windows() {
        let transition = (x, y);
        let paths = cur_paths[&transition].clone();
        if robots == 0 {
            tot += paths.into_iter().map(|path| path.len()).min().unwrap() as u128
        } else {
            tot += paths
                .into_iter()
                .map(|path| score_rec(path, robots - 1, dir_paths, dir_paths, memo))
                .min()
                .unwrap();
        }
    }
    tot
}

pub fn score_rec(
    code: Vec<char>,
    robots: usize,
    cur_paths: &Paths,
    dir_paths: &Paths,
    memo: &mut Memo,
) -> u128 {
    let key = (code, robots);
    if let Some(res) = memo.get(&key) {
        return *res;
    }
    let score = score_unmemod(key.0.clone(), robots, cur_paths, dir_paths, memo);
    memo.insert(key, score);
    score
}

pub fn score(code: Vec<char>, robots: usize) -> u128 {
    let mut memo = Memo::default();
    let dir_paths = all_paths(DIR_PAD);
    let num_paths = all_paths(NUM_PAD);
    score_rec(code, robots, &num_paths, &dir_paths, &mut memo)
}

pub fn int_value(code: &[char]) -> u128 {
    let num_portion = &code[..code.len() - 1];
    let num_str: String = num_portion.iter().collect();
    num_str.parse().unwrap()
}
