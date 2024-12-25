use std::ops::Add;
use strum::VariantArray;

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct Point(pub usize, pub usize);

#[derive(PartialEq, Eq, derive_more::Display)]
pub enum Cell {
    #[display(".")]
    Open,
    #[display("#")]
    Wall,
}

impl From<char> for Cell {
    fn from(value: char) -> Self {
        match value {
            'S' | 'E' | '.' => Self::Open,
            '#' => Self::Wall,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, Hash, PartialEq, VariantArray, derive_more::Display)]
pub enum Dir {
    #[display(">")]
    East,
    #[display("v")]
    South,
    #[display("<")]
    West,
    #[display("^")]
    North,
}

impl Dir {
    pub fn turn_cw(self) -> Self {
        let nxt = (self.position() + 1) % Self::VARIANTS.len();
        Self::VARIANTS[nxt]
    }

    pub fn turn_ccw(self) -> Self {
        let nxt = self
            .position()
            .checked_sub(1)
            .unwrap_or(Self::VARIANTS.len() - 1);
        Self::VARIANTS[nxt]
    }

    pub fn step(self, pt: Point) -> Option<Point> {
        let dir = match self {
            Dir::East => (0, 1),
            Dir::South => (1, 0),
            Dir::West => (0, -1),
            Dir::North => (-1, 0),
        };
        pt + dir
    }

    fn position(self) -> usize {
        Self::VARIANTS.iter().position(|&x| x == self).unwrap()
    }
}

impl Add<(isize, isize)> for Point {
    type Output = Option<Point>;

    fn add(self, rhs: (isize, isize)) -> Self::Output {
        let Self(x, y) = self;
        Some(Self(
            x.checked_add_signed(rhs.0)?,
            y.checked_add_signed(rhs.1)?,
        ))
    }
}
