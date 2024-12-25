#[derive(Eq, PartialEq, Copy, Clone, derive_more::Display, Debug)]
pub enum ObjectKind {
    #[display("#")]
    Wall,
    #[display(".")]
    Open,
    #[display("O")]
    Box,
}

impl From<char> for ObjectKind {
    fn from(value: char) -> Self {
        match value {
            '#' => Self::Wall,
            '.' | '@' => Self::Open,
            'O' => Self::Box,
            _ => unreachable!("encountered [{}]", value),
        }
    }
}

impl crate::ObjectKind for ObjectKind {
    fn is_box(&self) -> bool {
        matches!(self, ObjectKind::Box)
    }

    fn is_open(&self) -> bool {
        matches!(self, ObjectKind::Open)
    }

    fn gps(&self, pt: crate::Point) -> usize {
        pt.0 * 100 + pt.1
    }
}
