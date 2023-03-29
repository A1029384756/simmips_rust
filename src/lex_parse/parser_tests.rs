#[cfg(test)]
use super::{lexer::tokenize, parser::parse};

#[test]
fn section_annotation() {
    {
        let input: &str = ".data\n";
        assert!(matches!(parse(tokenize(input).unwrap()), Ok(..)));
    }
    {
        let input: &str = ".text\n";
        assert!(matches!(parse(tokenize(input).unwrap()), Ok(..)));
    }
    {
        let input: &str = "";
        assert!(matches!(parse(tokenize(input).unwrap()), Err(..)));
    }
    {
        let input: &str = ".data\ntest=10\n.text\nlb $t2, ($t0)\n";
        assert!(matches!(parse(tokenize(input).unwrap()), Ok(..)));
    }
}

#[test]
fn error_message() {
    let input: &str = ".data\ntest\n";
    assert!(parse(tokenize(input).unwrap())
        .unwrap_err()
        .starts_with("Error:2:"));
}

#[test]
fn data_parse() {
    {
        let input: &str = ".data\ntest = 10\n";
        assert!(matches!(parse(tokenize(input).unwrap()), Ok(..)));
    }
    {
        let input: &str = ".data\n7est = 10\n";
        assert!(matches!(parse(tokenize(input).unwrap()), Err(..)));
    }
    {
        let input: &str = ".data\nconst\n";
        assert!(matches!(parse(tokenize(input).unwrap()), Err(..)));
    }
    {
        let input: &str = ".data\nconst = \n";
        assert!(matches!(parse(tokenize(input).unwrap()), Err(..)));
    }
    {
        let input: &str = &String::new();
        assert!(matches!(parse(tokenize(input).unwrap()), Err(..)));
    }
    {
        let input: &str = ".data\nlabel: .space 100\n";
        assert!(matches!(parse(tokenize(input).unwrap()), Ok(..)));
    }
    {
        let input: &str = ".data
            label: .space 100
            .word 3, 289, 5, 19
        ";
        assert!(matches!(parse(tokenize(input).unwrap()), Ok(..)));
    }
    {
        let input: &str = ".data
            label: .space 100
            .word 42, 53 653
        ";
        assert!(matches!(parse(tokenize(input).unwrap()), Err(..)));
    }
}

#[test]
fn integer_layouts() {
    {
        let input: &str = ".data\n.word 4294967295\n";
        assert!(matches!(parse(tokenize(input).unwrap()), Ok(..)));
    }
    {
        let input: &str = ".data\n.word 4294967296\n";
        assert!(matches!(parse(tokenize(input).unwrap()), Err(..)));
    }
    {
        let input: &str = ".data\n.word 3, 289, 5, 19\n";
        assert!(matches!(parse(tokenize(input).unwrap()), Ok(..)));
    }
    {
        let input: &str = ".data\n.word 3, 289, 5, 19\n.word 62\n";
        assert!(matches!(parse(tokenize(input).unwrap()), Ok(..)));
    }
    {
        let input: &str = ".data\n.space 3, 289, 5, 19\n";
        assert!(matches!(parse(tokenize(input).unwrap()), Err(..)));
    }
}

#[test]
fn half_layouts() {
    {
        let input = ".data\n.half 3\n";
        assert!(matches!(parse(tokenize(input).unwrap()), Ok(..)));
    }
    {
        let input = ".data\n.half 65536\n";
        assert!(matches!(parse(tokenize(input).unwrap()), Err(..)));
    }
    {
        let input = ".data\n.half -1\n";
        assert!(matches!(parse(tokenize(input).unwrap()), Ok(..)));
    }
    {
        let input = ".data\n.half -32789\n";
        assert!(matches!(parse(tokenize(input).unwrap()), Err(..)));
    }
    {
        let input = ".data\n.half +32768\n";
        assert!(matches!(parse(tokenize(input).unwrap()), Err(..)));
    }
    {
        let input = ".data\n.half +11\n";
        assert!(matches!(parse(tokenize(input).unwrap()), Ok(..)));
    }
}

#[test]
fn byte_layouts() {
    {
        let input = ".data\n.byte 3\n";
        assert!(matches!(parse(tokenize(input).unwrap()), Ok(..)));
    }
    {
        let input = ".data\n.byte 256\n";
        assert!(matches!(parse(tokenize(input).unwrap()), Err(..)));
    }
    {
        let input = ".data\n.byte -1\n";
        assert!(matches!(parse(tokenize(input).unwrap()), Ok(..)));
    }
    {
        let input = ".data\n.byte -129\n";
        assert!(matches!(parse(tokenize(input).unwrap()), Err(..)));
    }
    {
        let input = ".data\n.byte +128\n";
        assert!(matches!(parse(tokenize(input).unwrap()), Err(..)));
    }
    {
        let input = ".data\n.byte +11\n";
        assert!(matches!(parse(tokenize(input).unwrap()), Ok(..)));
    }
}

#[test]
fn text_parse() {
    {
        let input = ".text\nlw $t2, $t0\n";
        assert!(matches!(parse(tokenize(input).unwrap()), Ok(..)));
    }
    {
        let input = ".text\nlh $s9, $s3\n";
        assert!(matches!(parse(tokenize(input).unwrap()), Err(..)));
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
        match tokenize(data) {
            Ok(tokens) => {
                assert!(matches!(parse(tokens), Err(..)))
            },
            Err(_) => (),
        }
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
        assert!(matches!(parse(tokenize(data).unwrap()), Ok(..)));
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

    assert!(matches!(parse(tokenize(data).unwrap()), Ok(..)));
}
