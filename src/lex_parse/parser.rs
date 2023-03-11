use std::collections::BTreeMap;

use crate::lex_parse::vm_defs::{Argument, LabelType};

use super::{
    token::Token,
    token::{TokenList, TokenType},
    virtual_machine_interface::RegisterKind,
    virtualmachine::VirtualMachine,
    vm_defs::InstructionBuilder,
};

#[derive(Debug, Default)]
pub struct ParseError {
    has_error: bool,
    error_contents: String,
}

impl ParseError {
    pub fn new() -> ParseError {
        ParseError {
            has_error: false,
            error_contents: String::new(),
        }
    }

    pub fn set(&mut self, message: &str, line: u32) -> () {
        self.has_error = true;
        self.error_contents = format!("Error:{}: {}", line, message.to_string());
    }

    pub fn message(&self) -> String {
        self.error_contents.to_string()
    }
}

impl From<ParseError> for bool {
    fn from(parse_error: ParseError) -> bool {
        parse_error.has_error
    }
}

pub fn parse(tokens: TokenList) -> ParseError {
    parse_vm(tokens).0
}

pub fn parse_vm(tokens: TokenList) -> (ParseError, VirtualMachine) {
    let mut parser: Parser = Parser::new(&tokens);
    select_mode(&mut parser);

    if matches!(parser.mode, ParseMode::ERROR) {
        parser
            .error
            .set("invalid section annotation", parser.get_line());
        return (parser.error, parser.vm);
    }

    while select_mode(&mut parser) {
        match parser.peek().get_value() {
            ".data" => continue,
            ".text" => continue,
            _ => (),
        };

        match parser.mode {
            ParseMode::DATA => {
                if !parse_declaration(&mut parser) {
                    parser.error.set("invalid declaration", parser.get_line());
                    return (parser.error, parser.vm);
                }
            }
            ParseMode::TEXT => {
                if !parse_instruction(&mut parser) {
                    parser.error.set("invalid instruction", parser.get_line());
                    return (parser.error, parser.vm);
                }
            }
            ParseMode::ERROR => return (parser.error, parser.vm),
        };

        parser.advance();
    }

    (parser.error, parser.vm)
}

fn parse_declaration(parser: &mut Parser) -> bool {
    if parse_constant(parser) {
        parser.advance();
        return matches!(parser.peek().get_type(), TokenType::EOL);
    } else if parse_label_declaration(parser) {
        parser.advance();
        if matches!(parser.peek().get_type(), TokenType::EOL) {
            return true;
        } else if parse_layout(parser) {
            return matches!(parser.peek().get_type(), TokenType::EOL);
        } else {
            return false;
        }
    } else if parse_layout(parser) {
        return matches!(parser.peek().get_type(), TokenType::EOL);
    } else {
        return false;
    }
}

fn parse_instruction(parser: &mut Parser) -> bool {
    if parse_label_declaration(parser) {
        parser.advance();
        if parse_operation(parser) {
            parser.advance();
            return matches!(parser.peek().get_type(), TokenType::EOL);
        }
        return matches!(parser.peek().get_type(), TokenType::EOL);
    } else if parse_operation(parser) {
        parser.advance();
        return matches!(parser.peek().get_type(), TokenType::EOL);
    } else {
        return false;
    }
}

fn parse_operation(parser: &mut Parser) -> bool {
    parser.instruction.set_line(parser.get_line());
    if parse_data_movement(parser) {
        parser.vm.add_instruction(parser.instruction.get_inst());
        true
    } else if parse_int_arithmetic(parser) {
        parser.vm.add_instruction(parser.instruction.get_inst());
        true
    } else if parse_logical(parser) {
        parser.vm.add_instruction(parser.instruction.get_inst());
        true
    } else if parse_control(parser) {
        parser.vm.add_instruction(parser.instruction.get_inst());
        true
    } else if parser.peek().get_value() == "nop" {
        parser.instruction.set_opcode("nop");
        parser.vm.add_instruction(parser.instruction.get_inst());
        true
    } else {
        false
    }
}

