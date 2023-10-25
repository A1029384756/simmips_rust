use super::{BEQ_OPCODE, BNE_OPCODE, JR_FUNCT};

#[derive(Debug)]
pub enum AluOp {
    RType,
    Add,
    Addu,
    Sub,
    And,
    Or,
    Slt,
    Sltu,
    None,
}

pub struct ControlUnitOutput {
    pub reg_dst: RegDst,
    pub alu_src: bool,
    pub mem_to_reg: MemToReg,
    pub reg_write: bool,
    pub mem_read: Mem,
    pub mem_write: Mem,
    pub pc_src: PCSrc,
    pub alu_op: AluOp,
}

#[derive(Debug)]
pub enum MemToReg {
    ALUResult,
    MemoryRead,
    PCInc,
    ImmLeftShift16,
}

#[derive(Debug)]
pub enum PCSrc {
    PC,
    PCBranch,
    Jump,
    RegJump,
}

#[derive(Debug)]
pub enum RegDst {
    RT,
    RD,
    RA,
}

#[derive(Debug)]
pub enum Mem {
    None,
    Byte,
    Half,
    Word,
}

pub fn control_unit(opcode: u32, function: u32) -> ControlUnitOutput {
    match opcode {
        // RTYPE
        0x00 => {
            if function == JR_FUNCT {
                ControlUnitOutput {
                    reg_dst: RegDst::RD,
                    alu_src: false,
                    mem_to_reg: MemToReg::ALUResult,
                    reg_write: false,
                    mem_read: Mem::None,
                    mem_write: Mem::None,
                    pc_src: PCSrc::RegJump,
                    alu_op: AluOp::RType,
                }
            } else {
                ControlUnitOutput {
                    reg_dst: RegDst::RD,
                    alu_src: false,
                    mem_to_reg: MemToReg::ALUResult,
                    reg_write: true,
                    mem_read: Mem::None,
                    mem_write: Mem::None,
                    pc_src: PCSrc::PC,
                    alu_op: AluOp::RType,
                }
            }
        }
        // LW
        0x23 => ControlUnitOutput {
            reg_dst: RegDst::RT,
            alu_src: true,
            mem_to_reg: MemToReg::MemoryRead,
            reg_write: true,
            mem_read: Mem::Word,
            mem_write: Mem::None,
            pc_src: PCSrc::PC,
            alu_op: AluOp::Add,
        },
        // LBU
        0x24 => ControlUnitOutput {
            reg_dst: RegDst::RT,
            alu_src: true,
            mem_to_reg: MemToReg::MemoryRead,
            reg_write: true,
            mem_read: Mem::Byte,
            mem_write: Mem::None,
            pc_src: PCSrc::PC,
            alu_op: AluOp::Add,
        },
        // LHU
        0x25 => ControlUnitOutput {
            reg_dst: RegDst::RT,
            alu_src: true,
            mem_to_reg: MemToReg::MemoryRead,
            reg_write: true,
            mem_read: Mem::Half,
            mem_write: Mem::None,
            pc_src: PCSrc::PC,
            alu_op: AluOp::Add,
        },
        // LL
        0x30 => ControlUnitOutput {
            reg_dst: RegDst::RT,
            alu_src: true,
            mem_to_reg: MemToReg::MemoryRead,
            reg_write: true,
            mem_read: Mem::Word,
            mem_write: Mem::None,
            pc_src: PCSrc::PC,
            alu_op: AluOp::Add,
        },
        // SW
        0x2B => ControlUnitOutput {
            reg_dst: RegDst::RT,
            alu_src: true,
            mem_to_reg: MemToReg::ALUResult,
            reg_write: false,
            mem_read: Mem::None,
            mem_write: Mem::Word,
            pc_src: PCSrc::PC,
            alu_op: AluOp::Add,
        },
        // SB
        0x28 => ControlUnitOutput {
            reg_dst: RegDst::RT,
            alu_src: true,
            mem_to_reg: MemToReg::ALUResult,
            reg_write: false,
            mem_read: Mem::None,
            mem_write: Mem::Byte,
            pc_src: PCSrc::PC,
            alu_op: AluOp::Add,
        },
        // SC
        0x38 => ControlUnitOutput {
            reg_dst: RegDst::RT,
            alu_src: true,
            mem_to_reg: MemToReg::ALUResult,
            reg_write: false,
            mem_read: Mem::None,
            mem_write: Mem::Word,
            pc_src: PCSrc::PC,
            alu_op: AluOp::Add,
        },
        // SW
        0x29 => ControlUnitOutput {
            reg_dst: RegDst::RT,
            alu_src: true,
            mem_to_reg: MemToReg::ALUResult,
            reg_write: false,
            mem_read: Mem::None,
            mem_write: Mem::Half,
            pc_src: PCSrc::PC,
            alu_op: AluOp::Add,
        },
        // BEQ
        BEQ_OPCODE => ControlUnitOutput {
            reg_dst: RegDst::RT,
            alu_src: false,
            mem_to_reg: MemToReg::ALUResult,
            reg_write: false,
            mem_read: Mem::None,
            mem_write: Mem::None,
            pc_src: PCSrc::PCBranch,
            alu_op: AluOp::Sub,
        },
        // BNE
        BNE_OPCODE => ControlUnitOutput {
            reg_dst: RegDst::RT,
            alu_src: false,
            mem_to_reg: MemToReg::ALUResult,
            reg_write: false,
            mem_read: Mem::None,
            mem_write: Mem::None,
            pc_src: PCSrc::PCBranch,
            alu_op: AluOp::Sub,
        },
        // ADDI
        0x08 => ControlUnitOutput {
            reg_dst: RegDst::RT,
            alu_src: true,
            mem_to_reg: MemToReg::ALUResult,
            reg_write: true,
            mem_read: Mem::None,
            mem_write: Mem::None,
            pc_src: PCSrc::PC,
            alu_op: AluOp::Add,
        },
        // ADDIU
        0x09 => ControlUnitOutput {
            reg_dst: RegDst::RT,
            alu_src: true,
            mem_to_reg: MemToReg::ALUResult,
            reg_write: true,
            mem_read: Mem::None,
            mem_write: Mem::None,
            pc_src: PCSrc::PC,
            alu_op: AluOp::Addu,
        },
        // ANDI
        0x0C => ControlUnitOutput {
            reg_dst: RegDst::RT,
            alu_src: true,
            mem_to_reg: MemToReg::ALUResult,
            reg_write: true,
            mem_read: Mem::None,
            mem_write: Mem::None,
            pc_src: PCSrc::PC,
            alu_op: AluOp::And,
        },
        // LUI
        0x0F => ControlUnitOutput {
            reg_dst: RegDst::RT,
            alu_src: false,
            mem_to_reg: MemToReg::ImmLeftShift16,
            reg_write: true,
            mem_read: Mem::None,
            mem_write: Mem::None,
            pc_src: PCSrc::PC,
            alu_op: AluOp::None,
        },
        // ORI
        0x0D => ControlUnitOutput {
            reg_dst: RegDst::RT,
            alu_src: true,
            mem_to_reg: MemToReg::ALUResult,
            reg_write: true,
            mem_read: Mem::None,
            mem_write: Mem::None,
            pc_src: PCSrc::PC,
            alu_op: AluOp::Or,
        },
        // SLTI
        0x0A => ControlUnitOutput {
            reg_dst: RegDst::RT,
            alu_src: true,
            mem_to_reg: MemToReg::ALUResult,
            reg_write: true,
            mem_read: Mem::None,
            mem_write: Mem::None,
            pc_src: PCSrc::PC,
            alu_op: AluOp::Slt,
        },
        // SLTIU
        0x0B => ControlUnitOutput {
            reg_dst: RegDst::RT,
            alu_src: true,
            mem_to_reg: MemToReg::ALUResult,
            reg_write: true,
            mem_read: Mem::None,
            mem_write: Mem::None,
            pc_src: PCSrc::PC,
            alu_op: AluOp::Sltu,
        },
        // J
        0x02 => ControlUnitOutput {
            reg_dst: RegDst::RD,
            alu_src: false,
            mem_to_reg: MemToReg::ALUResult,
            reg_write: false,
            mem_read: Mem::None,
            mem_write: Mem::None,
            pc_src: PCSrc::Jump,
            alu_op: AluOp::Sltu,
        },
        // JAL
        0x03 => ControlUnitOutput {
            reg_dst: RegDst::RA,
            alu_src: false,
            mem_to_reg: MemToReg::ALUResult,
            reg_write: true,
            mem_read: Mem::None,
            mem_write: Mem::None,
            pc_src: PCSrc::Jump,
            alu_op: AluOp::Sltu,
        },
        _ => panic!("unhandled opcode: {opcode:#x}"),
    }
}
