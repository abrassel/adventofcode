use super::{Instruction, adv::dv_eval};

#[derive(Default)]

pub struct Bdv;

impl Instruction for Bdv {
    fn eval(&self, state: &mut crate::ProgramState, operand: u8) {
        state.b = dv_eval(state, operand);
    }
}
