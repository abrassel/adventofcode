use std::{
    fmt::Display,
    fs::File,
    io::{BufReader, Read},
};

const INPUT: &str = "d9p1/input/test.txt";

#[derive(Copy, Clone, Debug)]
enum FileMeta {
    Free,
    File { id: usize },
}

impl Display for FileMeta {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let letter = match self {
            FileMeta::Free => ".".to_owned(),
            FileMeta::File { id } => id.to_string(),
        };
        write!(f, "{}", letter)
    }
}

#[derive(Copy, Clone, Debug)]
struct Space {
    file_meta: FileMeta,
    offset: u32,
    size: u32,
    is_used: bool,
}

impl Space {
    pub fn new(idx: usize, offset: u32, size: u32) -> Self {
        let file_meta = if idx % 2 == 0 {
            FileMeta::File { id: idx / 2 }
        } else {
            FileMeta::Free
        };
        Self {
            file_meta,
            offset,
            size,
            is_used: false,
        }
    }

    pub fn simulate_move(&mut self, to: &Space) -> Result<&mut Self, ()> {
        if self.is_used || matches!(self.file_meta, FileMeta::Free) {
            return Err(());
        }

        match self.size.cmp(&to.size) {
            std::cmp::Ordering::Less | std::cmp::Ordering::Equal => {
                self.offset = to.offset;
                self.is_used = true;
                Ok(self)
            }
            std::cmp::Ordering::Greater => Err(()),
        }
    }

    pub fn checksum(&self) -> Result<u128, ()> {
        match self.file_meta {
            FileMeta::Free => Err(()),
            FileMeta::File { id: idx } => Ok(idx as u128 * self.score()),
        }
    }

    fn score(&self) -> u128 {
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

impl Display for Space {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self.file_meta {
            FileMeta::Free => ".",
            FileMeta::File { id } => &id.to_string(),
        };
        write!(f, "{}", c.repeat(self.size as usize))
    }
}

fn compact_last<'a>(nums: &'a mut Vec<Space>, free_space: &Space) -> Option<&'a mut Space> {
    nums.into_iter()
        .rev()
        .find_map(|x| x.simulate_move(free_space).ok())
}

fn main() -> anyhow::Result<()> {
    let mut nums = read_input()?;
    // let total_size = nums.iter().map(|x| x.size as usize).sum();
    let mut tot_checksum = 0;
    let mut idx = 0;
    while idx < nums.len() {
        let num = nums[idx];
        if num.is_used {
            idx += 1;
            continue;
        }

        match num.checksum() {
            Ok(checksum) => {
                tot_checksum += checksum;
                idx += 1;
            }
            Err(_) => match compact_last(&mut nums, &num) {
                Some(compacter) => {
                    println!("Trying to compact: {:?} at {:?}", compacter, num);
                    tot_checksum += compacter.checksum().unwrap();
                    let size = compacter.size;
                    nums[idx].size -= size;
                    nums[idx].offset += size;
                    if nums[idx].size == 0 {
                        idx += 1;
                    }
                }
                None => {
                    idx += 1;
                }
            },
        }
    }

    println!("\nThe answer is {}", tot_checksum);

    Ok(())
}

// fn fun_name(total_size: usize, nums: &Vec<Space>) {
//     let mut spaces = vec![".".to_owned(); total_size];
//     for x in nums {
//         for i in 0..x.size {
//             spaces[x.offset as usize + i as usize] = x.file_meta.to_string();
//         }
//     }
//     println!(
//         "{}",
//         String::from_iter(spaces.iter().map(|x| x.to_string()))
//     );
// }