fn parse_control(parser: &mut Parser) -> bool {
    parser
        .instruction
        .set_opcode(&parser.peek().get_value().to_string());
    match parser.peek().get_value() {
        "beq" | "bne" | "blt" | "ble" | "bgt" | "bge" => {
            parser.advance();
            if !parse_reg_sep(parser) {
                return false;
            }
            parser.advance();
            if parse_register(parser) {
                parser.advance();
            } else if parse_immediate(parser) {
                parser.advance();
            } else {
                return false;
            }
            if !matches!(parser.peek().get_type(), TokenType::SEP) {
                return false;
            }
            parser.advance();
            parse_label_reference(parser)
        }
        "j" => {
            parser.advance();
            parse_label_reference(parser)
        }
        _ => false,
    }
}

fn parse_logical(parser: &mut Parser) -> bool {
    parser
        .instruction
        .set_opcode(&parser.peek().get_value().to_string());
    match parser.peek().get_value() {
        "and" | "nor" | "or" | "xor" => {
            parser.advance();
            if !parse_reg_sep(parser) {
                return false;
            }
            parser.advance();
            if !parse_reg_sep(parser) {
                return false;
            }
            parser.advance();
            parse_register(parser) || parse_immediate(parser)
        }
        "not" => {
            parser.advance();
            if !parse_reg_sep(parser) {
                return false;
            }
            parser.advance();
            parse_register(parser) || parse_immediate(parser)
        }
        _ => false,
    }
}

fn parse_int_arithmetic(parser: &mut Parser) -> bool {
    parser
        .instruction
        .set_opcode(&parser.peek().get_value().to_owned());
    match parser.peek().get_value() {
        "add" | "addu" | "sub" | "subu" | "mul" | "mulo" | "mulou" | "rem" | "remu" => {
            parser.advance();
            if !parse_reg_sep(parser) {
                return false;
            }
            parser.advance();
            if !parse_reg_sep(parser) {
                return false;
            }
            parser.advance();
            parse_register(parser) || parse_immediate(parser)
        }
        "mult" | "multu" | "abs" | "neg" | "negu" => {
            parser.advance();
            if !parse_reg_sep(parser) {
                return false;
            }
            parser.advance();
            parse_register(parser)
        }
        "div" | "divu" => {
            parser.advance();
            if !parse_reg_sep(parser) {
                return false;
            }
            parser.advance();
            if !parse_register(parser) {
                return false;
            }
            parser.advance();
            if matches!(parser.peek().get_type(), TokenType::SEP) {
                parser.advance();
                parse_register(parser) || parse_immediate(parser)
            } else {
                parser.pos -= 1;
                true
            }
        }
        _ => false,
    }
}

fn parse_reg_sep(parser: &mut Parser) -> bool {
    if !parse_register(parser) {
        return false;
    }
    parser.advance();
    matches!(parser.peek().get_type(), TokenType::SEP)
}

fn parse_memref(parser: &mut Parser) -> bool {
    if parse_label_reference(parser) {
        true
    } else if parse_register(parser) {
        true
    } else if !matches!(parser.get(parser.pos + 1).get_type(), TokenType::OPENPAREN)
        && parse_immediate(parser)
    {
        true
    } else {
        if parse_offset(parser) {
            parser.advance();
        }
        if !matches!(parser.peek().get_type(), TokenType::OPENPAREN) {
            return false;
        }
        parser.advance();

        if !parse_register(parser) && !parse_immediate(parser) && !parse_label_reference(parser) {
            return false;
        }
        parser.advance();

        matches!(parser.peek().get_type(), TokenType::CLOSEPAREN)
    }
}

fn parse_offset(parser: &mut Parser) -> bool {
    if layout_int_compat(".word", parser.peek().get_value()) {
        parser
            .instruction
            .add_arg(Argument::OFFSET(parser.peek().get_value().parse().unwrap()));
        true
    } else {
        false
    }
}

fn parse_label_reference(parser: &mut Parser) -> bool {
    if parser
        .labels
        .contains(&parser.peek().get_value().to_owned())
    {
        parser
            .instruction
            .add_arg(Argument::LABEL(parser.peek().get_value().to_string()));
        true
    } else {
        false
    }
}

