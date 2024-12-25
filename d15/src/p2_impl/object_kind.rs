use crate::{Move, Point};

#[derive(Eq, PartialEq, Copy, Clone, derive_more::Display, Debug, Default)]
pub enum ObjectKind {
    #[display("#")]
    Wall,
    #[display(".")]
    #[default]
    Open,
    #[display("[")]
    BoxOpen,
    #[display("]")]
    BoxClose,
}

impl From<char> for ObjectKind {
    fn from(value: char) -> Self {
        match value {
            '#' => Self::Wall,
            '.' | '@' => Self::Open,
            '[' => Self::BoxOpen,
            ']' => Self::BoxClose,
            _ => unreachable!("encountered [{}]", value),
        }
    }
}

impl crate::ObjectKind for ObjectKind {
    fn is_box(&self) -> bool {
        matches!(self, ObjectKind::BoxOpen | ObjectKind::BoxClose)
    }

    fn is_open(&self) -> bool {
        matches!(self, ObjectKind::Open)
    }

    fn gps(&self, pt: Point) -> usize {
        match self {
            ObjectKind::Wall | ObjectKind::BoxClose | ObjectKind::Open => 0,
            ObjectKind::BoxOpen => 100 * pt.0 + 1 * pt.1,
        }
    }
}

impl ObjectKind {
    pub fn other_half(self, pos: Point, dir: Move) -> Option<Point> {
        match dir {
            Move::Left | Move::Right => None,
            Move::Up | Move::Down => Some(match self {
                ObjectKind::BoxOpen => Move::Right.new_pos(pos),
                ObjectKind::BoxClose => Move::Left.new_pos(pos),
                _ => unreachable!(),
            }),
        }
    }
}
