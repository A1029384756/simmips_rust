use core::fmt;

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub enum TokenType {
    EOL,
    SEP,
    OPENPAREN,
    CLOSEPAREN,
    STRINGDELIM,
    EQUAL,
    STRING,
    ERROR,
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug)]
pub struct Token {
    token_type: TokenType,
    line_number: u32,
    token: String,
}

#[allow(dead_code)]
impl Token {
    fn new_empty_token(tt: TokenType, line: u32) -> Self {
        Token {
            token_type: tt,
            line_number: line,
            token: "".to_string(),
        }
    }

    fn new_token(tt: TokenType, line: u32, value: String) -> Self {
        Token {
            token_type: tt,
            line_number: line,
            token: value,
        }
    }

    fn get_type(&self) -> &TokenType {
        &self.token_type
    }

    fn get_line(&self) -> &u32 {
        &self.line_number
    }

    fn get_value(&self) -> &str {
        &self.token
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            format!(
                "Token: type({}) value ({}) source line ({})",
                self.token_type,
                self.token,
                self.line_number
            )
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
type TokenList = Vec<Token>;

#[test]
fn token_print() {
    let t: Token = Token::new_token(TokenType::EQUAL, 35, "=".to_string());
    assert_eq!(format!("{}", t), "Token: type(EQUAL) value (=) source line (35)");
}

#[test]
fn token_type_print() {
    let tt: TokenType = TokenType::ERROR;
    assert_eq!(format!("{}", tt), "ERROR");
}
