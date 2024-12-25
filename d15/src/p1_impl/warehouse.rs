use crate::{Move, Point, Sol, Warehouse};

use super::object::ObjectKind;

impl Sol for Warehouse<ObjectKind> {
    fn push_box(&mut self, box_loc: Point, dir: Move) {
        let mut target = box_loc;
        while self[target] == ObjectKind::Box {
            target = dir.new_pos(target);
        }
        if self[target] == ObjectKind::Open {
            self[target] = ObjectKind::Box;
            self[box_loc] = ObjectKind::Open;
        }
    }
}
