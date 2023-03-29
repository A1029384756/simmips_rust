mod lex_parse;
use lex_parse::virtual_machine_interface::RegisterKind;
use lex_parse::virtual_machine_interface::VirtualMachineInterface;
use lex_parse::virtualmachine::VirtualMachine;
use lex_parse::{lexer::tokenize, parser::parse_vm, token::TokenType};
use std::collections::HashMap;
use std::env;
use std::io::stdin;
use std::io::stdout;
use std::io::Write;
use std::process::ExitCode;

struct Repl {
    events: HashMap<String, fn(&str, &mut VirtualMachine)>,
    vm: VirtualMachine,
}

impl Repl {
    fn new(vm: VirtualMachine) -> Repl {
        Repl {
            events: HashMap::new(),
            vm,
        }
    }

    fn add_handler(&mut self, event: &str, callback: fn(&str, &mut VirtualMachine)) {
        self.events.insert(event.to_string(), callback);
    }

    fn handle_event(&mut self, event: &str) -> bool {
        match self.events.get(event.split_whitespace().next().unwrap()) {
            Some(operation) => {
                operation(event, &mut self.vm);
                true
            }
            None => {
                println!("Error: unknown command");
                false
            }
        }
    }
}

fn get_valid_register(reg: &str) -> Option<RegisterKind> {
    match reg {
        "$0" => Some(RegisterKind::REG00),
        "$1" => Some(RegisterKind::REG01),
        "$2" => Some(RegisterKind::REG02),
        "$3" => Some(RegisterKind::REG03),
        "$4" => Some(RegisterKind::REG04),
        "$5" => Some(RegisterKind::REG05),
        "$6" => Some(RegisterKind::REG06),
        "$7" => Some(RegisterKind::REG07),
        "$8" => Some(RegisterKind::REG08),
        "$9" => Some(RegisterKind::REG09),
        "$10" => Some(RegisterKind::REG10),
        "$11" => Some(RegisterKind::REG11),
        "$12" => Some(RegisterKind::REG12),
        "$13" => Some(RegisterKind::REG13),
        "$14" => Some(RegisterKind::REG14),
        "$15" => Some(RegisterKind::REG15),
        "$16" => Some(RegisterKind::REG16),
        "$17" => Some(RegisterKind::REG17),
        "$18" => Some(RegisterKind::REG18),
        "$19" => Some(RegisterKind::REG19),
        "$20" => Some(RegisterKind::REG20),
        "$21" => Some(RegisterKind::REG21),
        "$22" => Some(RegisterKind::REG22),
        "$23" => Some(RegisterKind::REG23),
        "$24" => Some(RegisterKind::REG24),
        "$25" => Some(RegisterKind::REG25),
        "$26" => Some(RegisterKind::REG26),
        "$27" => Some(RegisterKind::REG27),
        "$28" => Some(RegisterKind::REG28),
        "$29" => Some(RegisterKind::REG29),
        "$30" => Some(RegisterKind::REG30),
        "$31" => Some(RegisterKind::REG31),
        "$zero" => Some(RegisterKind::REG00),
        "$at" => Some(RegisterKind::REG01),
        "$v0" => Some(RegisterKind::REG02),
        "$v1" => Some(RegisterKind::REG03),
        "$a0" => Some(RegisterKind::REG04),
        "$a1" => Some(RegisterKind::REG05),
        "$a2" => Some(RegisterKind::REG06),
        "$a3" => Some(RegisterKind::REG07),
        "$t0" => Some(RegisterKind::REG08),
        "$t1" => Some(RegisterKind::REG09),
        "$t2" => Some(RegisterKind::REG10),
        "$t3" => Some(RegisterKind::REG11),
        "$t4" => Some(RegisterKind::REG12),
        "$t5" => Some(RegisterKind::REG13),
        "$t6" => Some(RegisterKind::REG14),
        "$t7" => Some(RegisterKind::REG15),
        "$s0" => Some(RegisterKind::REG16),
        "$s1" => Some(RegisterKind::REG17),
        "$s2" => Some(RegisterKind::REG18),
        "$s3" => Some(RegisterKind::REG19),
        "$s4" => Some(RegisterKind::REG20),
        "$s5" => Some(RegisterKind::REG21),
        "$s6" => Some(RegisterKind::REG22),
        "$s7" => Some(RegisterKind::REG23),
        "$t8" => Some(RegisterKind::REG24),
        "$t9" => Some(RegisterKind::REG25),
        "$k0" => Some(RegisterKind::REG26),
        "$k1" => Some(RegisterKind::REG27),
        "$gp" => Some(RegisterKind::REG28),
        "$sp" => Some(RegisterKind::REG29),
        "$fp" => Some(RegisterKind::REG30),
        "$ra" => Some(RegisterKind::REG31),
        "$pc" => Some(RegisterKind::REGPC),
        "$hi" => Some(RegisterKind::REGHI),
        "$lo" => Some(RegisterKind::REGLO),
        _ => None,
    }
}

fn get_valid_address(addr: &str) -> Option<u32> {
    match addr.chars().next() {
        Some('&') => match &addr[1..=2] {
            "0x" => match u32::from_str_radix(&addr[3..], 16) {
                Ok(val) => Some(val),
                Err(_) => None,
            },
            _ => match u32::from_str_radix(&addr[1..], 10) {
                Ok(val) => Some(val),
                Err(_) => None,
            },
        },
        _ => None,
    }
}

fn handle_print(event: &str, vm: &mut VirtualMachine) {
    let reg_addr = event.split_whitespace().last().unwrap();
    match (get_valid_register(reg_addr), get_valid_address(reg_addr)) {
        (Some(register), None) => println!("0x{:08x}", vm.get_register(register)),
        (None, Some(address)) => match vm.get_memory_byte(address) {
            Some(byte) => println!("0x{:02x?}", byte),
            None => println!("Error: out of range memory address"),
        },
        _ => println!("Error: malformed print instruction"),
    }
}

fn handle_step(_: &str, vm: &mut VirtualMachine) {
    vm.step();
    if vm.is_error() {
        println!("{}", vm.get_error());
    } else {
        println!("0x{:08x}", vm.get_register(RegisterKind::REGPC));
    }
}

fn handle_status(_: &str, vm: &mut VirtualMachine) {
    if vm.is_error() {
        println!("{}", vm.get_error());
    }
}

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Error: Invalid number of arguments");
        return ExitCode::FAILURE;
    }

    match std::fs::read_to_string(&args[1]) {
        Ok(asm_file) => {
            let tokens = tokenize(&asm_file);

            if tokens.last().unwrap().get_type() == &TokenType::ERROR {
                println!("{}", tokens.last().unwrap().get_value());
                return ExitCode::FAILURE;
            }

            match parse_vm(tokens) {
                Ok(vm) => {
                    let mut repl = Repl::new(vm);
                    repl.add_handler("print", handle_print);
                    repl.add_handler("step", handle_step);
                    repl.add_handler("status", handle_status);

                    loop {
                        print!("simmips> ");
                        let _ = stdout().flush();
                        let mut event = String::new();
                        stdin().read_line(&mut event).unwrap();

                        if event == "quit\n" {
                            return ExitCode::SUCCESS;
                        }

                        repl.handle_event(&event);
                    }
                }
                Err(error) => {
                    println!("{}", error.message());
                    return ExitCode::FAILURE;
                }
            }
        }
        Err(..) => {
            println!("Error: File not found");
            return ExitCode::FAILURE;
        }
    }
}
