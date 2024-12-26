use crate::operand::{Operand, literal_operand::LiteralOperand};

use super::Instruction;

#[derive(Default)]

pub struct Bxl;

impl Instruction for Bxl {
    fn eval(&self, state: &mut crate::ProgramState, operand: u8) {
        let res = state.b ^ LiteralOperand(operand).eval(&state);
        state.b = res;
    }
}
