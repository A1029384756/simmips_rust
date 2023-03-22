mod lex_parse;
use lex_parse::{
    parser::parse,
    lexer::tokenize
};

fn main() {
    let _val = parse(tokenize(".data"));
}
