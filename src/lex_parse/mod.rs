use self::token::*;
use self::lexer::*;
pub mod token;
pub mod lexer;

#[test]
fn test_mod() -> () {
    let t: Token = Token::new_empty_token(TokenType::EQUAL, 35);
    t.get_value();
    println!("{}", t);

    let t2: Token = Token::new_token(TokenType::ERROR, 35, "erroring");

    assert!(t != t2);
}
