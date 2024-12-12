use std::{
    collections::HashMap,
    fs::File,
    io::{BufReader, Read},
};

use itertools::Itertools;
use rayon::prelude::*;

#[memoize::memoize]
pub fn evolve(rock: u128) -> Vec<u128> {
    if rock == 0 {
        return vec![1];
    }

    let digit_len = rock.ilog10();
    if digit_len % 2 == 1 {
        let half = 10u128.pow((digit_len + 1) / 2);
        let low = rock / half;
        let high = rock - low * half;
        return vec![low, high];
    }

    return vec![rock * 2024];
}

pub struct RockSet(HashMap<u128, usize>);

const INPUT: &str = "d11/input/input.txt";

impl RockSet {
    pub fn read_input() -> Self {
        let mut buf = String::new();
        BufReader::new(File::open(INPUT).unwrap())
            .read_to_string(&mut buf)
            .unwrap();
        Self(
            buf.split_ascii_whitespace()
                .map(|x| x.parse().unwrap())
                .counts(),
        )
    }

    pub fn evolve(self) -> Self {
        let new_rocks = self
            .0
            .into_par_iter()
            .flat_map_iter(|(rock, count)| {
                let rocks = evolve(rock);

                rocks.into_iter().map(move |rock| (rock, count))
            })
            .collect_vec_list()
            .into_iter()
            .flatten()
            .into_grouping_map()
            .sum();
        Self(new_rocks)
    }

    pub fn len(&self) -> usize {
        self.0.values().sum()
    }
}
