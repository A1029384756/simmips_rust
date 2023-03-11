#[cfg(test)]
use super::{lexer::tokenize, parser::parse};

#[test]
fn section_annotation() {
    {
        let input: &str = ".data\n";
        assert!(!bool::from(parse(tokenize(input))));
    }
    {
        let input: &str = ".text\n";
        assert!(!bool::from(parse(tokenize(input))));
    }
    {
        let input: &str = "";
        assert!(bool::from(parse(tokenize(input))));
    }
    {
        let input: &str = ".data\ntest=10\n.text\nlb $t2, ($t0)\n";
        assert!(!bool::from(parse(tokenize(input))));
    }
}

#[test]
fn error_message() {
    let input: &str = ".data\ntest\n";
    assert!(parse(tokenize(input)).message().starts_with("Error:2:"));
}

#[test]
fn data_parse() {
    {
        let input: &str = ".data\ntest = 10\n";
        assert!(!bool::from(parse(tokenize(input))));
    }
    {
        let input: &str = ".data\n7est = 10\n";
        assert!(bool::from(parse(tokenize(input))));
    }
    {
        let input: &str = ".data\nconst\n";
        assert!(bool::from(parse(tokenize(input))));
    }
    {
        let input: &str = ".data\nconst = \n";
        assert!(bool::from(parse(tokenize(input))));
    }
    {
        let input: &str = &String::new();
        assert!(bool::from(parse(tokenize(input))));
    }
    {
        let input: &str = ".data\nlabel: .space 100\n";
        assert!(!bool::from(parse(tokenize(input))));
    }
    {
        let input: &str = ".data
            label: .space 100
            .word 3, 289, 5, 19
        ";
        assert!(!bool::from(parse(tokenize(input))));
    }
    {
        let input: &str = ".data
            label: .space 100
            .word 42, 53 653
        ";
        assert!(bool::from(parse(tokenize(input))));
    }
}

#[test]
fn integer_layouts() {
    {
        let input: &str = ".data\n.word 4294967295\n";
        assert!(!bool::from(parse(tokenize(input))));
    }
    {
        let input: &str = ".data\n.word 4294967296\n";
        assert!(bool::from(parse(tokenize(input))));
    }
    {
        let input: &str = ".data\n.word 3, 289, 5, 19\n";
        assert!(!bool::from(parse(tokenize(input))));
    }
    {
        let input: &str = ".data\n.word 3, 289, 5, 19\n.word 62\n";
        assert!(!bool::from(parse(tokenize(input))));
    }
    {
        let input: &str = ".data\n.space 3, 289, 5, 19\n";
        assert!(bool::from(parse(tokenize(input))));
    }
}

#[test]
fn half_layouts() {
    {
        let input = ".data\n.half 3\n";
        assert!(!bool::from(parse(tokenize(input))));
    }
    {
        let input = ".data\n.half 65536\n";
        assert!(bool::from(parse(tokenize(input))));
    }
    {
        let input = ".data\n.half -1\n";
        assert!(!bool::from(parse(tokenize(input))));
    }
    {
        let input = ".data\n.half -32789\n";
        assert!(bool::from(parse(tokenize(input))));
    }
    {
        let input = ".data\n.half +32768\n";
        assert!(bool::from(parse(tokenize(input))));
    }
    {
        let input = ".data\n.half +11\n";
        assert!(!bool::from(parse(tokenize(input))));
    }
}

#[test]
fn byte_layouts() {
    {
        let input = ".data\n.byte 3\n";
        assert!(!bool::from(parse(tokenize(input))));
    }
    {
        let input = ".data\n.byte 256\n";
        assert!(bool::from(parse(tokenize(input))));
    }
    {
        let input = ".data\n.byte -1\n";
        assert!(!bool::from(parse(tokenize(input))));
    }
    {
        let input = ".data\n.byte -129\n";
        assert!(bool::from(parse(tokenize(input))));
    }
    {
        let input = ".data\n.byte +128\n";
        assert!(bool::from(parse(tokenize(input))));
    }
    {
        let input = ".data\n.byte +11\n";
        assert!(!bool::from(parse(tokenize(input))));
    }
}

#[test]
fn text_parse() {
    {
        let input = ".text\nlw $t2, $t0\n";
        assert!(!bool::from(parse(tokenize(input))));
    }
    {
        let input = ".text\nlh $s9, $s3\n";
        assert!(bool::from(parse(tokenize(input))));
    }
}

#[test]
fn failing_parse_files() {
    let mut path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("tests/fail/unix/");

    (0..=9).for_each(|val| {
        let mut test_path = path.clone();
        test_path.push(format!("test0{val}.asm"));
        let data = &std::fs::read_to_string(test_path).unwrap();
        assert!(bool::from(parse(tokenize(data))));
    });
}

#[test]
fn passing_parse_files() {
    let mut path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("tests/pass/unix/");

    (0..=8).for_each(|val| {
        let mut test_path = path.clone();
        test_path.push(format!("test0{val}.asm"));
        let data = &std::fs::read_to_string(test_path).unwrap();
        assert!(!bool::from(parse(tokenize(data))));
    });
}

#[test]
fn interleaved_data_text() {
    let data = ".data
      .word 10
      label1: .word 20
      .text
      li $t0, 16
      lw $t1, label1
      .data
      .space 12
      .word 5
      label2: .word 7
      .text
      li $t2, 9
      lb $t3, label2
      .data
      .text
      .text";

    let error = parse(tokenize(data));

    assert!(!bool::from(error));
}
