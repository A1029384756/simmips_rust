#[cfg(test)]
use super::token::{Token, TokenType};

#[test]
fn token_print() {
    let t: Token = Token::new_token(TokenType::Equal, 35, "=");
    assert_eq!(
        format!("{}", t),
        "Token: type(Equal) value (=) source line (35)"
    );
}

#[test]
fn token_type_print() {
    let tt: TokenType = TokenType::Error;
    assert_eq!(format!("{}", tt), "Error");
}

#[test]
fn token_equality() {
    let t: Token = Token::new_token(TokenType::Equal, 35, "=");
    let t1: Token = Token::new_token(TokenType::Equal, 35, "=");
    let t2: Token = Token::new_token(TokenType::Sep, 35, "=");

    assert!(t == t1);
    assert!(t != t2);
}
