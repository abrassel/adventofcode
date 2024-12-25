use derive_more::derive;

use crate::point::Point;

#[derive(Copy, Clone, derive::Display)]
pub enum Move {
    #[display("<")]
    Left,
    #[display("^")]
    Up,
    #[display("v")]
    Down,
    #[display(">")]
    Right,
}

impl From<char> for Move {
    fn from(value: char) -> Self {
        match value {
            '>' => Self::Right,
            'v' => Self::Down,
            '<' => Self::Left,
            '^' => Self::Up,
            _ => unreachable!(),
        }
    }
}

impl Move {
    pub fn new_pos(self, pt: Point) -> Point {
        let delta = match self {
            Move::Left => (0, -1),
            Move::Up => (-1, 0),
            Move::Down => (1, 0),
            Move::Right => (0, 1),
        };
        Point(
            pt.0.saturating_add_signed(delta.0),
            pt.1.saturating_add_signed(delta.1),
        )
    }
}