fn parse_data_movement(parser: &mut Parser) -> bool {
    parser.instruction.set_line(parser.get_line());
    parser
        .instruction
        .set_opcode(&parser.peek().get_value().to_string());

    match parser.peek().get_value() {
        "mfhi" | "mflo" | "mthi" | "mtlo" => {
            parser.advance();
            parse_register(parser)
        }
        "lw" | "lh" | "lb" | "la" | "sw" | "sh" | "sb" => {
            parser.advance();
            if !parse_reg_sep(parser) {
                return false;
            }
            parser.advance();
            parse_memref(parser)
        }
        "move" => {
            parser.advance();
            if !parse_reg_sep(parser) {
                return false;
            }
            parser.advance();
            parse_register(parser)
        }
        "li" => {
            parser.advance();
            if !parse_reg_sep(parser) {
                return false;
            }
            parser.advance();
            parse_immediate(parser)
        }
        _ => false,
    }
}

fn parse_immediate(parser: &mut Parser) -> bool {
    if let Some(constant) = parser.constants.get(parser.peek().get_value()) {
        if layout_int_compat(".word", constant) {
            parser
                .instruction
                .add_arg(Argument::IMMEDIATE(constant.parse::<i64>().unwrap() as u32));
            true
        } else {
            false
        }
    } else if layout_int_compat(".word", parser.peek().get_value()) {
        parser.instruction.add_arg(Argument::IMMEDIATE(
            parser.peek().get_value().parse::<i64>().unwrap() as u32,
        ));
        true
    } else {
        false
    }
}

fn parse_register(parser: &mut Parser) -> bool {
    let registers: BTreeMap<&str, RegisterKind> = [
        ("$0", RegisterKind::REG00),
        ("$1", RegisterKind::REG01),
        ("$2", RegisterKind::REG02),
        ("$3", RegisterKind::REG03),
        ("$4", RegisterKind::REG04),
        ("$5", RegisterKind::REG05),
        ("$6", RegisterKind::REG06),
        ("$7", RegisterKind::REG07),
        ("$8", RegisterKind::REG08),
        ("$9", RegisterKind::REG09),
        ("$10", RegisterKind::REG10),
        ("$11", RegisterKind::REG11),
        ("$12", RegisterKind::REG12),
        ("$13", RegisterKind::REG13),
        ("$14", RegisterKind::REG14),
        ("$15", RegisterKind::REG15),
        ("$16", RegisterKind::REG16),
        ("$17", RegisterKind::REG17),
        ("$18", RegisterKind::REG18),
        ("$19", RegisterKind::REG19),
        ("$20", RegisterKind::REG20),
        ("$21", RegisterKind::REG21),
        ("$22", RegisterKind::REG22),
        ("$23", RegisterKind::REG23),
        ("$24", RegisterKind::REG24),
        ("$25", RegisterKind::REG25),
        ("$26", RegisterKind::REG26),
        ("$27", RegisterKind::REG27),
        ("$28", RegisterKind::REG28),
        ("$29", RegisterKind::REG29),
        ("$30", RegisterKind::REG30),
        ("$31", RegisterKind::REG31),
        ("$zero", RegisterKind::REG00),
        ("$at", RegisterKind::REG01),
        ("$v0", RegisterKind::REG02),
        ("$v1", RegisterKind::REG03),
        ("$a0", RegisterKind::REG04),
        ("$a1", RegisterKind::REG05),
        ("$a2", RegisterKind::REG06),
        ("$a3", RegisterKind::REG07),
        ("$t0", RegisterKind::REG08),
        ("$t1", RegisterKind::REG09),
        ("$t2", RegisterKind::REG10),
        ("$t3", RegisterKind::REG11),
        ("$t4", RegisterKind::REG12),
        ("$t5", RegisterKind::REG13),
        ("$t6", RegisterKind::REG14),
        ("$t7", RegisterKind::REG15),
        ("$s0", RegisterKind::REG16),
        ("$s1", RegisterKind::REG17),
        ("$s2", RegisterKind::REG18),
        ("$s3", RegisterKind::REG19),
        ("$s4", RegisterKind::REG20),
        ("$s5", RegisterKind::REG21),
        ("$s6", RegisterKind::REG22),
        ("$s7", RegisterKind::REG23),
        ("$t8", RegisterKind::REG24),
        ("$t9", RegisterKind::REG25),
        ("$k0", RegisterKind::REG26),
        ("$k1", RegisterKind::REG27),
        ("$gp", RegisterKind::REG28),
        ("$sp", RegisterKind::REG29),
        ("$fp", RegisterKind::REG30),
        ("$ra", RegisterKind::REG31),
    ]
    .into();

    match registers.get(parser.peek().get_value()) {
        Some(reg) => {
            parser
                .instruction
                .add_arg(Argument::REGISTER(reg.to_owned()));
            true
        }
        None => false,
    }
}

