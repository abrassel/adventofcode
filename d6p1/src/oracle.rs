use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use itertools::Itertools;

use crate::INPUT;

pub fn do_problem() {
    let mut start = None;
    let mut maze = BufReader::new(File::open(INPUT).unwrap())
        .lines()
        .enumerate()
        .map(|(idx, line)| {
            let line = line.unwrap().chars().collect_vec();
            if let Some((cidx, &char)) = line
                .iter()
                .find_position(|&&x| x == '^' || x == '<' || x == '>' || x == 'v')
            {
                start = Some((idx, cidx, char));
            }
            line
        })
        .collect_vec();
    let (mut rdir, mut cdir, mut dir) = start.unwrap();
    loop {
        maze[rdir][cdir] = 'X';
        let dir_step = match dir {
            '>' => (0, 1),
            '<' => (0, -1),
            '^' => (-1, 0),
            'v' => (1, 0),
            _ => unreachable!(),
        };
        let res: anyhow::Result<()> = try {
            let nrdir = usize::try_from(rdir as i64 + dir_step.0)?;
            let ncdir = usize::try_from(cdir as i64 + dir_step.1)?;
            let val = *maze
                .get(nrdir)
                .ok_or(anyhow::anyhow!("whee"))?
                .get(ncdir)
                .ok_or(anyhow::anyhow!("whee"))?;
            match val {
                '#' => {
                    dir = match dir {
                        '>' => 'v',
                        '<' => '^',
                        '^' => '>',
                        'v' => '<',
                        _ => unreachable!(),
                    };
                }
                _ => {
                    rdir = nrdir;
                    cdir = ncdir;
                }
            }
        };

        if res.is_err() {
            break;
        }
    }

    println!(
        "The total count is (oracle): {}",
        maze.into_iter().flatten().filter(|&x| x == 'X').count()
    )
}
