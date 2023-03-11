use super::virtual_machine_interface::RegisterKind;
use std::collections::BTreeMap;

#[derive(Debug, Clone, Copy)]
pub(crate) enum Opcode {
    MFHI,
    MFLO,
    MTHI,
    MTLO,
    LW,
    LH,
    LB,
    LA,
    SW,
    SH,
    SB,
    ADD,
    ADDU,
    SUB,
    SUBU,
    MUL,
    MULO,
    MULOU,
    REM,
    REMU,
    MULT,
    MULTU,
    ABS,
    NEG,
    NEGU,
    AND,
    NOR,
    OR,
    XOR,
    NOP,
    MOVE,
    LI,
    DIV,
    DIVU,
    NOT,
    BEQ,
    BNE,
    BLT,
    BLE,
    BGT,
    BGE,
    JUMP,
    NONE,
}

#[derive(Debug, Clone)]
pub(crate) enum Argument {
    REGISTER(RegisterKind),
    IMMEDIATE(u32),
    LABEL(String),
    OFFSET(u32),
}

#[derive(Debug)]
pub(crate) enum LabelType {
    DATA(String),
    INSTRUCTION(String),
}

#[derive(Debug, Clone)]
pub(crate) struct Instruction {
    pub opcode: Opcode,
    pub args: Vec<Argument>,
    pub source_line: u32,
}

#[derive(Debug)]
pub(crate) struct InstructionBuilder {
    inst: Instruction,
}

impl InstructionBuilder {
    pub(crate) fn new() -> InstructionBuilder {
        InstructionBuilder {
            inst: Instruction {
                opcode: Opcode::NONE,
                args: Vec::new(),
                source_line: 0,
            },
        }
    }

    pub(crate) fn set_opcode(&mut self, opcode: &str) {
        self.inst.opcode = match opcode {
            "mfhi" => Opcode::MFHI,
            "mflo" => Opcode::MFLO,
            "mthi" => Opcode::MTHI,
            "lw" => Opcode::LW,
            "lh" => Opcode::LH,
            "lb" => Opcode::LB,
            "la" => Opcode::LA,
            "sw" => Opcode::SW,
            "sh" => Opcode::SH,
            "sb" => Opcode::SB,
            "add" => Opcode::ADD,
            "addu" => Opcode::ADDU,
            "sub" => Opcode::SUB,
            "subu" => Opcode::SUBU,
            "mul" => Opcode::MUL,
            "mulo" => Opcode::MULO,
            "mulou" => Opcode::MULOU,
            "rem" => Opcode::REM,
            "remu" => Opcode::REMU,
            "mult" => Opcode::MULT,
            "multu" => Opcode::MULTU,
            "abs" => Opcode::ABS,
            "neg" => Opcode::NEG,
            "negu" => Opcode::NEGU,
            "and" => Opcode::AND,
            "nor" => Opcode::NOR,
            "or" => Opcode::OR,
            "xor" => Opcode::XOR,
            "nop" => Opcode::NOP,
            "move" => Opcode::MOVE,
            "li" => Opcode::LI,
            "div" => Opcode::DIV,
            "divu" => Opcode::DIVU,
            "not" => Opcode::NOT,
            "beq" => Opcode::BEQ,
            "bne" => Opcode::BNE,
            "blt" => Opcode::BLT,
            "ble" => Opcode::BLE,
            "bgt" => Opcode::BGT,
            "bge" => Opcode::BGE,
            "j" => Opcode::JUMP,
            _ => Opcode::NONE,
        };
    }

    pub(crate) fn add_arg(&mut self, arg: Argument) {
        self.inst.args.push(arg);
    }

    pub(crate) fn set_line(&mut self, line: u32) {
        self.inst.source_line = line
    }

    pub(crate) fn get_inst(&mut self) -> Instruction {
        let return_inst = self.inst.clone();
        self.inst = Instruction {
            opcode: Opcode::NONE,
            args: Vec::new(),
            source_line: 0,
        };

        return_inst
    }
}

pub(crate) type Labels = BTreeMap<String, u32>;
