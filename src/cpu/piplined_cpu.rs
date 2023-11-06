use super::{
    alu::AluOperation,
    control_unit::ControlUnitOutput,
    cpu_interface::{CPUInterface, RegisterKind},
    data_memory::DataMemory,
    instruction_memory::InstructionMemory,
    registers::Registers,
    INST_MEM_START,
};

#[derive(Debug, Clone)]
pub struct PipelinedCPU {
    error_message: Option<String>,
    registers: Registers,
    pc: u32,

    instruction_memory: InstructionMemory,
    data_memory: DataMemory,
}

impl CPUInterface for PipelinedCPU {
    fn get_memory_size(&self) -> u32 {
        self.data_memory.len() as u32
    }

    fn get_instruction_size(&self) -> u32 {
        self.instruction_memory.len() as u32
    }

    fn get_register(&self, reg: RegisterKind) -> u32 {
        match reg {
            RegisterKind::RegPC => self.pc,
            _ => self.registers.get(reg as usize).copied().unwrap(),
        }
    }

    fn get_memory_byte(&self, address: u32) -> Option<u8> {
        self.data_memory.get(address as usize).copied()
    }

    fn get_control_signals(&self) -> ControlUnitOutput {
        todo!()
    }

    fn get_alu_signals(&self) -> AluOperation {
        todo!()
    }

    fn get_error(&self) -> Option<String> {
        self.error_message.clone()
    }

    fn step(&mut self) {
        todo!()
    }
}

impl PipelinedCPU {
    pub fn new() -> Self {
        Self {
            error_message: None,
            registers: [0; 32],
            pc: INST_MEM_START,
            instruction_memory: Vec::new(),
            data_memory: vec![0; 1024],
        }
    }

    pub fn new_from_memory(instruction_memory: InstructionMemory, data_memory: DataMemory) -> Self {
        Self {
            error_message: None,
            registers: [0; 32],
            pc: INST_MEM_START,
            instruction_memory,
            data_memory,
        }
    }
}
