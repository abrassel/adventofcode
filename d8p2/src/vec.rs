use std::ops::{Mul, Sub};

use derive_more::{
    From,
    derive::{Add, Mul},
};
use memoize::memoize;

#[derive(From, Copy, Clone, Eq, Hash, PartialEq, Debug, Mul, Add)]
#[mul(forward)]
pub struct Vec2D(pub i64, pub i64);

impl Sub for Vec2D {
    type Output = Vec2D;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl Mul<i64> for Vec2D {
    type Output = Self;

    fn mul(self, rhs: i64) -> Self::Output {
        Self(self.0 * rhs, self.1 * rhs)
    }
}

impl Vec2D {
    pub fn unitize(self) -> (Self, i64) {
        let reduction_factor = gcd(self.0, self.1);
        (
            Self(self.0 / reduction_factor, self.1 / reduction_factor),
            reduction_factor,
        )
    }

    pub fn contains(&self, other: &Vec2D) -> bool {
        (0..=self.0).contains(&other.0) && (0..=self.1).contains(&other.1)
    }
}

#[memoize]
fn gcd(a: i64, b: i64) -> i64 {
    num::integer::gcd(a, b)
}
