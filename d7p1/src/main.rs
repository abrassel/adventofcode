#![feature(iterator_try_collect)]

use rayon::prelude::*;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use itertools::Itertools;

const INPUT: &str = "d7p1/input/input.txt";

struct Calibration {
    total: u128,
    nums: Vec<u128>,
}

impl TryFrom<&str> for Calibration {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let (total, nums) = value.split_once(": ").ok_or(anyhow::anyhow!("foo"))?;
        Ok(Self {
            total: total.parse()?,
            nums: nums
                .split(" ")
                .map(|num| num.parse::<u128>())
                .collect::<Result<Vec<_>, _>>()?,
        })
    }
}

const SYMBOLS: [fn(u128, u128) -> u128; 2] = [|x: u128, y: u128| x + y, |x: u128, y: u128| x * y];

impl Calibration {
    pub fn validate(&self) -> bool {
        self.validate_rec(None, 0)
    }

    fn validate_rec(&self, acc: Option<u128>, offset: usize) -> bool {
        if offset == self.nums.len() {
            return acc == Some(self.total);
        }

        return SYMBOLS.into_iter().any(|op| match acc {
            Some(acc) => self.validate_rec(Some(op(acc, self.nums[offset])), offset + 1),
            None => self.validate_rec(Some(self.nums[offset]), offset + 1),
        });
    }
}

fn main() {
    let total: u128 = BufReader::new(File::open(INPUT).unwrap())
        .lines()
        .map(Result::unwrap)
        .collect_vec()
        .par_iter()
        .map(|line| Calibration::try_from(line.as_ref()).unwrap())
        .filter(Calibration::validate)
        .map(|calibration| calibration.total)
        .sum();

    println!("The total is: {:?}", total);
}
