use std::mem::size_of_val;

use super::{
    alu::{alu, alu_control},
    control_unit::{control_unit, MemToReg, PCSrc, RegDst},
    data_memory::{DataMem, DataMemory},
    instruction_memory::{InstructionMem, InstructionMemory},
    registers::{Register, Registers},
    virtual_machine_interface::{CPUInterface, RegisterKind},
};

#[derive(Debug, Clone)]
pub struct SingleCycleCPU {
    error_message: Option<String>,
    registers: Registers,
    pc: u32,

    instruction_memory: InstructionMemory,
    data_memory: DataMemory,
}

impl CPUInterface for SingleCycleCPU {
    fn get_memory_size(&self) -> u32 {
        self.data_memory.len() as u32
    }

    fn get_instruction_size(&self) -> u32 {
        self.instruction_memory.len() as u32
    }

    fn get_memory_byte(&self, address: u32) -> Option<u8> {
        self.data_memory.get(address as usize).copied()
    }

    fn get_register(&self, reg: RegisterKind) -> u32 {
        if matches!(reg, RegisterKind::RegPC) {
            self.pc
        } else {
            self.registers.get(reg as usize).copied().unwrap()
        }
    }

    fn get_error(&self) -> Option<String> {
        self.error_message.clone()
    }

    fn step(&mut self) {
        if let Some(inst) = self.instruction_memory.get_instruction(self.pc) {
            let inc_pc = self.pc + 4;
            let opcode = inst >> 26;
            let read_r1 = (inst >> 21) & 0b11111;
            let read_r2_write = (inst >> 16) & 0b11111;
            let write = (inst >> 11) & 0b11111;
            let immediate = inst & 0b1111_1111_1111_1111;
            let shamt = (inst >> 6) & 0b11111;
            let funct = inst & 0b111111;
            let jump_addr =
                ((inst & 0b11_1111_1111_1111_1111_1111_1111) >> 2) + (inc_pc & 0xFFFFFFF);

            let imm_sign_extended = sign_extend(immediate, 32);
            let branch_addr = (imm_sign_extended >> 2) + inc_pc;

            let data_1 = self.registers.read(read_r1);
            let data_2 = self.registers.read(read_r2_write);

            let control_signals = control_unit(opcode, funct);
            let alu_control_signals = alu_control(control_signals.alu_op, funct);
            let alu_result = alu(
                data_1,
                if control_signals.alu_src {
                    imm_sign_extended
                } else {
                    data_2
                },
                shamt,
                alu_control_signals,
            );

            let write_register = match control_signals.reg_dst {
                RegDst::RT => read_r2_write,
                RegDst::RD => write,
                RegDst::RA => 31,
            };

            self.pc = match control_signals.pc_src {
                PCSrc::PCBranch => {
                    if alu_result == 0 && funct == 0x04 {
                        branch_addr
                    } else if alu_result != 0 && funct == 0x05 {
                        branch_addr
                    } else {
                        inc_pc
                    }
                }
                PCSrc::PC => inc_pc,
                PCSrc::Jump => jump_addr,
                PCSrc::RegJump => read_r1,
            };

            let read_data = self.data_memory.load(alu_result, control_signals.mem_read);

            let reg_write_data = match control_signals.mem_to_reg {
                MemToReg::MemoryRead => match read_data {
                    Some(data) => data,
                    None => {
                        self.error_message = Some("Invalid memory read".to_string());
                        0
                    }
                },
                MemToReg::PCInc => inc_pc,
                MemToReg::ALUResult => alu_result,
                MemToReg::ImmLeftShift16 => immediate << 16,
            };

            self.data_memory
                .store(read_r2_write, alu_result, control_signals.mem_write);
            self.registers
                .write(reg_write_data, write_register, control_signals.reg_write);
        } else {
            self.error_message = Some("Out of range instruction".to_string());
        }
    }
}

impl SingleCycleCPU {
    pub fn new() -> SingleCycleCPU {
        SingleCycleCPU {
            error_message: None,
            registers: [0; 32],
            pc: 0,
            data_memory: vec![0; 1024],
            instruction_memory: Vec::new(),
        }
    }

    pub fn new_from_memory(instruction_memory: InstructionMemory, data_memory: DataMemory) -> Self {
        Self {
            error_message: None,
            registers: [0; 32],
            pc: 0,
            instruction_memory,
            data_memory,
        }
    }
}

fn sign_extend(v: u32, n_bits: u32) -> u32 {
    let other_bits = size_of_val(&v) as u32 * 8 - n_bits;
    v.wrapping_shl(other_bits).wrapping_shr(other_bits)
}
