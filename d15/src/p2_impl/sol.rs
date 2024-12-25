use crate::{ObjectKind as _, Sol, Warehouse};

use super::object_kind::ObjectKind;

impl Sol for Warehouse<ObjectKind> {
    fn push_box(&mut self, box_loc: crate::Point, dir: crate::Move) {
        if self.can_push(box_loc, dir) {
            self.push_box_rec(box_loc, dir, ObjectKind::Open);
        }
    }
}

impl Warehouse<ObjectKind> {
    fn can_push(&self, box_loc: crate::Point, dir: crate::Move) -> bool {
        match self[box_loc] {
            ObjectKind::Wall => false,
            ObjectKind::Open => true,
            r#box => {
                self.can_push(dir.new_pos(box_loc), dir)
                    && r#box
                        .other_half(box_loc, dir)
                        .is_none_or(|box_loc| self.can_push(dir.new_pos(box_loc), dir))
            }
        }
    }

    // assumes we have checked that box can be pushed
    fn push_box_rec(&mut self, box_loc: crate::Point, dir: crate::Move, parent_obj: ObjectKind) {
        // start by moving parent object into place
        let self_obj = std::mem::replace(&mut self[box_loc], parent_obj);
        // if this is a box, then move its companion, too
        if self_obj.is_box() {
            if let Some(companion) = self_obj.other_half(box_loc, dir) {
                let companion_obj = std::mem::take(&mut self[companion]);
                // need to propagate this move
                self.push_box_rec(dir.new_pos(companion), dir, companion_obj);
            }
            // since we pushed this box, need to push anything it was touching
            self.push_box_rec(dir.new_pos(box_loc), dir, self_obj);
        }

        // implicit base case: if we replaced empty space, then we can stop.
        // we already checked that there is no wall.
    }
}
