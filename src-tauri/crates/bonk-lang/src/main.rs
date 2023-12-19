use std::fs;

use lalrpop_util::lalrpop_mod;

use crate::lexer::Lexer;

mod lexer;
mod types;

lalrpop_mod!(
    #[allow(clippy::all)]
    #[allow(unused)]
    pub grammar
);

fn main() {
    let src = &fs::read_to_string("test.bonk").unwrap();
    let lexer = Lexer::new(src)
        .spanned()
        .map(|(token, span)| token.map(|token| (span.start, token, span.end)));
    let parser = grammar::LedgerParser::new();
    parser.parse(src, lexer).unwrap();

    println!("Hello, world!");
}
