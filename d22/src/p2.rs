#![feature(iter_map_windows)]

use d22::read_input;
use itertools::Itertools;
use rayon::prelude::*;
use rustc_hash::{FxHashMap as HashMap, FxHashSet as HashSet};

const PATH: &str = "d23/input/input.txt";
const DAY: usize = 2000;

fn prune(a: u32) -> u32 {
    a % 16777216
}

fn mix(a: u32, b: u32) -> u32 {
    a ^ b
}

fn evolve(mut cur: u32) -> u32 {
    cur = prune(mix(cur << 6, cur));
    cur = prune(mix(cur >> 5, cur));
    cur = prune(mix(cur << 11, cur));
    cur
}

fn day_values(mut seed: u32, day: usize) -> Vec<i32> {
    let seed_ref = &mut seed;
    (0..day)
        .map(|_| {
            let res = *seed_ref % 10;
            *seed_ref = evolve(*seed_ref);
            res as i32
        })
        .collect()
}

fn main() {
    let input = read_input(PATH);
    let buyers = input
        .into_par_iter()
        .map(|seed| day_values(seed, DAY))
        .collect::<Vec<_>>();
    let mut tot_windows = HashMap::default();
    for buyer in buyers {
        let mut seen: HashSet<(i32, i32, i32, i32)> = HashSet::default();
        for (a, b, c, d, e) in buyer.into_iter().tuple_windows() {
            let diffs = (e - d, d - c, c - b, b - a);
            if seen.insert(diffs) {
                *tot_windows.entry(diffs).or_insert(0i32) += e;
            }
        }
    }
    let best = tot_windows.values().max().unwrap();
    println!("The best is {}", best);
}
