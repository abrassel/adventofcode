use derive_more::derive::From;

use crate::vec::Vec2D;

#[derive(From, Debug, Clone, Copy)]
pub struct Line {
    m: Vec2D,
    b: Vec2D,
}

impl Line {
    pub fn through(from: Vec2D, to: Vec2D) -> Self {
        Self {
            m: (to - from).unitize().0,
            b: from,
        }
    }

    pub fn apply(&self, t: i64) -> Vec2D {
        self.b + self.m * t
    }

    pub fn antinodes(self, bounds: Vec2D) -> impl Iterator<Item = Vec2D> {
        fn move_dir(
            slf: Line,
            stepper: impl Iterator<Item = i64>,
            bounds: Vec2D,
        ) -> impl Iterator<Item = Vec2D> {
            stepper
                .map(move |t| slf.apply(t))
                .take_while(move |pt| bounds.contains(pt))
        }

        move_dir(self, 0.., bounds).chain(move_dir(self, (1..).map(|i| -i), bounds))
    }
}
