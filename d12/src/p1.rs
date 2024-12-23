#![feature(file_buffered)]

use std::{collections::HashSet, fs::File, io::BufRead};

const INPUT: &str = "d12/input/input.txt";
const DIRS: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

fn neighbors(
    region: &[Vec<char>],
    loc: (usize, usize),
) -> impl Iterator<Item = ((usize, usize), char)> {
    DIRS.into_iter().filter_map(move |dir| {
        let new = (
            loc.0.checked_add_signed(dir.0)?,
            loc.1.checked_add_signed(dir.1)?,
        );
        let plant = region.get(new.0)?.get(new.1)?;
        Some((new, *plant))
    })
}

fn main() {
    let maze = File::open_buffered(INPUT).unwrap().lines();
    let mut region: Vec<Vec<char>> = maze
        .into_iter()
        .map(|line| line.unwrap().chars().collect())
        .collect();

    // add a boundary of "-"
    for row in &mut region {
        row.insert(0, '-');
        row.push('-');
    }
    let horizontal_border = vec!['-'; region[0].len()];
    region.insert(0, horizontal_border.clone());
    region.push(horizontal_border);

    let mut visited: HashSet<(usize, usize)> = HashSet::with_capacity(region.len().pow(2));
    let mut fence_cost = 0;
    for (ridx, row) in region.iter().enumerate() {
        for (cidx, &plant) in row.into_iter().enumerate() {
            if plant == '-' {
                continue;
            }

            // init BFS
            let mut area = 0;
            let mut perimeter = 0;
            let mut to_visit = vec![(ridx, cidx)];
            while let Some(loc) = to_visit.pop() {
                if visited.contains(&loc) {
                    continue;
                }
                area += 1;
                visited.insert(loc);
                for (nloc, nplant) in neighbors(&region, loc) {
                    if nplant == plant {
                        to_visit.push(nloc);
                    } else {
                        perimeter += 1;
                    }
                }
            }
            fence_cost += area * perimeter;
        }
    }
    println!("The fence cost is: {fence_cost}");
}
