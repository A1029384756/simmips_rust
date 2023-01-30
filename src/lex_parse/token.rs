use core::fmt;
use std::vec::Vec;

#[allow(dead_code)]
#[derive(Debug, Default, PartialEq)]
pub enum TokenType {
    #[default]
    STRING,
    EOL,
    SEP,
    OPENPAREN,
    CLOSEPAREN,
    STRINGDELIM,
    EQUAL,
    ERROR,
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Default)]
pub struct Token {
    token_type: TokenType,
    line_number: u32,
    token: String,
}

#[allow(dead_code)]
impl Token {
    pub fn new_empty_token(tt: TokenType, line: u32) -> Self {
        Token {
            token_type: tt,
            line_number: line,
            token: "".to_owned(),
        }
    }

    pub fn new_token(tt: TokenType, line: u32, value: &str) -> Self {
        Token {
            token_type: tt,
            line_number: line,
            token: value.to_string(),
        }
    }

    pub fn get_type(&self) -> &TokenType {
        &self.token_type
    }

    pub fn get_line(&self) -> &u32 {
        &self.line_number
    }

    pub fn get_value(&self) -> &str {
        &self.token
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Token: type({}) value ({}) source line ({})",
            self.token_type, self.token, self.line_number
        )
    }
}

impl PartialEq for Token {
    fn eq(&self, rhs: &Self) -> bool {
        (self.token_type == rhs.token_type)
            && (self.line_number == rhs.line_number)
            && (self.token == rhs.token)
    }
}

#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct TokenList(Vec<Token>);

impl TokenList {
    pub fn push(&mut self, token: Token) -> () {
        self.0.push(token);
    }
}

impl fmt::Display for TokenList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut str: String = String::default();
        for elem in &self.0 {
            str.push_str(&format!("({},{:?}) ", elem.token_type, elem.token));
        }
        str.pop();

        write!(f, "{}", str)
    }
}

#[test]
fn token_print() {
    let t: Token = Token::new_token(TokenType::EQUAL, 35, "=");
    assert_eq!(
        format!("{}", t),
        "Token: type(EQUAL) value (=) source line (35)"
    );
}

#[test]
fn token_type_print() {
    let tt: TokenType = TokenType::ERROR;
    assert_eq!(format!("{}", tt), "ERROR");
}

#[test]
fn token_equality() {
    let t: Token = Token::new_token(TokenType::EQUAL, 35, "=");
    let t1: Token = Token::new_token(TokenType::EQUAL, 35, "=");
    let t2: Token = Token::new_token(TokenType::SEP, 35, "=");

    assert!(t == t1);
    assert!(t != t2);
}
