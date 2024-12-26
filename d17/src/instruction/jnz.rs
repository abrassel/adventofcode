use crate::operand::{Operand, literal_operand::LiteralOperand};

use super::Instruction;

#[derive(Default)]

pub struct Jnz;

impl Instruction for Jnz {
    fn eval(&self, state: &mut crate::ProgramState, operand: u8) {
        if state.a == 0 {
            return;
        }

        let new_dest = LiteralOperand(operand).eval(state);
        state.jumped_flag = true;
        state.loc = new_dest as usize;
    }
}
