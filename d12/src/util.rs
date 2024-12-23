pub const DIRS: [(isize, isize); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

pub fn step(cur: (usize, usize), dir: (isize, isize)) -> Option<(usize, usize)> {
    Some((
        cur.0.checked_add_signed(dir.0)?,
        cur.1.checked_add_signed(dir.1)?,
    ))
}

pub fn neighbors(loc: (usize, usize)) -> impl Iterator<Item = (usize, usize)> {
    DIRS.into_iter().filter_map(move |dir| step(loc, dir))
}
