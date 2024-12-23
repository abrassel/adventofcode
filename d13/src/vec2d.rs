use derive_more::derive::{Add, Constructor, Div, Mul, Sub};

use crate::{A_COST, B_COST, NumSize};

#[derive(Clone, Copy, Eq, PartialEq, Constructor, Sub, Mul, Div, Debug, Add)]
#[div(forward)]
pub struct Vec2D(pub NumSize, pub NumSize);

#[derive(Clone, Copy)]
pub struct DivRem {
    pub div: Vec2D,
    pub rem: Vec2D,
}

impl DivRem {
    pub fn valid(self) -> bool {
        self.rem == Vec2D(0, 0) && self.div.diagonal()
    }
}

impl Vec2D {
    pub fn div_rem(self, other: Self) -> DivRem {
        DivRem {
            div: Self(self.0 / other.0, self.1 / other.1),
            rem: Self(self.0 % other.0, self.1 % other.1),
        }
    }

    pub fn lte(self, other: Self) -> bool {
        self.0 <= other.0 && self.1 <= other.1
    }

    pub fn cost(self) -> NumSize {
        self.0 * A_COST + self.1 * B_COST
    }

    pub fn diagonal(self) -> bool {
        self.0 == self.1
    }
}

impl FromIterator<NumSize> for Vec2D {
    fn from_iter<T: IntoIterator<Item = NumSize>>(iter: T) -> Self {
        let mut iter = iter.into_iter();
        Self(iter.next().unwrap(), iter.next().unwrap())
    }
}
