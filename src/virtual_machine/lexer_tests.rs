#[cfg(test)]
use super::{
    lexer::tokenize,
    token::{TokenList, TokenType},
};

#[test]
fn tokenize_test() {
    let data: &str = "#Dummy comment
        .data # another comment
        LENGTH = 10
        array:  .space LENGTH
        str:    .asciiz \"the (end)\"
        .text
        main:  lw $t0, array
        lw $t1, ($t0)
        ";

    let result: TokenList = tokenize(data).unwrap();

    assert_eq!(result.last().unwrap().get_type(), &TokenType::Eol);
    assert_eq!(result.first().unwrap().get_type(), &TokenType::String);
    assert_eq!(result.first().unwrap().get_line(), &2);
    assert_eq!(result.len(), 31);
}

#[test]
fn tokenize_edge_cases() {
    {
        let data: &str = "string = 100\n\"\"data";
        let result: TokenList = tokenize(data).unwrap();
        assert_eq!(result.last().unwrap().get_type(), &TokenType::Eol);
        assert_eq!(result.len(), 9);
        assert_eq!(result.get(5).unwrap().get_type(), &TokenType::String);
        assert_eq!(result.get(5).unwrap().get_value(), "");
    }
    {
        let data: &str = "test\n\ndummydata\n";
        let result: TokenList = tokenize(data).unwrap();
        assert_eq!(result.len(), 4);
    }
    {
        let data: &str = "(()data";
        let result = tokenize(data);
        assert!(matches!(result, Err(..)));
    }
    {
        let data: &str = "(data))";
        let result = tokenize(data);
        assert!(matches!(result, Err(..)));
    }
    {
        let data: &str = "\n\n#comment \"here\"\n\n";
        let result: TokenList = tokenize(data).unwrap();
        assert_eq!(result.len(), 0);
    }
    {
        let data: &str = "\"( #\"";
        let result: TokenList = tokenize(data).unwrap();
        assert_eq!(result.len(), 4);
        assert_eq!(result.last().unwrap().get_type(), &TokenType::Eol);
    }
    {
        let data: &str = "\"";
        let result = tokenize(data);
        assert!(matches!(result, Err(..)));
    }
    {
        let data: &str = "";
        let result: TokenList = tokenize(data).unwrap();
        assert_eq!(result.len(), 0);
    }
}

#[test]
fn file_tokenize_tests() {
    {
        let mut path: std::path::PathBuf = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/pass/unix/test00.asm");

        let data: &str = &std::fs::read_to_string(path).unwrap();
        let result: TokenList = tokenize(data).unwrap();
        assert!(!result.is_empty());
    }
    {
        let mut path: std::path::PathBuf = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/fail/unix/test02.asm");

        let data: &str = &std::fs::read_to_string(path).unwrap();
        let result = tokenize(data);
        assert!(matches!(result, Err(..)));
    }
    {
        let mut path: std::path::PathBuf = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/fail/unix/test07.asm");

        let data: &str = &std::fs::read_to_string(path).unwrap();
        let result: TokenList = tokenize(data).unwrap();
        assert_ne!(result.last().unwrap().get_type(), &TokenType::Error);
        assert!(!result.is_empty());
        assert_eq!(result.first().unwrap().get_type(), &TokenType::String);
        assert_eq!(result.first().unwrap().get_line(), &6);
        assert_eq!(result.last().unwrap().get_type(), &TokenType::Eol);
        assert_eq!(result.last().unwrap().get_line(), &26);
        assert_eq!(result.len(), 69);
    }
}
