use super::virtual_machine_interface::RegisterKind;
use std::collections::BTreeMap;

#[derive(Debug, Clone, Copy)]
pub(crate) enum Opcode {
    Mfhi,
    Mflo,
    Mthi,
    Mtlo,
    Lw,
    Lh,
    Lb,
    La,
    Sw,
    Sh,
    Sb,
    Add,
    Addu,
    Sub,
    Subu,
    Mul,
    Mulo,
    Mulou,
    Rem,
    Remu,
    Mult,
    Multu,
    Abs,
    Neg,
    Negu,
    And,
    Nor,
    Or,
    Xor,
    Nop,
    Move,
    Li,
    Div,
    Divu,
    Not,
    Beq,
    Bne,
    Blt,
    Ble,
    Bgt,
    Bge,
    Jump,
    None,
}

#[derive(Debug, Clone)]
pub(crate) enum Argument {
    Register(RegisterKind),
    Immediate(u32),
    Label(String),
    Offset(u32),
}

#[derive(Debug)]
pub(crate) enum LabelType {
    Data(String),
    Instruction(String),
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
                opcode: Opcode::None,
                args: Vec::new(),
                source_line: 0,
            },
        }
    }

    pub(crate) fn set_opcode(&mut self, opcode: &str) {
        self.inst.opcode = match opcode {
            "mfhi" => Opcode::Mfhi,
            "mflo" => Opcode::Mflo,
            "mthi" => Opcode::Mthi,
            "lw" => Opcode::Lw,
            "lh" => Opcode::Lh,
            "lb" => Opcode::Lb,
            "la" => Opcode::La,
            "sw" => Opcode::Sw,
            "sh" => Opcode::Sh,
            "sb" => Opcode::Sb,
            "add" => Opcode::Add,
            "addu" => Opcode::Addu,
            "sub" => Opcode::Sub,
            "subu" => Opcode::Subu,
            "mul" => Opcode::Mul,
            "mulo" => Opcode::Mulo,
            "mulou" => Opcode::Mulou,
            "rem" => Opcode::Rem,
            "remu" => Opcode::Remu,
            "mult" => Opcode::Mult,
            "multu" => Opcode::Multu,
            "abs" => Opcode::Abs,
            "neg" => Opcode::Neg,
            "negu" => Opcode::Negu,
            "and" => Opcode::And,
            "nor" => Opcode::Nor,
            "or" => Opcode::Or,
            "xor" => Opcode::Xor,
            "nop" => Opcode::Nop,
            "move" => Opcode::Move,
            "li" => Opcode::Li,
            "div" => Opcode::Div,
            "divu" => Opcode::Divu,
            "not" => Opcode::Not,
            "beq" => Opcode::Beq,
            "bne" => Opcode::Bne,
            "blt" => Opcode::Blt,
            "ble" => Opcode::Ble,
            "bgt" => Opcode::Bgt,
            "bge" => Opcode::Bge,
            "j" => Opcode::Jump,
            _ => Opcode::None,
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
            opcode: Opcode::None,
            args: Vec::new(),
            source_line: 0,
        };

        return_inst
    }
}

pub(crate) type Labels = BTreeMap<String, u32>;
