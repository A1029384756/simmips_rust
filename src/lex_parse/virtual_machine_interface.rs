use num_derive::FromPrimitive;

#[derive(Debug, Clone, Copy, FromPrimitive)]
pub enum RegisterKind {
    REG00 = 0,
    REG01 = 1,
    REG02 = 2,
    REG03 = 3,
    REG04 = 4,
    REG05 = 5,
    REG06 = 6,
    REG07 = 7,
    REG08 = 8,
    REG09 = 9,
    REG10 = 10,
    REG11 = 11,
    REG12 = 12,
    REG13 = 13,
    REG14 = 14,
    REG15 = 15,
    REG16 = 16,
    REG17 = 17,
    REG18 = 18,
    REG19 = 19,
    REG20 = 20,
    REG21 = 21,
    REG22 = 22,
    REG23 = 23,
    REG24 = 24,
    REG25 = 25,
    REG26 = 26,
    REG27 = 27,
    REG28 = 28,
    REG29 = 29,
    REG30 = 30,
    REG31 = 31,
    REGHI = 32,
    REGLO = 33,
    REGPC = 34,
}

pub trait VirtualMachineInterface {
    fn get_memory_size(&self) -> u32;
    fn get_instruction_size(&self) -> u32;
    fn get_memory_byte(&self, address: u32) -> Option<u8>;
    fn get_register(&self, reg: RegisterKind) -> u32;
    fn get_current_source_line(&self) -> u32;
    fn is_error(&self) -> bool;
    fn get_error(&self) -> String;
    fn step(&mut self);
}
