#![allow(dead_code)]

pub mod alu;
pub mod control_unit;
pub mod cpu_interface;
pub mod data_memory;
pub mod instruction_memory;
pub mod registers;
pub mod single_cycle_cpu;

const INST_MEM_START: u32 = 0x00400000;
const DATA_MEM_START: u32 = 0x10010000;

const BEQ_OPCODE: u32 = 0x04;
const BNE_OPCODE: u32 = 0x05;

const ADD_FUNCT: u32 = 0x20;
const ADDU_FUNCT: u32 = 0x21;
const AND_FUNCT: u32 = 0x24;
const JR_FUNCT: u32 = 0x08;
const NOR_FUNCT: u32 = 0x27;
const OR_FUNCT: u32 = 0x25;
const SLT_FUNCT: u32 = 0x2A;
const SLTU_FUNCT: u32 = 0x2B;
const SLL_FUNCT: u32 = 0x00;
const SRL_FUNCT: u32 = 0x02;
const SUB_FUNCT: u32 = 0x22;
const SUBU_FUNCT: u32 = 0x23;
