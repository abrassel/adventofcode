pub mod input;
mod moves;
mod p1_impl;
mod p2_impl;
mod point;
mod warehouse;

pub use moves::Move;
pub use p1_impl::object::ObjectKind as P1ObjectKind;
pub use p2_impl::object_kind::ObjectKind as P2ObjectKind;
pub use point::Point;
pub use warehouse::Warehouse;

pub trait Sol {
    fn push_box(&mut self, box_loc: Point, dir: Move);
}

pub trait ObjectKind {
    fn is_box(&self) -> bool;
    fn is_open(&self) -> bool;
    fn gps(&self, pt: Point) -> usize;
}

pub fn solve<Obj: Copy + ObjectKind + From<char> + std::fmt::Display>(
    mut warehouse: Warehouse<Obj>,
    moves: &[Move],
) where
    Warehouse<Obj>: Sol,
{
    for &r#move in moves {
        // println!("Moving: {}", r#move);
        // println!("{}", warehouse);
        // std::io::stdin().read_line(&mut String::new());
        warehouse.move_robot(r#move);
    }
    let score: usize = warehouse
        .boxes()
        .map(|r#box| warehouse[r#box].gps(r#box))
        .sum();
    println!("The score is {}", score);
}
