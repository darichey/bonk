mod lexer;

use lalrpop_util::{lalrpop_mod, ParseError};

use self::lexer::Token;

lalrpop_mod!(
    #[allow(clippy::all)]
    #[allow(unused)]
    pub grammar
);

pub fn parse(src: &str) -> Result<(), ParseError<usize, Token<'_>, ()>> {
    let lexer = lexer::Lexer::new(src)
        .spanned()
        .map(|(token, span)| token.map(|token| (span.start, token, span.end)));
    let parser = grammar::LedgerParser::new();
    parser.parse(src, lexer)
}

#[cfg(test)]
mod tests {
    use crate::parse;

    #[test]
    fn parses() {
        let src = r#"2023-01-01 "Mcdonald's"
  expenses:fast_food         10.91
  liabilities:my_credit_card

2023-01-02 "Paying credit card"
  liabilities:my_credit_card    10.91
  assets:my_checking
"#;

        assert_eq!(parse(src), Ok(()));
    }
}
