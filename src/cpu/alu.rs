use super::control_unit::AluOp;

pub enum AluOperation {
    Add,
    Addu,
    Sub,
    Subu,
    And,
    Or,
    Nor,
    Slt,
    Sltu,
    Sll,
    Srl,
    None,
}

pub fn alu_control(alu_op: AluOp, function_code: u32) -> AluOperation {
    match alu_op {
        AluOp::RType => match function_code {
            0x20 => AluOperation::Add,
            0x21 => AluOperation::Addu,
            0x24 => AluOperation::And,
            0x08 => AluOperation::None,
            0x27 => AluOperation::Nor,
            0x25 => AluOperation::Or,
            0x2A => AluOperation::Slt,
            0x2B => AluOperation::Sltu,
            0x00 => AluOperation::Sll,
            0x02 => AluOperation::Srl,
            0x22 => AluOperation::Sub,
            0x23 => AluOperation::Subu,
            _ => panic!("unhandled function code: {function_code:#x}"),
        },
        AluOp::Add => AluOperation::Add,
        AluOp::Addu => AluOperation::Addu,
        AluOp::Sub => AluOperation::Sub,
        AluOp::And => AluOperation::And,
        AluOp::Or => AluOperation::Or,
        AluOp::Slt => AluOperation::Slt,
        AluOp::Sltu => AluOperation::Sltu,
        AluOp::None => AluOperation::None,
    }
}

pub fn alu(op_a: u32, op_b: u32, shamt: u32, operation: AluOperation) -> u32 {
    match operation {
        AluOperation::Add => (op_a as i32).wrapping_add(op_b as i32) as u32,
        AluOperation::Addu => op_a.wrapping_add(op_b),
        AluOperation::Sub => (op_a as i32).wrapping_sub(op_b as i32) as u32,
        AluOperation::Subu => op_a.wrapping_sub(op_b),
        AluOperation::And => op_a & op_b,
        AluOperation::Or => op_a | op_b,
        AluOperation::Nor => !(op_a | op_b),
        AluOperation::Slt => ((op_a as i32) < (op_b as i32)) as u32,
        AluOperation::Sltu => (op_a < op_b) as u32,
        AluOperation::Sll => op_a << shamt,
        AluOperation::Srl => op_a >> shamt,
        AluOperation::None => 0,
    }
}
