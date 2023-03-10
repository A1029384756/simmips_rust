#[cfg(test)]
use {
    crate::lex_parse::{
        lexer::tokenize,
        parser::parse_vm,
        virtual_machine_interface::{RegisterKind, VirtualMachineInterface},
    },
    std::{fs::read_to_string, path::PathBuf},
};

#[test]
fn vm_load_instructions() {
    {
        let data = ".data\ntest: .byte 10\n.text\nlb $t2, ($t0)\n";
        let (error, mut vm) = parse_vm(tokenize(data));

        assert!(!bool::from(error));
        assert_eq!(vm.get_instruction_size(), 1);
        assert_eq!(vm.get_memory_byte(0).unwrap(), 10);

        vm.step();
        assert_eq!(vm.get_register(RegisterKind::REG10), 10);
        assert!(!vm.is_error());

        vm.step();
        assert!(vm.is_error());
        assert!(!vm.get_error().is_empty());
    }
    {
        let data = ".data\ntest: .half 1300\n.text\nlh $t2, ($t0)\n";
        let (error, mut vm) = parse_vm(tokenize(data));

        assert!(!bool::from(error));
        assert_eq!(vm.get_instruction_size(), 1);
        assert_eq!(vm.get_memory_byte(0).unwrap(), 20);
        assert_eq!(vm.get_memory_byte(1).unwrap(), 5);

        vm.step();
        assert_eq!(vm.get_register(RegisterKind::REG10), 1300);
    }
    {
        let data = ".data\ntest: .word 300000\n.text\nlw $t2, ($t0)\n";
        let (error, mut vm) = parse_vm(tokenize(data));

        assert!(!bool::from(error));
        assert_eq!(vm.get_instruction_size(), 1);
        assert_eq!(vm.get_memory_byte(0).unwrap(), 224);
        assert_eq!(vm.get_memory_byte(1).unwrap(), 147);
        assert_eq!(vm.get_memory_byte(2).unwrap(), 4);

        vm.step();
        assert_eq!(vm.get_register(RegisterKind::REG10), 300000);
    }
}

#[test]
fn vm_load_invalid_mem() {
    {
        let data = ".data
        test: .word 30
        test2: .word 15
        .text
        lw $t1, 1024
    ";

        let (error, mut vm) = parse_vm(tokenize(data));

        assert!(!bool::from(error));
        assert_eq!(vm.get_instruction_size(), 1);

        vm.step();
        assert!(vm.is_error());
    }
    {
        let data = ".data
        test: .word 30
        test2: .word 15
        .text
        lw $t1, 1021
    ";

        let (error, mut vm) = parse_vm(tokenize(data));

        assert!(!bool::from(error));
        assert_eq!(vm.get_instruction_size(), 1);

        vm.step();
        assert!(vm.is_error());
    }
    {
        let data = ".data
        test: .word 30
        test2: .word 15
        .text
        lw $t1, 1020
    ";

        let (error, mut vm) = parse_vm(tokenize(data));

        assert!(!bool::from(error));
        assert_eq!(vm.get_instruction_size(), 1);

        vm.step();
        assert!(!vm.is_error());
    }
}

#[test]
fn vm_instructionless() {
    let data = ".data\ntest: .word 30 \n test2: .word 25\n";
    let (error, vm) = parse_vm(tokenize(data));

    assert!(!bool::from(error));
    assert_eq!(vm.get_current_source_line(), 0);
    assert!(matches!(vm.get_memory_byte(1024), None));
}

#[test]
fn vm_special_registers() {
    let data = ".text
      add $t0, $t0, 5
      mult $t0, $t0
    ";

    let (error, mut vm) = parse_vm(tokenize(data));
    assert!(!bool::from(error));

    while !vm.is_error() {
        vm.step();
    }

    assert_eq!(vm.get_register(RegisterKind::REG08), 5);
    assert_eq!(vm.get_register(RegisterKind::REGHI), 0);
    assert_eq!(vm.get_register(RegisterKind::REGLO), 25);
    assert_eq!(vm.get_register(RegisterKind::REGPC), 2);
}

#[test]
fn vm_jump() {
    let data = ".text
        initial:
        add $t0, $t0, 5
        j initial
    ";

    let (error, mut vm) = parse_vm(tokenize(data));
    assert!(!bool::from(error));
    vm.step();

    assert_eq!(vm.get_register(RegisterKind::REG08), 5);
    assert_eq!(vm.get_register(RegisterKind::REGPC), 1);
    vm.step();

    assert_eq!(vm.get_register(RegisterKind::REGPC), 0);
}

#[test]
fn vm_beq() {
    let data = ".text
      sum: add $t0, $t0, 5
      beq $t0, 5, sum ";

    let (error, mut vm) = parse_vm(tokenize(data));
    assert!(!bool::from(error));
    while !vm.is_error() {
        vm.step();
    }

    assert_eq!(vm.get_register(RegisterKind::REG08), 10);
}

