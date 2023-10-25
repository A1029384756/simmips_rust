#[cfg(test)]
use {
    super::{
        lexer::tokenize,
        parser::parse_vm,
        virtual_machine_interface::{RegisterKind, CPUInterface},
    },
    std::{fs::read_to_string, path::PathBuf},
};

#[test]
fn vm_load_instructions() {
    {
        let data = ".data\ntest: .byte 10\n.text\nlb $t2, ($t0)\n";
        let mut vm = parse_vm(tokenize(data).unwrap()).unwrap();

        assert_eq!(vm.get_instruction_size(), 1);
        assert_eq!(vm.get_memory_byte(0).unwrap(), 10);

        vm.step();
        assert_eq!(vm.get_register(RegisterKind::Reg10), 10);
        assert!(!vm.is_error());

        vm.step();
        assert!(vm.is_error());
        assert!(!vm.get_error().is_empty());
    }
    {
        let data = ".data\ntest: .half 1300\n.text\nlh $t2, ($t0)\n";
        let mut vm = parse_vm(tokenize(data).unwrap()).unwrap();

        assert_eq!(vm.get_instruction_size(), 1);
        assert_eq!(vm.get_memory_byte(0).unwrap(), 20);
        assert_eq!(vm.get_memory_byte(1).unwrap(), 5);

        vm.step();
        assert_eq!(vm.get_register(RegisterKind::Reg10), 1300);
    }
    {
        let data = ".data\ntest: .word 300000\n.text\nlw $t2, ($t0)\n";
        let mut vm = parse_vm(tokenize(data).unwrap()).unwrap();

        assert_eq!(vm.get_instruction_size(), 1);
        assert_eq!(vm.get_memory_byte(0).unwrap(), 224);
        assert_eq!(vm.get_memory_byte(1).unwrap(), 147);
        assert_eq!(vm.get_memory_byte(2).unwrap(), 4);

        vm.step();
        assert_eq!(vm.get_register(RegisterKind::Reg10), 300000);
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

        let mut vm = parse_vm(tokenize(data).unwrap()).unwrap();

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

        let mut vm = parse_vm(tokenize(data).unwrap()).unwrap();

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

        let mut vm = parse_vm(tokenize(data).unwrap()).unwrap();

        assert_eq!(vm.get_instruction_size(), 1);

        vm.step();
        assert!(!vm.is_error());
    }
}

#[test]
fn vm_instructionless() {
    let data = ".data\ntest: .word 30 \n test2: .word 25\n";
    let vm = parse_vm(tokenize(data).unwrap()).unwrap();

    assert_eq!(vm.get_current_source_line(), 0);
    assert!(vm.get_memory_byte(1024).is_none());
}

#[test]
fn vm_special_registers() {
    let data = ".text
      add $t0, $t0, 5
      mult $t0, $t0
    ";

    let mut vm = parse_vm(tokenize(data).unwrap()).unwrap();

    while !vm.is_error() {
        vm.step();
    }

    assert_eq!(vm.get_register(RegisterKind::Reg08), 5);
    assert_eq!(vm.get_register(RegisterKind::RegHi), 0);
    assert_eq!(vm.get_register(RegisterKind::RegLo), 25);
    assert_eq!(vm.get_register(RegisterKind::RegPC), 2);
}

#[test]
fn vm_jump() {
    let data = ".text
        initial:
        add $t0, $t0, 5
        j initial
    ";

    let mut vm = parse_vm(tokenize(data).unwrap()).unwrap();
    vm.step();

    assert_eq!(vm.get_register(RegisterKind::Reg08), 5);
    assert_eq!(vm.get_register(RegisterKind::RegPC), 1);
    vm.step();

    assert_eq!(vm.get_register(RegisterKind::RegPC), 0);
}

#[test]
fn vm_beq() {
    let data = ".text
      sum: add $t0, $t0, 5
      beq $t0, 5, sum ";

    let mut vm = parse_vm(tokenize(data).unwrap()).unwrap();
    while !vm.is_error() {
        vm.step();
    }

    assert_eq!(vm.get_register(RegisterKind::Reg08), 10);
}

#[test]
fn vm_bne() {
    let data = ".text
      sum: add $t0, $t0, 5
      bne $t0, 10, sum ";

    let mut vm = parse_vm(tokenize(data).unwrap()).unwrap();
    while !vm.is_error() {
        vm.step();
    }

    assert_eq!(vm.get_register(RegisterKind::Reg08), 10);
}

#[test]
fn vm_ble() {
    let data = ".text
      sum: add $t0, $t0, 5
      ble $t0, 10, sum ";

    let mut vm = parse_vm(tokenize(data).unwrap()).unwrap();
    while !vm.is_error() {
        vm.step();
    }

    assert_eq!(vm.get_register(RegisterKind::Reg08), 15);
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

    let mut vm = parse_vm(tokenize(data).unwrap()).unwrap();

    assert_eq!(vm.get_memory_byte(0).unwrap(), b'h');
    assert_eq!(vm.get_memory_byte(1).unwrap(), b'e');
    assert_eq!(vm.get_memory_byte(2).unwrap(), b'l');
    assert_eq!(vm.get_memory_byte(3).unwrap(), b'l');
    assert_eq!(vm.get_memory_byte(4).unwrap(), b'o');

    vm.step();
    assert_eq!(vm.get_register(RegisterKind::Reg08), 'h' as u32);
    vm.step();
    assert_eq!(vm.get_register(RegisterKind::Reg09), 'e' as u32);
    vm.step();
    assert_eq!(vm.get_register(RegisterKind::Reg10), 'l' as u32);
    vm.step();
    assert_eq!(vm.get_register(RegisterKind::Reg11), 'l' as u32);
    vm.step();
    assert_eq!(vm.get_register(RegisterKind::Reg12), 'o' as u32);
}

#[test]
fn vm_nop() {
    let data = ".text\nnop\n";

    let mut vm = parse_vm(tokenize(data).unwrap()).unwrap();

    vm.step();
    assert_eq!(vm.get_register(RegisterKind::RegPC), 1);
}

#[test]
fn vm_store() {
    let data = ".text
        add $t0, $t0, 258
        sw $t0, 0";

    let mut vm = parse_vm(tokenize(data).unwrap()).unwrap();

    vm.step();
    vm.step();

    assert_eq!(vm.get_register(RegisterKind::Reg08), 258);
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

        let mut vm = parse_vm(tokenize(data).unwrap()).unwrap();

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

        let mut vm = parse_vm(tokenize(data).unwrap()).unwrap();

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
    let mut vm = parse_vm(tokenize(data).unwrap()).unwrap();

    while !vm.is_error() {
        vm.step();
    }

    assert_eq!(vm.get_register(RegisterKind::RegHi), 4294967295);
}

#[test]
fn vm_test_11() {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("tests/vm/test11.asm");

    let data = &read_to_string(path).unwrap();
    let mut vm = parse_vm(tokenize(data).unwrap()).unwrap();

    while !vm.is_error() {
        vm.step();
    }

    assert_eq!(vm.get_register(RegisterKind::RegPC), 21);
}

#[test]
fn vm_test_17() {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("tests/vm/test17.asm");

    let data = &read_to_string(path).unwrap();
    let mut vm = parse_vm(tokenize(data).unwrap()).unwrap();

    while !vm.is_error() {
        vm.step();
    }

    assert_eq!(vm.get_register(RegisterKind::RegPC), 5);
}
