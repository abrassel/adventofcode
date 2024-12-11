use std::{
    fs::File,
    io::{BufReader, Read},
};

const INPUT: &str = "d9p1/input/input.txt";

#[derive(Copy, Clone, Debug)]
struct Space {
    id: usize,
    offset: u32,
    size: u32,
}

impl Space {
    pub fn new(idx: usize, offset: u32, size: u32) -> Self {
        Self {
            id: idx / 2,
            offset,
            size,
        }
    }

    pub fn checksum(&self) -> u128 {
        self.id as u128 * self.score()
    }

    fn score(&self) -> u128 {
        if self.size == 0 {
            return 0;
        }
        (self.size * (self.size + 2 * self.offset - 1) / 2) as u128
    }
}

fn read_input() -> anyhow::Result<Vec<Space>> {
    let res = {
        let mut buf = String::new();
        BufReader::new(File::open(INPUT)?).read_to_string(&mut buf)?;
        buf
    };
    let mut offset = 0;
    res.chars()
        .enumerate()
        .map(|(idx, c)| {
            let size = c
                .to_digit(10)
                .ok_or_else(|| anyhow::anyhow!("tried to read {}", c))?;
            offset += size;
            Ok(Space::new(idx, offset - size, size))
        })
        .collect()
}

fn main() -> anyhow::Result<()> {
    let nums = read_input()?;
    let (mut reals, mut spaces): (Vec<_>, Vec<_>) = nums
        .into_iter()
        .enumerate()
        .partition(|(idx, _)| idx % 2 == 0);

    // try to compact everything before scoring
    for (_, real) in reals.iter_mut().rev() {
        for (_, space) in spaces.iter_mut() {
            if space.offset >= real.offset {
                break;
            }
            if space.size >= real.size {
                real.offset = space.offset;
                space.offset += real.size;
                space.size -= real.size;
                break;
            }
        }
    }

    let tot_checksum: u128 = reals.into_iter().map(|(_, space)| space.checksum()).sum();

    println!("\nThe answer is {}", tot_checksum);

    Ok(())
}
