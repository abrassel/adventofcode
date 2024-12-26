use super::{Instruction, adv::dv_eval};

#[derive(Default)]

pub struct Cdv;

impl Instruction for Cdv {
    fn eval(&self, state: &mut crate::ProgramState, operand: u8) {
        state.c = dv_eval(state, operand);
    }
}