fn select_mode(parser: &mut Parser) -> bool {
    let next_idx: usize = parser.pos + 1;
    if parser.at_end() || next_idx >= parser.tokens.len() {
        return false;
    }

    match (parser.peek().get_value(), parser.get(next_idx).get_type()) {
        (".text", &TokenType::EOL) => {
            parser.mode = ParseMode::TEXT;
            parser.advance();
            parser.advance();
        }
        (".data", &TokenType::EOL) => {
            parser.mode = ParseMode::DATA;
            parser.advance();
            parser.advance();
        }
        (_, _) => (),
    };

    !parser.at_end()
}

fn parse_layout(parser: &mut Parser) -> bool {
    if parse_int_layout(parser) {
        let layout: String = parser.peek().get_value().to_string();
        parser.advance();
        if !layout_int_compat(&layout, parser.peek().get_value()) {
            match parser.constants.get(parser.peek().get_value()) {
                Some(constant) => {
                    if !layout_int_compat(&layout, &constant) {
                        return false;
                    }
                    parser.vm.add_data(&layout, constant);
                }
                None => return false,
            }
        } else {
            parser
                .vm
                .add_data(&layout, &parser.peek().get_value().to_string());
        }
        parser.advance();
        if layout == ".space" {
            return matches!(parser.peek().get_type(), TokenType::EOL);
        }

        while !parser.at_end() && !matches!(parser.peek().get_type(), TokenType::EOL) {
            if !matches!(parser.peek().get_type(), TokenType::SEP) {
                return false;
            }
            parser.advance();
            if !layout_int_compat(&layout, parser.peek().get_value()) {
                return false;
            }
            parser
                .vm
                .add_data(&layout, &parser.peek().get_value().to_string());
            parser.advance();
        }
        return true;
    } else if parse_string_layout(parser) {
        let layout: String = parser.peek().get_value().to_string();
        parser.advance();
        if !parse_string(parser) {
            return false;
        }
        parser
            .vm
            .add_data(&layout, &parser.get(parser.pos - 1).get_value().to_string());
        parser.advance();
        return matches!(parser.peek().get_type(), TokenType::EOL);
    } else {
        return false;
    }
}

fn parse_label_declaration(parser: &mut Parser) -> bool {
    if parser.peek().get_value().is_empty() {
        return false;
    }

    let label_trunc: String =
        parser.peek().get_value()[0..parser.peek().get_value().len() - 1].to_string();

    if !parser.labels.contains(&label_trunc) {
        return false;
    }

    match parser.mode {
        ParseMode::DATA => {
            parser.vm.insert_label(LabelType::DATA(label_trunc));
            true
        }
        ParseMode::TEXT => {
            parser.vm.insert_label(LabelType::INSTRUCTION(label_trunc));
            true
        }
        ParseMode::ERROR => false,
    }
}

fn parse_string(parser: &mut Parser) -> bool {
    if !matches!(parser.peek().get_type(), TokenType::STRINGDELIM) {
        return false;
    }
    parser.advance();
    if !matches!(parser.peek().get_type(), TokenType::STRING) {
        return false;
    }
    if !parser.peek().get_value().is_ascii() {
        return false;
    }
    parser.advance();
    true
}

