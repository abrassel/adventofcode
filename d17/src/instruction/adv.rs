use crate::{
    ProgramState,
    operand::{Operand, combo_operand::ComboOperand},
};

use super::Instruction;

#[derive(Default)]
pub struct Adv;

pub(crate) fn dv_eval(state: &mut ProgramState, operand: u8) -> u64 {
    let numerator = state.a;
    let denominator = 2u64.pow(ComboOperand::from(operand).eval(&state) as u32);
    numerator / denominator
}

impl Instruction for Adv {
    fn eval(&self, state: &mut crate::ProgramState, operand: u8) {
        state.a = dv_eval(state, operand);
    }
}
