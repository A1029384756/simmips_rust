use std::fmt::Debug;

use num_derive::FromPrimitive;

use super::{alu::AluOperation, control_unit::ControlUnitOutput};

#[derive(Debug, Clone, Copy, FromPrimitive)]
pub enum RegisterKind {
    Reg00 = 0,
    Reg01 = 1,
    Reg02 = 2,
    Reg03 = 3,
    Reg04 = 4,
    Reg05 = 5,
    Reg06 = 6,
    Reg07 = 7,
    Reg08 = 8,
    Reg09 = 9,
    Reg10 = 10,
    Reg11 = 11,
    Reg12 = 12,
    Reg13 = 13,
    Reg14 = 14,
    Reg15 = 15,
    Reg16 = 16,
    Reg17 = 17,
    Reg18 = 18,
    Reg19 = 19,
    Reg20 = 20,
    Reg21 = 21,
    Reg22 = 22,
    Reg23 = 23,
    Reg24 = 24,
    Reg25 = 25,
    Reg26 = 26,
    Reg27 = 27,
    Reg28 = 28,
    Reg29 = 29,
    Reg30 = 30,
    Reg31 = 31,
    RegPC = 32,
}

pub trait CPUInterface: Send {
    fn get_memory_size(&self) -> u32;
    fn get_instruction_size(&self) -> u32;

    fn get_register(&self, reg: RegisterKind) -> u32;
    fn get_memory_byte(&self, address: u32) -> Option<u8>;

    fn get_control_signals(&self) -> ControlUnitOutput;
    fn get_alu_signals(&self) -> AluOperation;

    fn get_error(&self) -> Option<String>;

    fn step(&mut self);
}

impl Debug for dyn CPUInterface {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "CPU with memory size: {}, instruction size: {}, and error: {:?}",
            self.get_memory_size(),
            self.get_instruction_size(),
            self.get_error()
        )
    }
}
