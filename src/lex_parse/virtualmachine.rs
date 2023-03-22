use super::virtual_machine_interface::{RegisterKind, VirtualMachineInterface};
use super::vm_defs::{Argument, Instruction, LabelType, Labels, Opcode};

#[derive(Debug)]
pub struct VirtualMachine {
    error_state: bool,
    error_message: String,
    registers: [u32; 32],
    pc: u32,
    hi: u32,
    lo: u32,

    data_mem_back: u32,
    data_memory: Vec<u8>,
    instruction_memory: Vec<Instruction>,
    labels: Labels,
}

impl VirtualMachineInterface for VirtualMachine {
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
        if reg as usize <= 31 {
            self.registers[reg as usize]
        } else if reg as usize == 32 {
            self.hi
        } else if reg as usize == 33 {
            self.lo
        } else if reg as usize == 34 {
            self.pc
        } else {
            0
        }
    }

    fn get_current_source_line(&self) -> u32 {
        match self.instruction_memory.get(self.pc as usize) {
            Some(line) => line.source_line,
            None => 0,
        }
    }

    fn is_error(&self) -> bool {
        self.error_state
    }

    fn get_error(&self) -> String {
        self.error_message.to_string()
    }

    fn step(&mut self) {
        if self.error_state {
            return;
        }

        match self.instruction_memory.get(self.pc as usize) {
            Some(inst) => match inst.opcode {
                Opcode::MFHI => match inst.args.first() {
                    Some(Argument::REGISTER(reg)) => {
                        self.registers[*reg as usize] = self.hi;
                        self.pc += 1;
                    }
                    _ => {
                        self.error_state = true;
                        self.error_message =
                            "Invalid instruction format, resulting from parse error".to_string();
                    }
                },
                Opcode::MFLO => match inst.args.first() {
                    Some(Argument::REGISTER(reg)) => {
                        self.registers[*reg as usize] = self.lo;
                        self.pc += 1;
                    }
                    _ => {
                        self.error_state = true;
                        self.error_message =
                            "Invalid instruction format, resulting from parse error".to_string();
                    }
                },
                Opcode::MTHI => match inst.args.first() {
                    Some(Argument::REGISTER(reg)) => {
                        self.hi = self.get_register(*reg);
                        self.pc += 1;
                    }
                    _ => {
                        self.error_state = true;
                        self.error_message =
                            "Invalid instruction format, resulting from parse error".to_string();
                    }
                },
                Opcode::MTLO => match inst.args.first() {
                    Some(Argument::REGISTER(reg)) => {
                        self.lo = self.get_register(*reg);
                        self.pc += 1;
                    }
                    _ => {
                        self.error_state = true;
                        self.error_message =
                            "Invalid instruction format, resulting from parse error".to_string();
                    }
                },
                Opcode::LW => match self.get_data_instruction() {
                    Some((dest, address)) => match self
                        .data_memory
                        .get((address as usize)..=(address + 3) as usize)
                    {
                        Some(word) => {
                            self.registers[*dest as usize] = word
                                .iter()
                                .rev()
                                .fold(0u32, |total, val| (total << 8) + (*val as u32));
                            self.pc += 1;
                        }
                        None => {
                            self.error_state = true;
                            self.error_message =
                                "Attempted to load from invalid memory address".to_string();
                        }
                    },
                    None => {
                        self.error_state = true;
                        self.error_message =
                            "Invalid instruction format, resulting from parse error".to_string();
                    }
                },
                Opcode::LH => match self.get_data_instruction() {
                    Some((dest, address)) => match self
                        .data_memory
                        .get((address as usize)..=(address + 1) as usize)
                    {
                        Some(half) => {
                            self.registers[*dest as usize] = half
                                .iter()
                                .rev()
                                .fold(0u32, |total, val| (total << 8) + (*val as u32));
                            self.pc += 1;
                        }
                        None => {
                            self.error_state = true;
                            self.error_message =
                                "Attempted to load from invalid memory address".to_string();
                        }
                    },
                    None => {
                        self.error_state = true;
                        self.error_message =
                            "Invalid instruction format, resulting from parse error".to_string();
                    }
                },
                Opcode::LB => match self.get_data_instruction() {
                    Some((dest, address)) => match self.data_memory.get(address as usize) {
                        Some(byte) => {
                            self.registers[*dest as usize] = *byte as u32;
                            self.pc += 1;
                        }
                        None => {
                            self.error_state = true;
                            self.error_message =
                                "Attempted to load from invalid memory address".to_string();
                        }
                    },
                    None => {
                        self.error_state = true;
                        self.error_message =
                            "Invalid instruction format, resulting from parse error".to_string();
                    }
                },
                Opcode::LA => match self.get_data_instruction() {
                    Some((dest, address)) => {
                        self.registers[*dest as usize] = address;
                        self.pc += 1;
                    }
                    None => {
                        self.error_state = true;
                        self.error_message =
                            "Invalid instruction format, resulting from parse error".to_string();
                    }
                },
                Opcode::SW => match self.get_data_instruction() {
                    Some((source, address)) => {
                        self.get_register(*source)
                            .to_le_bytes()
                            .iter()
                            .enumerate()
                            .for_each(|(i, byte)| {
                                self.data_memory[address as usize + i] = *byte;
                            });
                        self.pc += 1;
                    }
                    None => {
                        self.error_state = true;
                        self.error_message =
                            "Invalid instruction format, resulting from parse error".to_string();
                    }
                },
                Opcode::SH => match self.get_data_instruction() {
                    Some((source, address)) => {
                        self.get_register(*source)
                            .to_le_bytes()
                            .iter()
                            .take(2)
                            .enumerate()
                            .for_each(|(i, byte)| {
                                self.data_memory[address as usize + i] = *byte;
                            });
                        self.pc += 1;
                    }
                    None => {
                        self.error_state = true;
                        self.error_message =
                            "Invalid instruction format, resulting from parse error".to_string();
                    }
                },
                Opcode::SB => match self.get_data_instruction() {
                    Some((source, address)) => {
                        self.data_memory[address as usize] =
                            *self.get_register(*source).to_le_bytes().first().unwrap();
                        self.pc += 1;
                    }
                    None => {
                        self.error_state = true;
                        self.error_message =
                            "Invalid instruction format, resulting from parse error".to_string();
                    }
                },
                Opcode::ADD => match self.get_signed_instruction() {
                    Some((dest, a, b)) => {
                        match i32::try_from((a as i32 as i64) + (b as i32 as i64)) {
                            Ok(result) => {
                                self.registers[*dest as usize] = result as u32;
                                self.pc += 1;
                            }
                            Err(..) => {
                                self.error_state = true;
                                self.error_message =
                                    "Integer overflow performing addition".to_string();
                            }
                        }
                    }
                    None => {
                        self.error_state = true;
                        self.error_message =
                            "Invalid instruction format, resulting from parse error".to_string();
                    }
                },
                Opcode::ADDU => match self.get_unsigned_instruction() {
                    Some((dest, a, b)) => {
                        self.registers[*dest as usize] = ((a as u64) + (b as u64)) as u32;
                        self.pc += 1;
                    }
                    None => {
                        self.error_state = true;
                        self.error_message =
                            "Invalid instruction format, resulting from parse error".to_string();
                    }
                },
                Opcode::SUB => match self.get_signed_instruction() {
                    Some((dest, a, b)) => {
                        match i32::try_from((a as i32 as i64) - (b as i32 as i64)) {
                            Ok(result) => {
                                self.registers[*dest as usize] = result as u32;
                                self.pc += 1;
                            }
                            Err(..) => {
                                self.error_state = true;
                                self.error_message =
                                    "Integer overflow performing subtraction".to_string();
                            }
                        }
                    }
                    None => {
                        self.error_state = true;
                        self.error_message =
                            "Invalid instruction format, resulting from parse error".to_string();
                    }
                },
                Opcode::SUBU => match self.get_unsigned_instruction() {
                    Some((dest, a, b)) => {
                        self.registers[*dest as usize] = ((a as u64) - (b as u64)) as u32;
                        self.pc += 1;
                    }
                    None => {
                        self.error_state = true;
                        self.error_message =
                            "Invalid instruction format, resulting from parse error".to_string();
                    }
                },
                Opcode::MUL => match self.get_signed_instruction() {
                    Some((dest, a, b)) => {
                        let result = a as i32 as i64 * b as i32 as i64;
                        self.registers[*dest as usize] = result as u32;
                        self.hi = (result >> 32) as u32;
                        self.lo = result as u32;
                        self.pc += 1;
                    }
                    None => {
                        self.error_state = true;
                        self.error_message =
                            "Invalid instruction format, resulting from parse error".to_string();
                    }
                },
                Opcode::MULO => match self.get_signed_instruction() {
                    Some((dest, a, b)) => {
                        match i32::try_from((a as i32 as i64) * (b as i32 as i64)) {
                            Ok(result) => {
                                self.registers[*dest as usize] = result as u32;
                                self.pc += 1;
                            }
                            Err(..) => {
                                self.error_state = true;
                                self.error_message =
                                    "Integer overflow performing multiplication".to_string();
                            }
                        }
                    }
                    None => {
                        self.error_state = true;
                        self.error_message =
                            "Invalid instruction format, resulting from parse error".to_string();
                    }
                },
                Opcode::MULOU => match self.get_unsigned_instruction() {
                    Some((dest, a, b)) => match u32::try_from((a as u64) * (b as u64)) {
                        Ok(result) => {
                            self.registers[*dest as usize] = result;
                            self.pc += 1;
                        }
                        Err(..) => {
                            self.error_state = true;
                            self.error_message =
                                "Integer overflow performing multiplication".to_string();
                        }
                    },
                    None => {
                        self.error_state = true;
                        self.error_message =
                            "Invalid instruction format, resulting from parse error".to_string();
                    }
                },
                Opcode::REM => match self.get_signed_instruction() {
                    Some((dest, a, b)) => {
                        let result = a as i32 as i64 % b as i32 as i64;
                        self.registers[*dest as usize] = result as u32;
                        self.hi = (result >> 32) as u32;
                        self.lo = result as u32;
                        self.pc += 1;
                    }
                    None => {
                        self.error_state = true;
                        self.error_message =
                            "Invalid instruction format, resulting from parse error".to_string();
                    }
                },
                Opcode::REMU => match self.get_unsigned_instruction() {
                    Some((dest, a, b)) => {
                        let result = a as u64 % b as u64;
                        self.registers[*dest as usize] = result as u32;
                        self.hi = (result >> 32) as u32;
                        self.lo = result as u32;
                        self.pc += 1;
                    }
                    None => {
                        self.error_state = true;
                        self.error_message =
                            "Invalid instruction format, resulting from parse error".to_string();
                    }
                },
                Opcode::MULT => match (inst.args.first(), inst.args.get(1)) {
                    (Some(Argument::REGISTER(a)), Some(Argument::REGISTER(b))) => {
                        let result = self.registers[*a as usize] as i32 as i64
                            * self.registers[*b as usize] as i32 as i64;

                        self.hi = (result >> 32) as u32;
                        self.lo = result as u32;
                        self.pc += 1;
                    }
                    _ => {
                        self.error_state = true;
                        self.error_message =
                            "Invalid instruction format, resulting from parse error".to_string();
                    }
                },
                Opcode::MULTU => match (inst.args.first(), inst.args.get(1)) {
                    (Some(Argument::REGISTER(a)), Some(Argument::REGISTER(b))) => {
                        let result =
                            self.registers[*a as usize] as u64 * self.registers[*b as usize] as u64;
                        self.hi = (result >> 32) as u32;
                        self.lo = result as u32;
                        self.pc += 1;
                    }
                    _ => {
                        self.error_state = true;
                        self.error_message =
                            "Invalid instruction format, resulting from parse error".to_string();
                    }
                },
                Opcode::ABS => match (inst.args.first(), inst.args.get(1)) {
                    (Some(Argument::REGISTER(dest)), Some(Argument::REGISTER(src))) => {
                        self.registers[*dest as usize] =
                            (self.registers[*src as usize] as i32).abs() as u32;
                        self.pc += 1;
                    }
                    _ => {
                        self.error_state = true;
                        self.error_message =
                            "Invalid instruction format, resulting from parse error".to_string();
                    }
                },
                Opcode::NEG => match (inst.args.first(), inst.args.get(1)) {
                    (Some(Argument::REGISTER(dest)), Some(Argument::REGISTER(src))) => {
                        match i32::try_from(-(self.registers[*src as usize] as i32)) {
                            Ok(neg) => {
                                self.registers[*dest as usize] = neg as u32;
                                self.pc += 1;
                            }
                            Err(..) => {
                                self.error_state = true;
                                self.error_message =
                                    "Integer overflow when attempting negation".to_string();
                            }
                        }
                    }
                    _ => {
                        self.error_state = true;
                        self.error_message =
                            "Invalid instruction format, resulting from parse error".to_string();
                    }
                },
                Opcode::NEGU => match (inst.args.first(), inst.args.get(1)) {
                    (Some(Argument::REGISTER(dest)), Some(Argument::REGISTER(src))) => {
                        self.registers[*dest as usize] =
                            -(self.registers[*src as usize] as i32) as u32;
                        self.pc += 1;
                    }
                    _ => {
                        self.error_state = true;
                        self.error_message =
                            "Invalid instruction format, resulting from parse error".to_string();
                    }
                },
                Opcode::AND => match self.get_unsigned_instruction() {
                    Some((dest, a, b)) => {
                        self.registers[*dest as usize] = a & b;
                        self.pc += 1;
                    }
                    None => {
                        self.error_state = true;
                        self.error_message =
                            "Invalid instruction format, resulting from parse error".to_string();
                    }
                },
                Opcode::NOR => match self.get_unsigned_instruction() {
                    Some((dest, a, b)) => {
                        self.registers[*dest as usize] = !(a | b);
                        self.pc += 1;
                    }
                    None => {
                        self.error_state = true;
                        self.error_message =
                            "Invalid instruction format, resulting from parse error".to_string();
                    }
                },
                Opcode::OR => match self.get_unsigned_instruction() {
                    Some((dest, a, b)) => {
                        self.registers[*dest as usize] = a | b;
                        self.pc += 1;
                    }
                    None => {
                        self.error_state = true;
                        self.error_message =
                            "Invalid instruction format, resulting from parse error".to_string();
                    }
                },
                Opcode::XOR => match self.get_unsigned_instruction() {
                    Some((dest, a, b)) => {
                        self.registers[*dest as usize] = a ^ b;
                        self.pc += 1;
                    }
                    None => {
                        self.error_state = true;
                        self.error_message =
                            "Invalid instruction format, resulting from parse error".to_string();
                    }
                },
                Opcode::MOVE => match (inst.args.first(), inst.args.get(1)) {
                    (Some(Argument::REGISTER(dest)), Some(Argument::REGISTER(src))) => {
                        self.registers[*dest as usize] = self.registers[*src as usize];
                        self.pc += 1;
                    }
                    _ => {
                        self.error_state = true;
                        self.error_message =
                            "Invalid instruction format, resulting from parse error".to_string();
                    }
                },
                Opcode::LI => match (inst.args.first(), inst.args.get(1)) {
                    (Some(Argument::REGISTER(dest)), Some(Argument::IMMEDIATE(imm))) => {
                        self.registers[*dest as usize] = *imm;
                        self.pc += 1;
                    }
                    _ => {
                        self.error_state = true;
                        self.error_message =
                            "Invalid instruction format, resulting from parse error".to_string();
                    }
                },
                Opcode::DIV => match (inst.args.first(), inst.args.get(1), inst.args.get(2)) {
                    (
                        Some(Argument::REGISTER(dest)),
                        Some(Argument::REGISTER(a)),
                        Some(Argument::REGISTER(b)),
                    ) => match (
                        (self.registers[*a as usize] as i32)
                            .checked_div(self.registers[*b as usize] as i32),
                        (self.registers[*a as usize] as i32)
                            .checked_rem(self.registers[*b as usize] as i32),
                    ) {
                        (Some(div_result), Some(remainder)) => {
                            self.registers[*dest as usize] = div_result as u32;
                            self.lo = div_result as u32;
                            self.hi = remainder as u32;
                            self.pc += 1;
                        }
                        _ => self.pc += 1,
                    },
                    (
                        Some(Argument::REGISTER(dest)),
                        Some(Argument::REGISTER(a)),
                        Some(Argument::IMMEDIATE(b)),
                    ) => match (
                        (self.registers[*a as usize] as i32).checked_div(*b as i32),
                        (self.registers[*a as usize] as i32).checked_rem(*b as i32),
                    ) {
                        (Some(div_result), Some(remainder)) => {
                            self.registers[*dest as usize] = div_result as u32;
                            self.lo = div_result as u32;
                            self.hi = remainder as u32;
                            self.pc += 1;
                        }
                        _ => self.pc += 1,
                    },
                    (Some(Argument::REGISTER(a)), Some(Argument::REGISTER(b)), None) => match (
                        (self.registers[*a as usize] as i32)
                            .checked_div(self.registers[*b as usize] as i32),
                        (self.registers[*a as usize] as i32)
                            .checked_rem(self.registers[*b as usize] as i32),
                    ) {
                        (Some(div_result), Some(remainder)) => {
                            self.lo = div_result as u32;
                            self.hi = remainder as u32;
                            self.pc += 1;
                        }
                        _ => self.pc += 1,
                    },
                    _ => {
                        self.error_state = true;
                        self.error_message =
                            "Invalid instruction format, resulting from parse error".to_string();
                    }
                },
                Opcode::DIVU => match (inst.args.first(), inst.args.get(1), inst.args.get(2)) {
                    (
                        Some(Argument::REGISTER(dest)),
                        Some(Argument::REGISTER(a)),
                        Some(Argument::REGISTER(b)),
                    ) => match (
                        (self.registers[*a as usize]).checked_div(self.registers[*b as usize]),
                        (self.registers[*a as usize]).checked_rem(self.registers[*b as usize]),
                    ) {
                        (Some(div_result), Some(remainder)) => {
                            self.registers[*dest as usize] = div_result;
                            self.lo = div_result;
                            self.hi = remainder;
                            self.pc += 1;
                        }
                        _ => self.pc += 1,
                    },
                    (
                        Some(Argument::REGISTER(dest)),
                        Some(Argument::REGISTER(a)),
                        Some(Argument::IMMEDIATE(b)),
                    ) => match (
                        (self.registers[*a as usize]).checked_div(*b),
                        (self.registers[*a as usize]).checked_rem(*b),
                    ) {
                        (Some(div_result), Some(remainder)) => {
                            self.registers[*dest as usize] = div_result;
                            self.lo = div_result;
                            self.hi = remainder;
                            self.pc += 1;
                        }
                        _ => self.pc += 1,
                    },
                    (Some(Argument::REGISTER(a)), Some(Argument::REGISTER(b)), None) => match (
                        (self.registers[*a as usize]).checked_div(self.registers[*b as usize]),
                        (self.registers[*a as usize]).checked_rem(self.registers[*b as usize]),
                    ) {
                        (Some(div_result), Some(remainder)) => {
                            self.lo = div_result;
                            self.hi = remainder;
                            self.pc += 1;
                        }
                        _ => self.pc += 1,
                    },
                    _ => {
                        self.error_state = true;
                        self.error_message =
                            "Invalid instruction format, resulting from parse error".to_string();
                    }
                },
                Opcode::NOT => match (inst.args.first(), inst.args.get(1)) {
                    (Some(Argument::REGISTER(dest)), Some(Argument::REGISTER(src))) => {
                        self.registers[*dest as usize] = !self.registers[*src as usize];
                        self.pc += 1;
                    }
                    (Some(Argument::REGISTER(dest)), Some(Argument::IMMEDIATE(src))) => {
                        self.registers[*dest as usize] = !(*src);
                        self.pc += 1;
                    }
                    _ => {
                        self.error_state = true;
                        self.error_message =
                            "Invalid instruction format, resulting from parse error".to_string();
                    }
                },
                Opcode::BEQ => match self.get_branch_operation() {
                    Some((a, b, label)) => {
                        if a == b {
                            match self.labels.get(&label) {
                                Some(addr) => self.pc = *addr,
                                None => {
                                    self.error_state = true;
                                    self.error_message =
                                        "Nonexistent label, resulting from parse error".to_string();
                                }
                            }
                        } else {
                            self.pc += 1;
                        }
                    }
                    _ => {
                        self.error_state = true;
                        self.error_message =
                            "Invalid instruction format, resulting from parse error".to_string();
                    }
                },
                Opcode::BNE => match self.get_branch_operation() {
                    Some((a, b, label)) => {
                        if a != b {
                            match self.labels.get(&label) {
                                Some(addr) => self.pc = *addr,
                                None => {
                                    self.error_state = true;
                                    self.error_message =
                                        "Nonexistent label, resulting from parse error".to_string();
                                }
                            }
                        } else {
                            self.pc += 1;
                        }
                    }
                    _ => {
                        self.error_state = true;
                        self.error_message =
                            "Invalid instruction format, resulting from parse error".to_string();
                    }
                },
                Opcode::BLT => match self.get_branch_operation() {
                    Some((a, b, label)) => {
                        if a < b {
                            match self.labels.get(&label) {
                                Some(addr) => self.pc = *addr,
                                None => {
                                    self.error_state = true;
                                    self.error_message =
                                        "Nonexistent label, resulting from parse error".to_string();
                                }
                            }
                        } else {
                            self.pc += 1;
                        }
                    }
                    _ => {
                        self.error_state = true;
                        self.error_message =
                            "Invalid instruction format, resulting from parse error".to_string();
                    }
                },
                Opcode::BLE => match self.get_branch_operation() {
                    Some((a, b, label)) => {
                        if a <= b {
                            match self.labels.get(&label) {
                                Some(addr) => self.pc = *addr,
                                None => {
                                    self.error_state = true;
                                    self.error_message =
                                        "Nonexistent label, resulting from parse error".to_string();
                                }
                            }
                        } else {
                            self.pc += 1;
                        }
                    }
                    _ => {
                        self.error_state = true;
                        self.error_message =
                            "Invalid instruction format, resulting from parse error".to_string();
                    }
                },
                Opcode::BGT => match self.get_branch_operation() {
                    Some((a, b, label)) => {
                        if a > b {
                            match self.labels.get(&label) {
                                Some(addr) => self.pc = *addr,
                                None => {
                                    self.error_state = true;
                                    self.error_message =
                                        "Nonexistent label, resulting from parse error".to_string();
                                }
                            }
                        } else {
                            self.pc += 1;
                        }
                    }
                    _ => {
                        self.error_state = true;
                        self.error_message =
                            "Invalid instruction format, resulting from parse error".to_string();
                    }
                },
                Opcode::BGE => match self.get_branch_operation() {
                    Some((a, b, label)) => {
                        if a >= b {
                            match self.labels.get(&label) {
                                Some(addr) => self.pc = *addr,
                                None => {
                                    self.error_state = true;
                                    self.error_message =
                                        "Nonexistent label, resulting from parse error".to_string();
                                }
                            }
                        } else {
                            self.pc += 1;
                        }
                    }
                    _ => {
                        self.error_state = true;
                        self.error_message =
                            "Invalid instruction format, resulting from parse error".to_string();
                    }
                },
                Opcode::JUMP => match inst.args.first() {
                    Some(Argument::LABEL(label)) => match self.labels.get(label) {
                        Some(addr) => self.pc = *addr,
                        None => {
                            self.error_state = true;
                            self.error_message =
                                "Nonexistent label, resulting from parse error".to_string();
                        }
                    },
                    _ => {
                        self.error_state = true;
                        self.error_message =
                            "Invalid instruction format, resulting from parse error".to_string();
                    }
                },
                Opcode::NOP => self.pc += 1,
                Opcode::NONE => {
                    self.error_state = true;
                    self.error_message =
                        "Unknown instruction, resulting from parse error".to_string();
                }
            },
            None => {
                self.error_state = true;
                self.error_message = "Program counter out of range".to_string();
                return;
            }
        }
    }
}

