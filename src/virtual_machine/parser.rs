use std::collections::BTreeMap;

use super::vm_defs::{Argument, LabelType};

use super::{
    token::Token,
    token::{TokenList, TokenType},
    util::get_valid_register,
    virtual_machine_interface::RegisterKind,
    virtualmachine::VirtualMachine,
    vm_defs::InstructionBuilder,
};

pub fn parse(tokens: TokenList) -> Result<(), String> {
    match parse_vm(tokens) {
        Ok(_) => Ok(()),
        Err(error) => Err(error),
    }
}

pub fn parse_vm(tokens: TokenList) -> Result<VirtualMachine, String> {
    let mut parser: Parser = Parser::new(&tokens);
    select_mode(&mut parser);

    if matches!(parser.mode, ParseMode::Error) {
        return Err(format!("Error:{}:section annotation", parser.get_line()));
    }

    while select_mode(&mut parser) {
        match parser.peek().get_value().as_str() {
            ".data" => continue,
            ".text" => continue,
            _ => (),
        };

        match parser.mode {
            ParseMode::Data => {
                if !parse_declaration(&mut parser) {
                    return Err(format!("Error:{}:invalid declaration", parser.get_line()));
                }
            }
            ParseMode::Text => {
                if !parse_instruction(&mut parser) {
                    return Err(format!("Error:{}:invalid instruction", parser.get_line()));
                }
            }
            ParseMode::Error => return Err(format!("Error:{}:unexpected EOF", parser.get_line())),
        };

        parser.advance();
    }

    Ok(parser.vm)
}

fn parse_declaration(parser: &mut Parser) -> bool {
    if parse_constant(parser) {
        parser.advance();
        matches!(parser.peek().get_type(), TokenType::Eol)
    } else if parse_label_declaration(parser) {
        parser.advance();
        return if matches!(parser.peek().get_type(), TokenType::Eol) {
            true
        } else if parse_layout(parser) {
            matches!(parser.peek().get_type(), TokenType::Eol)
        } else {
            false
        };
    } else if parse_layout(parser) {
        return matches!(parser.peek().get_type(), TokenType::Eol);
    } else {
        return false;
    }
}

fn parse_instruction(parser: &mut Parser) -> bool {
    if parse_label_declaration(parser) {
        parser.advance();
        if parse_operation(parser) {
            parser.advance();
            return matches!(parser.peek().get_type(), TokenType::Eol);
        }
        matches!(parser.peek().get_type(), TokenType::Eol)
    } else if parse_operation(parser) {
        parser.advance();
        return matches!(parser.peek().get_type(), TokenType::Eol);
    } else {
        return false;
    }
}

