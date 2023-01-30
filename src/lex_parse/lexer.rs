use crate::lex_parse::token::*;

fn push_str(string: &mut String, ln: &u32, tk_list: &mut TokenList) -> () {
    if !string.is_empty() {
        tk_list.push(Token::new_token(TokenType::STRING, *ln, &string));
        string.clear()
    }
}

pub fn tokenize(in_str: &str) -> TokenList {
    let mut line_number: u32 = 0;
    let mut tokens: TokenList = TokenList::default();

    let mut tmp: String = String::default();
    let mut in_paren: bool = false;

    let mut iter = in_str.chars().into_iter();
    while let Some(char) = iter.next() {
        match char {
            '=' => {
                push_str(&mut tmp, &line_number, &mut tokens);
                tokens.push(Token::new_empty_token(TokenType::EQUAL, line_number));
            }
            ' ' => {
                push_str(&mut tmp, &line_number, &mut tokens);
            }
            '#' => {
                push_str(&mut tmp, &line_number, &mut tokens);
                if let Some(token) = tokens.last() {
                    if token.get_type() != &TokenType::EOL {
                        tokens.push(Token::new_empty_token(TokenType::EOL, line_number));
                    }
                }

                loop {
                    let next_char = iter.next();
                    match next_char {
                        Some('\n') => {
                            break;
                        }
                        Some(..) => {}
                        None => {
                            break;
                        }
                    }
                }
            }
            '"' => {
                push_str(&mut tmp, &line_number, &mut tokens);
                tokens.push(Token::new_empty_token(TokenType::STRINGDELIM, line_number));

                loop {
                    let next_char = iter.next();
                    match next_char {
                        Some('"') => {
                            push_str(&mut tmp, &line_number, &mut tokens);
                            tokens
                                .push(Token::new_empty_token(TokenType::STRINGDELIM, line_number));
                            break;
                        }
                        Some(valid_char) => {
                            tmp.push(valid_char);
                        }
                        None => {
                            tokens.push(Token::new_token(
                                TokenType::ERROR,
                                line_number,
                                &format!(
                                    "Error: unmatched string delimeter on line {}",
                                    line_number
                                ),
                            ));
                            tmp.clear();
                            break;
                        }
                    }
                }
            }
            '(' => {
                push_str(&mut tmp, &line_number, &mut tokens);
                tokens.push(Token::new_empty_token(TokenType::OPENPAREN, line_number));
                in_paren = true;
            }
            ')' => {
                if !in_paren {
                    tokens.push(Token::new_token(
                        TokenType::ERROR,
                        line_number,
                        &format!("Error: unmatched \" on line {}", line_number),
                    ));
                    tmp.clear();
                    break;
                }

                push_str(&mut tmp, &line_number, &mut tokens);
                tokens.push(Token::new_empty_token(TokenType::CLOSEPAREN, line_number));
                in_paren = false;
            }
            ',' => {
                push_str(&mut tmp, &line_number, &mut tokens);
                tokens.push(Token::new_empty_token(TokenType::SEP, line_number));
            }
            '\n' => {
                if in_paren {
                    tokens.push(Token::new_token(
                        TokenType::ERROR,
                        line_number,
                        &format!("Error, unmatched paren on line {}", line_number),
                    ));
                    break;
                }
                push_str(&mut tmp, &line_number, &mut tokens);
                tokens.push(Token::new_empty_token(TokenType::EOL, line_number));
                line_number += 1;
            }
            _ => tmp.push(char),
        }
    }

    push_str(&mut tmp, &line_number, &mut tokens);

    tokens
}

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

    let cmp: &str = "(STRING,\".data\") (EOL,\"\") (STRING,\"LENGTH\") (EQUAL,\"\") (STRING,\"10\") (EOL,\"\") (STRING,\"array:\") (STRING,\".space\") (STRING,\"LENGTH\") (EOL,\"\") (STRING,\"str:\") (STRING,\".asciiz\") (STRINGDELIM,\"\") (STRING,\"the (end)\") (STRINGDELIM,\"\") (EOL,\"\") (STRING,\".text\") (EOL,\"\") (STRING,\"main:\") (STRING,\"lw\") (STRING,\"$t0\") (SEP,\"\") (STRING,\"array\") (EOL,\"\") (STRING,\"lw\") (STRING,\"$t1\") (SEP,\"\") (OPENPAREN,\"\") (STRING,\"$t0\") (CLOSEPAREN,\"\") (EOL,\"\")";

    assert_eq!(format!("{}", cmp), format!("{}", tokenize(data)));
}