impl VirtualMachine {
    pub(crate) fn new() -> VirtualMachine {
        VirtualMachine {
            error_state: false,
            error_message: String::new(),
            registers: [0; 32],
            pc: 0,
            hi: 0,
            lo: 0,
            data_mem_back: 0,
            data_memory: vec![0; 1024],
            instruction_memory: Vec::new(),
            labels: Labels::new(),
        }
    }

    pub(crate) fn insert_label(&mut self, label: LabelType) {
        match label {
            LabelType::DATA(val) => self.labels.insert(val, self.data_mem_back),
            LabelType::INSTRUCTION(val) => self
                .labels
                .insert(val, self.instruction_memory.len() as u32),
        };
    }

    pub(crate) fn add_data(&mut self, layout: &String, value: &String) {
        let mut data_val: i64 = 0;
        let mut iterations: u32 = 0;

        match layout.as_str() {
            ".word" => {
                iterations = 4;
                data_val = value.parse::<i64>().expect("Invalid integer");
            }
            ".half" => {
                iterations = 2;
                data_val = value.parse::<i64>().expect("Invalid integer");
            }
            ".byte" => {
                iterations = 1;
                data_val = value.parse::<i64>().expect("Invalid integer");
            }
            ".space" => {
                self.data_mem_back += value.parse::<u32>().expect("Invalid integer");
            }
            ".ascii" => {
                for c in value.chars() {
                    self.data_memory[self.data_mem_back as usize] = c as u8;
                    self.data_mem_back += 1;
                }
            }
            ".asciiz" => {
                for c in value.chars() {
                    self.data_memory[self.data_mem_back as usize] = c as u8;
                    self.data_mem_back += 1;
                }
                self.data_memory[self.data_mem_back as usize] = '\0' as u8;
                self.data_mem_back += 1;
            }
            _ => panic!("Invalid data layout"),
        }

        data_val.to_le_bytes()[0..iterations as usize]
            .iter()
            .for_each(|byte| {
                self.data_memory[self.data_mem_back as usize] = *byte;
                self.data_mem_back += 1;
            });
    }

