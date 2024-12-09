use std::ops::{Mul, Sub};

use derive_more::From;

#[derive(From, Copy, Clone, Eq, Hash, PartialEq, Debug)]
pub struct Vec2D(pub i64, pub i64);

impl Sub for Vec2D {
    type Output = Vec2D;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl Mul<Vec2D> for i64 {
    type Output = Vec2D;

    fn mul(self, rhs: Vec2D) -> Self::Output {
        Vec2D(self * rhs.0, self * rhs.1)
    }
}

impl Vec2D {
    pub fn contains(&self, other: &Vec2D) -> bool {
        (0..self.0).contains(&other.0) && (0..self.1).contains(&other.1)
    }
}
