use super::Operand;

pub struct LiteralOperand(pub u8);

impl Operand for LiteralOperand {
    fn eval(self, _state: &crate::ProgramState) -> u64 {
        self.0.into()
    }
}
