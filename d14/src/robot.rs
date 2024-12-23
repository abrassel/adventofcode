use derive_more::derive::Constructor;

use crate::{MAZE, vec2d::Vec2D};

#[derive(Copy, Clone, Constructor)]
pub struct Robot {
    pub pos: Vec2D,
    pub(crate) v: Vec2D,
}

impl Robot {
    pub fn step(self, t: isize) -> Vec2D {
        (self.pos + (self.v * t) % MAZE) % MAZE
    }
}
