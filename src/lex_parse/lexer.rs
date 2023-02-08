use std::str::Chars;

use crate::lex_parse::token::*;

struct TokenizeState {
    line_num: u32,
    token_val: String,
    paren_depth: i32,
    tokens: TokenList,
}

impl Default for TokenizeState {
    fn default() -> Self {
        TokenizeState {
            line_num: 1,
            token_val: String::default(),
            paren_depth: 0,
            tokens: TokenList::default(),
        }
    }
}

pub fn tokenize(in_str: &str) -> TokenList {
    let mut state: TokenizeState = TokenizeState::default();
    let mut line_iter = in_str.lines();

    while let Some(line) = line_iter.next() {
        let mut char_iter = line.chars();

        while let Some(char) = char_iter.next() {
            match char {
                '#' => break,
                '"' => handle_string_delim(&mut state, &mut char_iter),
                '(' => handle_open_paren(&mut state),
                ')' => handle_close_paren(&mut state),
                '=' => handle_eq(&mut state),
                ',' => handle_sep(&mut state),
                char if char.is_whitespace() => {
                    push_str(&mut state);
                }
                _ => state.token_val.push(char),
            }
            if let Some(last_token) = state.tokens.last() {
                if last_token.get_type() == &TokenType::ERROR {
                    return state.tokens;
                }
            }
        }

        if state.paren_depth > 0 {
            state.tokens.push(Token::new_token(
                TokenType::ERROR,
                state.line_num,
                "Error: mismatched paren",
            ));
            return state.tokens;
        }

        push_str(&mut state);
        if let Some(last_token) = state.tokens.last() {
            if last_token.get_type() != &TokenType::EOL {
                state
                    .tokens
                    .push(Token::new_empty_token(TokenType::EOL, state.line_num));
            }
        }

        state.line_num += 1;
    }

    state.tokens
}

fn push_str(state: &mut TokenizeState) -> () {
    if !state.token_val.is_empty() {
        state.tokens.push(Token::new_token(
            TokenType::STRING,
            state.line_num,
            &state.token_val,
        ));
        state.token_val.clear();
    }
}

fn handle_string_delim(state: &mut TokenizeState, char_iter: &mut Chars) -> () {
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

fn handle_open_paren(state: &mut TokenizeState) -> () {
    push_str(state);
    state
        .tokens
        .push(Token::new_empty_token(TokenType::OPENPAREN, state.line_num));
    state.paren_depth += 1;
}

fn handle_close_paren(state: &mut TokenizeState) -> () {
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

fn handle_eq(state: &mut TokenizeState) -> () {
    push_str(state);
    state
        .tokens
        .push(Token::new_empty_token(TokenType::EQUAL, state.line_num));
}

fn handle_sep(state: &mut TokenizeState) -> () {
    push_str(state);
    state
        .tokens
        .push(Token::new_empty_token(TokenType::SEP, state.line_num));
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
