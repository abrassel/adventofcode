#![feature(try_blocks)]

use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

use instruction::{Instruction, InstructionKind};

mod instruction;
mod operand;

#[derive(Default, Clone)]
pub struct ProgramState {
    pub a: u64,
    pub b: u64,
    pub c: u64,
    pub instructions: Vec<u8>,
    pub loc: usize,
    pub output: Vec<u64>,
    pub jumped_flag: bool,
}

impl ProgramState {
    pub fn run(&mut self) {
        while self.loc < self.instructions.len() {
            let instruction: InstructionKind = self.instructions[self.loc].into();
            let operand = self.instructions[self.loc + 1];
            instruction.eval(self, operand);
            if self.jumped_flag {
                self.jumped_flag = false;
            } else {
                self.loc += 2;
            }
        }
    }

    pub fn print_output(&self) {
        let formatted = self
            .output
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(",");
        println!("{}", formatted);
    }
}

fn try_parse_register(line: String) -> Option<u64> {
    line.split_once(": ").unwrap().1.parse().ok()
}

fn try_parse_program(line: String) -> Option<Vec<u8>> {
    Some(
        line.split_once(": ")
            .unwrap()
            .1
            .split(",")
            .map(|x| x.parse::<u8>().unwrap())
            .collect(),
    )
}

impl FromIterator<String> for ProgramState {
    fn from_iter<T: IntoIterator<Item = String>>(iter: T) -> Self {
        let mut iter = iter.into_iter();
        let res: Option<Self> = try {
            let a = try_parse_register(iter.next()?)?;
            let b = try_parse_register(iter.next()?)?;
            let c = try_parse_register(iter.next()?)?;
            iter.next().unwrap();
            let instructions = try_parse_program(iter.next()?)?;
            ProgramState {
                a,
                b,
                c,
                instructions,
                ..Default::default()
            }
        };
        res.unwrap()
    }
}

pub fn read_input(path: impl AsRef<Path>) -> impl Iterator<Item = String> {
    BufReader::new(File::open(path).unwrap())
        .lines()
        .map(|line| line.unwrap())
}
