use d23::read_input;
use rayon::prelude::*;

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

fn solve(seed: u32, day: usize) -> u32 {
    let mut cur = seed;
    for _ in 0..day {
        cur = evolve(cur);
    }
    cur
}

fn main() {
    let input = read_input(PATH);
    let res = input
        .into_par_iter()
        .map(|seed| solve(seed, DAY) as u128)
        .sum::<u128>();
    println!("The answer is {}", res);
}
