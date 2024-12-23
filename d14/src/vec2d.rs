use std::ops::Rem;

use derive_more::derive::{Add, Constructor, Div, Mul, Sub};

use crate::NumSize;

#[derive(Clone, Copy, Eq, PartialEq, Constructor, Sub, Mul, Div, Debug, Add, Hash)]
#[div(forward)]
pub struct Vec2D(pub NumSize, pub NumSize);

impl FromIterator<NumSize> for Vec2D {
    fn from_iter<T: IntoIterator<Item = NumSize>>(iter: T) -> Self {
        let mut iter = iter.into_iter();
        Self(iter.next().unwrap(), iter.next().unwrap())
    }
}

fn pos_mod(x: isize, m: isize) -> isize {
    assert!(m > 0);
    if x > 0 { x % m } else { (x + m) % m }
}

impl Rem<Vec2D> for Vec2D {
    type Output = Self;

    fn rem(self, rhs: Vec2D) -> Self::Output {
        Self(pos_mod(self.0, rhs.0), pos_mod(self.1, rhs.1))
    }
}
