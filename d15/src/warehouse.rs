use crate::{Move, ObjectKind, Sol, point::Point};
use std::ops::{Index, IndexMut};

#[derive(Debug)]
pub struct Warehouse<ObjectKind> {
    pub layout: Vec<Vec<ObjectKind>>,
    pub pos: Point,
}

impl<ObjectKind: std::fmt::Display + Copy> std::fmt::Display for Warehouse<ObjectKind> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (ridx, row) in self.layout.iter().enumerate() {
            for (cidx, &obj) in row.iter().enumerate() {
                if Point(ridx, cidx) == self.pos {
                    write!(f, "@").unwrap();
                } else {
                    write!(f, "{}", obj.to_string()).unwrap();
                }
            }
            writeln!(f).unwrap();
        }

        Ok(())
    }
}

impl<ObjectKind: Copy + From<char>> FromIterator<String> for Warehouse<ObjectKind> {
    fn from_iter<T: IntoIterator<Item = String>>(iter: T) -> Self {
        let mut start_pos = None;
        let mut layout = vec![];
        for (ridx, row) in iter.into_iter().enumerate() {
            let mut obj_row = vec![];
            for (cidx, ch) in row.char_indices() {
                if ch == '@' {
                    start_pos = Some((ridx, cidx))
                }
                obj_row.push(ch.into())
            }
            layout.push(obj_row);
        }
        Warehouse {
            layout,
            pos: start_pos.unwrap().into(),
        }
    }
}

impl<ObjectKind> Index<Point> for Warehouse<ObjectKind> {
    type Output = ObjectKind;

    fn index(&self, index: Point) -> &Self::Output {
        &self.layout[index.0][index.1]
    }
}

impl<ObjectKind> IndexMut<Point> for Warehouse<ObjectKind> {
    fn index_mut(&mut self, index: Point) -> &mut Self::Output {
        &mut self.layout[index.0][index.1]
    }
}

impl<Obj: Copy + ObjectKind> Warehouse<Obj>
where
    Self: Sol,
{
    pub fn iter(&self) -> impl Iterator<Item = (Point, Obj)> {
        self.layout.iter().enumerate().flat_map(|(ridx, row)| {
            row.iter()
                .enumerate()
                .map(move |(cidx, &object)| (Point(ridx, cidx), object))
        })
    }

    pub fn move_robot(&mut self, dir: Move) {
        let dest = dir.new_pos(self.pos);
        if self[dest].is_box() {
            // try to push the boxes out of the way
            self.push_box(dest, dir);
        }

        // now, try to move in to the next location
        if self[dest].is_open() {
            self.pos = dest;
        }
    }

    pub fn boxes(&self) -> impl Iterator<Item = Point> {
        self.iter()
            .filter_map(|(loc, object_kind)| object_kind.is_box().then_some(loc))
    }
}
