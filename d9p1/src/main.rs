use std::{
    fs::File,
    io::{BufReader, Read},
};

const INPUT: &str = "d9p1/input/input.txt";

fn read_input() -> anyhow::Result<Vec<u32>> {
    let res = {
        let mut buf = String::new();
        BufReader::new(File::open(INPUT)?).read_to_string(&mut buf)?;
        buf
    };
    res.chars()
        .map(|c| {
            c.to_digit(10)
                .ok_or_else(|| anyhow::anyhow!("tried to read {}", c))
        })
        .collect()
}

fn score(start: u32, len: u32) -> u128 {
    (len * (len + 2 * start - 1) / 2) as u128
}

fn main() -> anyhow::Result<()> {
    let mut nums = read_input()?;

    let mut lidx = 0;
    let mut ridx = nums.len() - 1;
    let mut checksum: u128 = 0;
    let mut offset = 0;

    while lidx <= ridx {
        let id = (lidx / 2) as u128;
        // phase 1: file
        let file_size = nums[lidx];
        checksum += id * score(offset, file_size);
        offset += file_size;
        lidx += 1;

        // phase 2: free space
        match nums.get(lidx) {
            Some(&free_space) => {
                let mut free_space = free_space;
                lidx += 1;
                // empty from back until free space is empty
                while ridx > lidx && free_space > 0 {
                    let id = ridx as u128 / 2;
                    let size = nums[ridx];
                    free_space = match free_space.checked_sub(size) {
                        Some(rem) => {
                            ridx -= 2;
                            checksum += id * score(offset, size);
                            offset += size;
                            rem
                        }
                        None => {
                            nums[ridx] = size - free_space;
                            checksum += id * score(offset, free_space);
                            offset += free_space;
                            0
                        }
                    }
                }
            }
            None => {
                panic!("uh oh");
            }
        }
    }

    println!("\nThe answer is {}", checksum);

    Ok(())
}
