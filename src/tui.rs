mod lex_parse;
use crate::lex_parse::util::get_valid_register;
use lex_parse::virtual_machine_interface::RegisterKind;
use lex_parse::virtual_machine_interface::VirtualMachineInterface;
use lex_parse::virtualmachine::VirtualMachine;
use lex_parse::{lexer::tokenize, parser::parse_vm};
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

fn get_valid_address(addr: &str) -> Option<u32> {
    match addr.chars().next() {
        Some('&') => match addr.starts_with("&0x") {
            true => match u32::from_str_radix(&addr[3..], 16) {
                Ok(val) => Some(val),
                Err(_) => None,
            },
            false => match str::parse::<u32>(&addr[1..]) {
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
        println!("0x{:08x}", vm.get_register(RegisterKind::RegPC));
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
        Ok(asm_file) => match tokenize(&asm_file) {
            Ok(tokens) => match parse_vm(tokens) {
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
                            break;
                        }

                        repl.handle_event(&event);
                    }
                    ExitCode::SUCCESS
                }
                Err(error) => {
                    println!("{}", error);
                    ExitCode::FAILURE
                }
            },
            Err(error) => {
                println!("{}", error);
                ExitCode::FAILURE
            }
        },
        Err(..) => {
            println!("Error: File not found");
            ExitCode::FAILURE
        }
    }
}