fn parse_operation(parser: &mut Parser) -> bool {
    parser.instruction.set_line(parser.get_line());
    if parse_data_movement(parser)
        || parse_int_arithmetic(parser)
        || parse_logical(parser)
        || parse_control(parser)
    {
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
    parser.instruction.set_opcode(&parser.peek().get_value());
    match parser.peek().get_value().as_str() {
        "beq" | "bne" | "blt" | "ble" | "bgt" | "bge" => {
            parser.advance();
            if !parse_reg_sep(parser) {
                return false;
            }
            parser.advance();
            if parse_register(parser) || parse_immediate(parser) {
                parser.advance();
            } else {
                return false;
            }
            if !matches!(parser.peek().get_type(), TokenType::Sep) {
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
    parser.instruction.set_opcode(&parser.peek().get_value());
    match parser.peek().get_value().as_str() {
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
    parser.instruction.set_opcode(&parser.peek().get_value());
    match parser.peek().get_value().as_str() {
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
            if matches!(parser.peek().get_type(), TokenType::Sep) {
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
    matches!(parser.peek().get_type(), TokenType::Sep)
}

fn parse_memref(parser: &mut Parser) -> bool {
    if parse_label_reference(parser)
        || parse_register(parser)
        || (!matches!(parser.get(parser.pos + 1).get_type(), TokenType::OpenParen)
            && parse_immediate(parser))
    {
        true
    } else {
        if parse_offset(parser) {
            parser.advance();
        }
        if !matches!(parser.peek().get_type(), TokenType::OpenParen) {
            return false;
        }
        parser.advance();

        if !parse_register(parser) && !parse_immediate(parser) && !parse_label_reference(parser) {
            return false;
        }
        parser.advance();

        matches!(parser.peek().get_type(), TokenType::CloseParen)
    }
}

fn parse_offset(parser: &mut Parser) -> bool {
    if layout_int_compat(".word", parser.peek().get_value().as_str()) {
        parser
            .instruction
            .add_arg(Argument::Offset(parser.peek().get_value().parse().unwrap()));
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
            .add_arg(Argument::Label(parser.peek().get_value().to_string()));
        true
    } else {
        false
    }
}

fn parse_data_movement(parser: &mut Parser) -> bool {
    parser
        .instruction
        .set_opcode(&parser.peek().get_value().to_string());

    match parser.peek().get_value().as_str() {
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
    if let Some(constant) = parser.constants.get(parser.peek().get_value().as_str()) {
        if layout_int_compat(".word", constant) {
            parser
                .instruction
                .add_arg(Argument::Immediate(constant.parse::<i64>().unwrap() as u32));
            true
        } else {
            false
        }
    } else if layout_int_compat(".word", parser.peek().get_value().as_str()) {
        parser.instruction.add_arg(Argument::Immediate(
            parser.peek().get_value().parse::<i64>().unwrap() as u32,
        ));
        true
    } else {
        false
    }
}

fn parse_register(parser: &mut Parser) -> bool {
    match get_valid_register(parser.peek().get_value().as_str()) {
        Some(RegisterKind::RegHi)
        | Some(RegisterKind::RegLo)
        | Some(RegisterKind::RegPC)
        | None => false,
        Some(reg) => {
            parser
                .instruction
                .add_arg(Argument::Register(reg.to_owned()));
            true
        }
    }
}

fn select_mode(parser: &mut Parser) -> bool {
    let next_idx: usize = parser.pos + 1;
    if parser.at_end() || next_idx >= parser.tokens.len() {
        return false;
    }

    match (
        parser.peek().get_value().as_str(),
        parser.get(next_idx).get_type(),
    ) {
        (".text", &TokenType::Eol) => {
            parser.mode = ParseMode::Text;
            parser.advance();
            parser.advance();
        }
        (".data", &TokenType::Eol) => {
            parser.mode = ParseMode::Data;
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
        if !layout_int_compat(&layout, parser.peek().get_value().as_str()) {
            match parser.constants.get(parser.peek().get_value().as_str()) {
                Some(constant) => {
                    if !layout_int_compat(&layout, constant) {
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
            return matches!(parser.peek().get_type(), TokenType::Eol);
        }

        while !parser.at_end() && !matches!(parser.peek().get_type(), TokenType::Eol) {
            if !matches!(parser.peek().get_type(), TokenType::Sep) {
                return false;
            }
            parser.advance();
            if !layout_int_compat(&layout, parser.peek().get_value().as_str()) {
                return false;
            }
            parser
                .vm
                .add_data(&layout, &parser.peek().get_value().to_string());
            parser.advance();
        }
        true
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
        return matches!(parser.peek().get_type(), TokenType::Eol);
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
        ParseMode::Data => {
            parser.vm.insert_label(LabelType::Data(label_trunc));
            true
        }
        ParseMode::Text => {
            parser.vm.insert_label(LabelType::Instruction(label_trunc));
            true
        }
        ParseMode::Error => false,
    }
}

fn parse_string(parser: &mut Parser) -> bool {
    if !matches!(parser.peek().get_type(), TokenType::StringDelim) {
        return false;
    }
    parser.advance();
    if !matches!(parser.peek().get_type(), TokenType::String) {
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
        parser.peek().get_value().as_str(),
        ".word" | ".byte" | ".half" | ".space"
    )
}

fn parse_string_layout(parser: &Parser) -> bool {
    matches!(parser.peek().get_value().as_str(), ".ascii" | ".asciiz")
}

fn layout_int_compat(layout: &str, value: &str) -> bool {
    let signed: bool = value.starts_with('+') || value.starts_with('-');
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

    if !matches!(parser.peek().get_type(), TokenType::Equal) {
        return false;
    }
    parser.advance();

    match parser.constants.get(parser.peek().get_value().as_str()) {
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
    tokens: TokenList,
    pos: usize,
    labels: Vec<String>,
    constants: BTreeMap<String, String>,
    mode: ParseMode,
    instruction: InstructionBuilder,
}

#[derive(Debug)]
enum ParseMode {
    Data,
    Text,
    Error,
}

impl Parser {
    fn new(t: &TokenList) -> Parser {
        let mut p: Parser = Parser {
            vm: VirtualMachine::new(),
            tokens: t.to_vec(),
            pos: 0,
            labels: Vec::new(),
            constants: BTreeMap::new(),
            mode: ParseMode::Error,
            instruction: InstructionBuilder::new(),
        };

        for token in t {
            if p.labels.contains(&token.get_value().to_string()) {
                continue;
            }

            if let (Some('A'..='z'), Some(':')) = (
                token.get_value().chars().next(),
                token.get_value().chars().last(),
            ) {
                if token.get_value()[0..token.get_value().len() - 1]
                    .chars()
                    .all(char::is_alphanumeric)
                {
                    p.labels
                        .push(token.get_value()[0..token.get_value().len() - 1].to_string())
                }
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
