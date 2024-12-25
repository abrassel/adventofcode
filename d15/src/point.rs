#[derive(
    derive_more::From,
    PartialEq,
    Eq,
    Hash,
    Debug,
    derive_more::Constructor,
    Copy,
    Clone,
    derive_more::Mul,
)]
pub struct Point(pub usize, pub usize);
