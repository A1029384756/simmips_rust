#[cfg(test)]
use crate::lex_parse::{token::{TokenList, TokenType}, lexer::tokenize};

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

    let result: TokenList = tokenize(data);

    assert_eq!(result.last().unwrap().get_type(), &TokenType::EOL);
    assert_eq!(result.first().unwrap().get_type(), &TokenType::STRING);
    assert_eq!(result.first().unwrap().get_line(), &2);
    assert_eq!(result.len(), 31);
}

#[test]
fn tokenize_edge_cases() {
    {
        let data: &str = "string = 100\n\"\"data";
        let result: TokenList = tokenize(data);
        assert_eq!(result.last().unwrap().get_type(), &TokenType::EOL);
        assert_eq!(result.len(), 9);
        assert_eq!(result.get(5).unwrap().get_type(), &TokenType::STRING);
        assert_eq!(result.get(5).unwrap().get_value(), "");
    }
    {
        let data: &str = "test\n\ndummydata\n";
        let result: TokenList = tokenize(data);
        assert_eq!(result.len(), 4);
    }
    {
        let data: &str = "(()data";
        let result: TokenList = tokenize(data);
        assert_eq!(result.len(), 4);
        assert_eq!(result.last().unwrap().get_type(), &TokenType::ERROR);
    }
    {
        let data: &str = "(data))";
        let result: TokenList = tokenize(data);
        assert_eq!(result.len(), 4);
        assert_eq!(result.last().unwrap().get_type(), &TokenType::ERROR);
    }
    {
        let data: &str = "\n\n#comment \"here\"\n\n";
        let result: TokenList = tokenize(data);
        assert_eq!(result.len(), 0);
    }
    {
        let data: &str = "\"( #\"";
        let result: TokenList = tokenize(data);
        assert_eq!(result.len(), 4);
        assert_eq!(result.last().unwrap().get_type(), &TokenType::EOL);
    }
    {
        let data: &str = "\"";
        let result: TokenList = tokenize(data);
        assert_eq!(result.len(), 2);
        assert_eq!(result.last().unwrap().get_type(), &TokenType::ERROR);
    }
    {
        let data: &str = "";
        let result: TokenList = tokenize(data);
        assert_eq!(result.len(), 0);
    }
}

#[test]
fn file_tokenize_tests() {
    {
        let mut path: std::path::PathBuf = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/pass/unix/test00.asm");

        let data: &str = &std::fs::read_to_string(path).unwrap();
        let result: TokenList = tokenize(data);
        assert_ne!(result.last().unwrap().get_type(), &TokenType::ERROR);
        assert!(result.len() > 0);
    }
    {
        let mut path: std::path::PathBuf = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/fail/unix/test02.asm");

        let data: &str = &std::fs::read_to_string(path).unwrap();
        let result: TokenList = tokenize(data);
        assert_eq!(result.last().unwrap().get_type(), &TokenType::ERROR);
        assert!(result.len() > 0);
    }
    {
        let mut path: std::path::PathBuf = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/fail/unix/test07.asm");

        let data: &str = &std::fs::read_to_string(path).unwrap();
        let result: TokenList = tokenize(data);
        assert_ne!(result.last().unwrap().get_type(), &TokenType::ERROR);
        assert!(result.len() > 0);
        assert_eq!(result.first().unwrap().get_type(), &TokenType::STRING);
        assert_eq!(result.first().unwrap().get_line(), &6);
        assert_eq!(result.last().unwrap().get_type(), &TokenType::EOL);
        assert_eq!(result.last().unwrap().get_line(), &26);
        assert_eq!(result.len(), 69);
    }
}
