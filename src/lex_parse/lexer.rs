use crate::lex_parse::token::*;
use std::str::Chars;

struct Tokenizer {
    line_num: u32,
    token_val: String,
    paren_depth: i32,
    tokens: TokenList,
}

impl Default for Tokenizer {
    fn default() -> Self {
        Tokenizer {
            line_num: 1,
            token_val: String::default(),
            paren_depth: 0,
            tokens: TokenList::default(),
        }
    }
}

pub fn tokenize(in_str: &str) -> Result<TokenList, String> {
    let mut tokenizer: Tokenizer = Tokenizer::default();
    let mut line_iter = in_str.lines();

    while let Some(line) = line_iter.next() {
        let mut char_iter = line.chars();

        while let Some(char) = char_iter.next() {
            match char {
                '#' => break,
                '"' => handle_string_delim(&mut tokenizer, &mut char_iter),
                '(' => handle_open_paren(&mut tokenizer),
                ')' => handle_close_paren(&mut tokenizer),
                '=' => handle_eq(&mut tokenizer),
                ',' => handle_sep(&mut tokenizer),
                char if char.is_whitespace() => {
                    push_str(&mut tokenizer);
                }
                _ => tokenizer.token_val.push(char),
            }
            if let Some(last_token) = tokenizer.tokens.last() {
                if last_token.get_type() == &TokenType::ERROR {
                    return Err(last_token.get_value().to_string());
                }
            }
        }

        if tokenizer.paren_depth > 0 {
            return Err("Error: mismatched paren".to_string());
        }

        push_str(&mut tokenizer);
        if let Some(last_token) = tokenizer.tokens.last() {
            if last_token.get_type() != &TokenType::EOL {
                tokenizer
                    .tokens
                    .push(Token::new_empty_token(TokenType::EOL, tokenizer.line_num));
            }
        }

        tokenizer.line_num += 1;
    }

    Ok(tokenizer.tokens)
}

fn push_str(state: &mut Tokenizer) -> () {
    if !state.token_val.is_empty() {
        state.tokens.push(Token::new_token(
            TokenType::STRING,
            state.line_num,
            &state.token_val,
        ));
        state.token_val.clear();
    }
}

fn handle_string_delim(state: &mut Tokenizer, char_iter: &mut Chars) -> () {
    push_str(state);
    state.tokens.push(Token::new_empty_token(
        TokenType::STRINGDELIM,
        state.line_num,
    ));
    loop {
        let string_char = char_iter.next();
        match string_char {
            Some('"') => {
                state.tokens.push(Token::new_token(
                    TokenType::STRING,
                    state.line_num,
                    &state.token_val,
                ));
                state.token_val.clear();
                state.tokens.push(Token::new_empty_token(
                    TokenType::STRINGDELIM,
                    state.line_num,
                ));
                break;
            }
            Some(string_char) => {
                state.token_val.push(string_char);
            }
            None => {
                state.tokens.push(Token::new_token(
                    TokenType::ERROR,
                    state.line_num,
                    "Error: misplaced string delim",
                ));
                break;
            }
        }
    }
}

fn handle_open_paren(state: &mut Tokenizer) -> () {
    push_str(state);
    state
        .tokens
        .push(Token::new_empty_token(TokenType::OPENPAREN, state.line_num));
    state.paren_depth += 1;
}

fn handle_close_paren(state: &mut Tokenizer) -> () {
    state.paren_depth -= 1;

    if state.paren_depth < 0 {
        state.tokens.push(Token::new_token(
            TokenType::ERROR,
            state.line_num,
            "Error: mismatched paren",
        ));
        return;
    }

    push_str(state);
    state.tokens.push(Token::new_empty_token(
        TokenType::CLOSEPAREN,
        state.line_num,
    ));
}

fn handle_eq(state: &mut Tokenizer) -> () {
    push_str(state);
    state
        .tokens
        .push(Token::new_empty_token(TokenType::EQUAL, state.line_num));
}

fn handle_sep(state: &mut Tokenizer) -> () {
    push_str(state);
    state
        .tokens
        .push(Token::new_empty_token(TokenType::SEP, state.line_num));
}
