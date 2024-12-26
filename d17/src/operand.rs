use crate::ProgramState;

pub mod combo_operand;
pub mod literal_operand;

pub trait Operand
where
    Self: Sized,
{
    fn eval(self, state: &ProgramState) -> u64;
}