#[test]
fn vm_bne() {
    let data = ".text
      sum: add $t0, $t0, 5
      bne $t0, 10, sum ";

    let (error, mut vm) = parse_vm(tokenize(data));
    assert!(!bool::from(error));
    while !vm.is_error() {
        vm.step();
    }

    assert_eq!(vm.get_register(RegisterKind::REG08), 10);
}

#[test]
fn vm_ble() {
    let data = ".text
      sum: add $t0, $t0, 5
      ble $t0, 10, sum ";

    let (error, mut vm) = parse_vm(tokenize(data));
    assert!(!bool::from(error));
    while !vm.is_error() {
        vm.step();
    }

    assert_eq!(vm.get_register(RegisterKind::REG08), 15);
}

#[test]
fn vm_load_offset() {
    let data = ".data
      str: .asciiz \"hello world!\"
      .text
      lb $t0, str
      lb $t1, 1(str)
      lb $t2, 2(str)
      lb $t3, 3(str)
      lb $t4, 4(str)";

    let (error, mut vm) = parse_vm(tokenize(data));

    assert!(!bool::from(error));
    assert_eq!(vm.get_memory_byte(0).unwrap(), 'h' as u8);
    assert_eq!(vm.get_memory_byte(1).unwrap(), 'e' as u8);
    assert_eq!(vm.get_memory_byte(2).unwrap(), 'l' as u8);
    assert_eq!(vm.get_memory_byte(3).unwrap(), 'l' as u8);
    assert_eq!(vm.get_memory_byte(4).unwrap(), 'o' as u8);

    vm.step();
    assert_eq!(vm.get_register(RegisterKind::REG08), 'h' as u32);
    vm.step();
    assert_eq!(vm.get_register(RegisterKind::REG09), 'e' as u32);
    vm.step();
    assert_eq!(vm.get_register(RegisterKind::REG10), 'l' as u32);
    vm.step();
    assert_eq!(vm.get_register(RegisterKind::REG11), 'l' as u32);
    vm.step();
    assert_eq!(vm.get_register(RegisterKind::REG12), 'o' as u32);
}

#[test]
fn vm_nop() {
    let data = ".text\nnop\n";

    let (error, mut vm) = parse_vm(tokenize(data));

    assert!(!bool::from(error));

    vm.step();
    assert_eq!(vm.get_register(RegisterKind::REGPC), 1);
}

#[test]
fn vm_store() {
    let data = ".text
        add $t0, $t0, 258
        sw $t0, 0";

    let (error, mut vm) = parse_vm(tokenize(data));

    assert!(!bool::from(error));
    vm.step();
    vm.step();

    assert_eq!(vm.get_register(RegisterKind::REG08), 258);
    assert_eq!(vm.get_memory_byte(0).unwrap(), 2);
    assert_eq!(vm.get_memory_byte(1).unwrap(), 1);
}

#[test]
fn vm_addition_range() {
    {
        let data = ".text
      add $t0, $t0, 2147483647
      add $t1, $t1, 1
      add $t0, $t1, $t0";

        let (error, mut vm) = parse_vm(tokenize(data));

        assert!(!bool::from(error));
        vm.step();
        vm.step();
        vm.step();

        assert!(vm.is_error());
    }
    {
        let data = ".text
      add $t0, $t0, -2147483648
      add $t1, $t1, -1
      add $t0, $t1, $t0";

        let (error, mut vm) = parse_vm(tokenize(data));

        assert!(!bool::from(error));
        vm.step();
        vm.step();
        vm.step();

        assert!(vm.is_error());
    }
}

#[test]
fn vm_test_09() {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("tests/vm/test09.asm");

    let data = &read_to_string(path).unwrap();
    let (error, mut vm) = parse_vm(tokenize(data));
    assert!(!bool::from(error));

    while !vm.is_error() {
        vm.step();
    }

    assert_eq!(vm.get_register(RegisterKind::REGHI), 4294967295);
}

#[test]
fn vm_test_11() {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("tests/vm/test11.asm");

    let data = &read_to_string(path).unwrap();
    let (error, mut vm) = parse_vm(tokenize(data));
    assert!(!bool::from(error));

    while !vm.is_error() {
        vm.step();
    }

    assert_eq!(vm.get_register(RegisterKind::REGPC), 21);
}

#[test]
fn vm_test_17() {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("tests/vm/test17.asm");

    let data = &read_to_string(path).unwrap();
    let (error, mut vm) = parse_vm(tokenize(data));
    assert!(!bool::from(error));

    while !vm.is_error() {
        vm.step();
    }

    assert_eq!(vm.get_register(RegisterKind::REGPC), 5);
    assert_eq!(vm.get_register(RegisterKind::REG10), 4294967292);
}
