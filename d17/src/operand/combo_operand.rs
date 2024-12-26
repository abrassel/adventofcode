use strum::VariantArray;

use super::Operand;

#[derive(VariantArray, Copy, Clone)]
pub enum ComboOperand {
    Lit0,
    Lit1,
    Lit2,
    Lit3,
    RegisterA,
    RegisterB,
    RegisterC,
}

impl Operand for ComboOperand {
    fn eval(self, state: &crate::ProgramState) -> u64 {
        match self {
            ComboOperand::Lit0 => 0,
            ComboOperand::Lit1 => 1,
            ComboOperand::Lit2 => 2,
            ComboOperand::Lit3 => 3,
            ComboOperand::RegisterA => state.a,
            ComboOperand::RegisterB => state.b,
            ComboOperand::RegisterC => state.c,
        }
    }
}

impl From<u8> for ComboOperand {
    fn from(value: u8) -> Self {
        Self::VARIANTS[value as usize]
    }
}
