use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
};

use itertools::Itertools;

const INPUT_PATH: &str = "d5p1/input/input.txt";

fn main() {
    let mut input = BufReader::new(File::open(INPUT_PATH).unwrap()).lines();
    let rules: HashMap<usize, HashSet<usize>> = (&mut input)
        // get before|after for each line, stop at the line break
        .map_while(|line| {
            let line = line.unwrap();
            (!line.is_empty()).then(|| {
                let (before, after) = line.split_once("|").unwrap();
                (
                    before.parse::<usize>().unwrap(),
                    after.parse::<usize>().unwrap(),
                )
            })
        })
        // collect into before -> {all after}
        .into_grouping_map()
        .collect();

    let total: usize = input
        .map(|line| {
            line.unwrap()
                .split(",")
                .map(|num| num.parse().unwrap())
                .collect_vec()
        })
        // keep only those lines where no `num` has _any_ `after` value occurring before it.
        .filter(|line| {
            let mut seen = HashSet::new();
            line.into_iter().all(|&num| {
                // check for all constraints for the current value
                let no_violations = rules
                    .get(&num)
                    .map_or(true, |afters| afters.intersection(&seen).count() == 0);
                // update the "befores" list with this number
                seen.insert(num);
                no_violations
            })
        })
        .map(|valid_line| valid_line[valid_line.len() / 2])
        .sum();
    println!("Total: {}", total);
}
