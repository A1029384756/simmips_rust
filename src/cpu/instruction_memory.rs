pub type InstructionMemory = Vec<u32>;

pub trait InstructionMem {
    fn get_instruction(&self, pc: u32) -> Option<u32>;
}

impl InstructionMem for InstructionMemory {
    fn get_instruction(&self, pc: u32) -> Option<u32> {
        self.get((pc as usize) / 4).copied()
    }
}
