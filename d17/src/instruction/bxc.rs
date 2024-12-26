use super::Instruction;

#[derive(Default)]

pub struct Bxc;

impl Instruction for Bxc {
    fn eval(&self, state: &mut crate::ProgramState, _operand: u8) {
        let bval = state.b;
        let cval = state.c;
        let xor = bval ^ cval;
        state.b = xor;
    }
}
