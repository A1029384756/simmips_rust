#[cfg(test)]
use super::token::{Token, TokenType};

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
