use adv::Adv;
use bdv::Bdv;
use bst::Bst;
use bxc::Bxc;
use bxl::Bxl;
use cdv::Cdv;
use jnz::Jnz;
use out::Out;
use strum::EnumIter;
use strum::IntoEnumIterator;

use crate::ProgramState;

mod adv;
mod bdv;
mod bst;
mod bxc;
mod bxl;
mod cdv;
mod jnz;
mod out;

#[enum_dispatch::enum_dispatch]
pub trait Instruction {
    fn eval(&self, state: &mut ProgramState, operand: u8);
}

#[enum_dispatch::enum_dispatch(Instruction)]
#[derive(EnumIter)]
pub enum InstructionKind {
    Adv(Adv),
    Bxl(Bxl),
    Bst(Bst),
    Jnz(Jnz),
    Bxc(Bxc),
    Out(Out),
    Bdv(Bdv),
    Cdv(Cdv),
}

impl From<u8> for InstructionKind {
    fn from(value: u8) -> Self {
        Self::iter().nth(value.into()).unwrap()
    }
}