fn parse_int_layout(parser: &Parser) -> bool {
    matches!(
        parser.peek().get_value(),
        ".word" | ".byte" | ".half" | ".space"
    )
}

fn parse_string_layout(parser: &Parser) -> bool {
    matches!(parser.peek().get_value(), ".ascii" | ".asciiz")
}

fn layout_int_compat(layout: &str, value: &str) -> bool {
    let signed: bool = value.starts_with("+") || value.starts_with("-");
    match value.parse::<i64>() {
        Ok(val) => match layout {
            ".word" => {
                if signed {
                    val >= i32::MIN as i64 && val <= i32::MAX as i64
                } else {
                    val <= u32::MAX as i64
                }
            }
            ".half" => {
                if signed {
                    val >= i16::MIN as i64 && val <= i16::MAX as i64
                } else {
                    val <= u16::MAX as i64
                }
            }
            ".byte" => {
                if signed {
                    val >= i8::MIN as i64 && val <= i8::MAX as i64
                } else {
                    val <= u8::MAX as i64
                }
            }
            ".space" => {
                if signed {
                    val.is_positive() && val <= i32::MAX as i64
                } else {
                    val <= u32::MAX as i64
                }
            }
            _ => false,
        },
        Err(..) => false,
    }
}

fn parse_constant(parser: &mut Parser) -> bool {
    if parser.peek().get_value().is_empty() {
        return false;
    }

    if !parser.peek().get_value().starts_with(char::is_alphabetic) {
        return false;
    }

    if !parser.peek().get_value().chars().all(char::is_alphanumeric) {
        return false;
    }

    let const_name: String = parser.peek().get_value().to_string();
    parser.advance();

    if !matches!(parser.peek().get_type(), TokenType::EQUAL) {
        return false;
    }
    parser.advance();

    match parser.constants.get(parser.peek().get_value()) {
        Some(..) => true,
        None => match parser.peek().get_value().parse::<i64>() {
            Ok(const_val) => {
                parser
                    .constants
                    .insert(const_name.to_string(), const_val.to_string());
                true
            }
            Err(..) => false,
        },
    }
}

#[derive(Debug)]
struct Parser {
    vm: VirtualMachine,
    error: ParseError,
    tokens: TokenList,
    pos: usize,
    labels: Vec<String>,
    constants: BTreeMap<String, String>,
    mode: ParseMode,
    instruction: InstructionBuilder,
}

#[derive(Debug)]
enum ParseMode {
    DATA,
    TEXT,
    ERROR,
}

impl Parser {
    fn new(t: &TokenList) -> Parser {
        let mut p: Parser = Parser {
            vm: VirtualMachine::new(),
            error: ParseError::new(),
            tokens: t.to_vec(),
            pos: 0,
            labels: Vec::new(),
            constants: BTreeMap::new(),
            mode: ParseMode::ERROR,
            instruction: InstructionBuilder::new(),
        };

        for token in t {
            if p.labels.contains(&token.get_value().to_string()) {
                continue;
            }

            match (
                token.get_value().chars().next(),
                token.get_value().chars().last(),
            ) {
                (Some('A'..='z'), Some(':')) => {
                    if token.get_value()[0..token.get_value().len() - 1]
                        .chars()
                        .all(char::is_alphanumeric)
                    {
                        p.labels
                            .push(token.get_value()[0..token.get_value().len() - 1].to_string())
                    }
                }
                (_, _) => (),
            };
        }

        p
    }

    fn at_end(&self) -> bool {
        self.pos >= self.tokens.len()
    }

    fn peek(&self) -> &Token {
        self.tokens.get(self.pos).unwrap()
    }

    fn get(&self, idx: usize) -> &Token {
        self.tokens.get(idx).unwrap()
    }

    fn get_line(&self) -> u32 {
        match self.tokens.get(self.pos) {
            Some(token) => *token.get_line(),
            None => 0,
        }
    }

    fn advance(&mut self) {
        self.pos += 1
    }
}
