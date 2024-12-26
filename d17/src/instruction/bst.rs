use crate::operand::{Operand, combo_operand::ComboOperand};

use super::Instruction;

#[derive(Default)]

pub struct Bst;

impl Instruction for Bst {
    fn eval(&self, state: &mut crate::ProgramState, operand: u8) {
        let val = ComboOperand::from(operand).eval(&state);
        state.b = val % 8;
    }
}
