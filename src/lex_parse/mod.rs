use self::token::Token;
use self::token::TokenType;
mod token;

#[test]
fn test_mod() -> () {
    let t: Token = Token::new_empty_token(TokenType::EQUAL, 35);
    t.get_value();
    println!("{}", t);

    let t2: Token = Token::new_token(TokenType::ERROR, 35, "erroring".to_owned());

    assert!(t != t2);
}
