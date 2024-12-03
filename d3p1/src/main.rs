use regex::Regex;
use std::{fs::File, io::Read};

const FILENAME: &str = "d3p1/input/input.txt";

fn parse_input() -> u64 {
    let buf = {
        let mut buf = String::new();
        File::open(FILENAME)
            .unwrap()
            .read_to_string(&mut buf)
            .unwrap();
        buf
    };
    let re = Regex::new(r"(mul\((?<left>\d{1,3}),(?<right>\d{1,3})\))|do\(\)|don't\(\)").unwrap();
    re.captures_iter(&buf)
        .map(|c| {
            c.name("left").unwrap().as_str().parse().unwrap()
                * c.name("right").unwrap().as_str().parse().unwrap()
        })
        .sum()
}

fn main() {
    let tot = parse_input();
    println!("Answer is {}", tot);
}
