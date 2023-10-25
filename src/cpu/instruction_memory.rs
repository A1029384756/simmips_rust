use super::INST_MEM_START;

pub type InstructionMemory = Vec<u32>;

pub trait InstructionMem {
    fn get_instruction(&self, pc: u32) -> Option<u32>;
}

impl InstructionMem for InstructionMemory {
    fn get_instruction(&self, pc: u32) -> Option<u32> {
        self.get(((pc - INST_MEM_START) as usize) / 4).copied()
    }
}