    pub(crate) fn add_instruction(&mut self, inst: Instruction) {
        self.instruction_memory.push(inst);
    }

    fn get_data_instruction(&self) -> Option<(&RegisterKind, u32)> {
        let inst = &self.instruction_memory[self.pc as usize];

        match (inst.args.first(), inst.args.get(1), inst.args.get(2)) {
            (Some(Argument::REGISTER(reg_s)), Some(Argument::REGISTER(reg_d)), None) => {
                Some((reg_s, self.registers[*reg_d as usize]))
            }
            (Some(Argument::REGISTER(reg_s)), Some(Argument::IMMEDIATE(imm)), None) => {
                Some((reg_s, *imm))
            }
            (Some(Argument::REGISTER(reg_s)), Some(Argument::LABEL(label)), None) => {
                Some((reg_s, *self.labels.get(label).unwrap()))
            }
            (
                Some(Argument::REGISTER(reg_s)),
                Some(Argument::OFFSET(offset)),
                Some(Argument::REGISTER(reg_d)),
            ) => Some((reg_s, self.registers[*reg_d as usize] + offset)),
            (
                Some(Argument::REGISTER(reg_s)),
                Some(Argument::OFFSET(offset)),
                Some(Argument::IMMEDIATE(imm)),
            ) => Some((reg_s, *imm + offset)),
            (
                Some(Argument::REGISTER(reg_s)),
                Some(Argument::OFFSET(offset)),
                Some(Argument::LABEL(label)),
            ) => Some((reg_s, *self.labels.get(label).unwrap() + offset)),
            _ => None,
        }
    }

