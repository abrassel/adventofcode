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

    let mut enabled = true;
    re.captures_iter(&buf)
        .filter_map(|capture| match capture.get(0).unwrap().as_str() {
            "do()" => {
                enabled = true;
                None
            }
            "don't()" => {
                enabled = false;
                None
            }
            _ => enabled.then(|| {
                capture
                    .name("left")
                    .unwrap()
                    .as_str()
                    .parse::<u64>()
                    .unwrap()
                    * capture
                        .name("right")
                        .unwrap()
                        .as_str()
                        .parse::<u64>()
                        .unwrap()
            }),
        })
        .sum()
}

fn main() {
    let tot = parse_input();
    println!("Answer is {}", tot);
}
