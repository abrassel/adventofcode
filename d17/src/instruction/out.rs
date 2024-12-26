use crate::operand::{Operand, combo_operand::ComboOperand};

use super::Instruction;

#[derive(Default)]

pub struct Out;

impl Instruction for Out {
    fn eval(&self, state: &mut crate::ProgramState, operand: u8) {
        let val = ComboOperand::from(operand).eval(state) % 8;
        state.output.push(val);
    }
}