    fn get_signed_instruction(&self) -> Option<(&RegisterKind, i32, i32)> {
        let inst = &self.instruction_memory[self.pc as usize];
        match (inst.args.first(), inst.args.get(1), inst.args.get(2)) {
            (
                Some(Argument::REGISTER(reg_d)),
                Some(Argument::REGISTER(reg_s1)),
                Some(Argument::REGISTER(reg_s2)),
            ) => Some((
                reg_d,
                self.get_register(*reg_s1) as i32,
                self.get_register(*reg_s2) as i32,
            )),
            (
                Some(Argument::REGISTER(reg_d)),
                Some(Argument::REGISTER(reg_s1)),
                Some(Argument::IMMEDIATE(imm)),
            ) => Some((reg_d, self.get_register(*reg_s1) as i32, *imm as i32)),
            _ => None,
        }
    }

    fn get_unsigned_instruction(&self) -> Option<(&RegisterKind, u32, u32)> {
        let inst = &self.instruction_memory[self.pc as usize];
        match (inst.args.first(), inst.args.get(1), inst.args.get(2)) {
            (
                Some(Argument::REGISTER(reg_d)),
                Some(Argument::REGISTER(reg_s1)),
                Some(Argument::REGISTER(reg_s2)),
            ) => Some((
                reg_d,
                self.get_register(*reg_s1),
                self.get_register(*reg_s2),
            )),
            (
                Some(Argument::REGISTER(reg_d)),
                Some(Argument::REGISTER(reg_s1)),
                Some(Argument::IMMEDIATE(imm)),
            ) => Some((reg_d, self.get_register(*reg_s1), *imm)),
            _ => None,
        }
    }

    fn get_branch_operation(&self) -> Option<(u32, u32, String)> {
        let inst = &self.instruction_memory[self.pc as usize];
        match (inst.args.first(), inst.args.get(1), inst.args.get(2)) {
            (
                Some(Argument::REGISTER(a)),
                Some(Argument::REGISTER(b)),
                Some(Argument::LABEL(label)),
            ) => Some((
                self.registers[*a as usize],
                self.registers[*b as usize],
                label.to_string(),
            )),
            (
                Some(Argument::REGISTER(a)),
                Some(Argument::IMMEDIATE(b)),
                Some(Argument::LABEL(label)),
            ) => Some((self.registers[*a as usize], *b, label.to_string())),
            _ => None,
        }
    }
}
