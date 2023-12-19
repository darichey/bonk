use logos::{Lexer as LogosLexer, Logos};

pub type Lexer<'input> = LogosLexer<'input, Token<'input>>;

#[derive(Logos, Copy, Clone, Debug, PartialEq)]
pub enum Token<'input> {
    #[regex(r"(\d{4})-(\d{2})-(\d{2})")]
    Date(&'input str),
    #[regex(r#""([^"\\]|\\["\\bnfrt])*""#)]
    Description(&'input str),
    #[regex(r"[A-Za-z_][A-Za-z0-9_]*(:[A-Za-z_][A-Za-z0-9_]*)*")]
    Account(&'input str),
    #[regex(r"-?\d+(\.\d+)?")]
    Amount(&'input str),
    #[token(" ")]
    Space,
    #[token("\n")]
    NewLine,
}

mod tests {
    use super::*;

    fn assert_tokens_eq(src: &str, expected: &[Token]) {
        let tokens: Vec<_> = Token::lexer(src).collect();
        eprintln!("{:#?}", tokens);
        let actual = tokens.into_iter().collect::<Result<Vec<_>, _>>().unwrap();
        assert_eq!(actual, expected);
    }

    #[test]
    fn lex() {
        assert_tokens_eq(
            r#"2023-01-01 "Mcdonald's"
  expenses:fast_food    10.91
  liabilities:my_credit_card"#,
            &[
                Token::Date("2023-01-01"),
                Token::Space,
                Token::Description(r#""Mcdonald's""#),
                Token::NewLine,
                Token::Space,
                Token::Space,
                Token::Account("expenses:fast_food"),
                Token::Space,
                Token::Space,
                Token::Space,
                Token::Space,
                Token::Amount("10.91"),
                Token::NewLine,
                Token::Space,
                Token::Space,
                Token::Account("liabilities:my_credit_card"),
            ],
        );
    }

    #[test]
    fn lex_dates() {
        assert_tokens_eq("2023-01-01", &[Token::Date("2023-01-01")]);
    }

    #[test]
    fn lex_descriptions() {
        assert_tokens_eq(r#""hello""#, &[Token::Description(r#""hello""#)]);
        assert_tokens_eq(
            r#""hello_world""#,
            &[Token::Description(r#""hello_world""#)],
        );
        assert_tokens_eq(
            r#""HeLLo_WoRLd""#,
            &[Token::Description(r#""HeLLo_WoRLd""#)],
        );
    }

    #[test]
    fn lex_accounts() {
        assert_tokens_eq("expenses", &[Token::Account("expenses")]);

        assert_tokens_eq(
            "expenses:fast_food",
            &[Token::Account("expenses:fast_food")],
        );
    }

    #[test]
    fn lex_amounts() {
        assert_tokens_eq("3", &[Token::Amount("3")]);
        assert_tokens_eq("3.15", &[Token::Amount("3.15")]);
        assert_tokens_eq("10.00", &[Token::Amount("10.00")]);
        assert_tokens_eq("-3", &[Token::Amount("-3")]);
        assert_tokens_eq("-3.0", &[Token::Amount("-3.0")]);
    }

    #[test]
    fn lex_ws() {
        assert_tokens_eq("\n", &[Token::NewLine]);
        assert_tokens_eq("\n\n", &[Token::NewLine, Token::NewLine]);
        assert_tokens_eq(" ", &[Token::Space]);
        assert_tokens_eq(
            "    ",
            &[Token::Space, Token::Space, Token::Space, Token::Space],
        );
    }
}
