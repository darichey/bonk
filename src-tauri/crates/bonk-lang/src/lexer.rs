use chrono::NaiveDate;
use logos::{Lexer as LogosLexer, Logos};

pub type Lexer<'input> = LogosLexer<'input, Token>;

#[derive(Logos, Debug, PartialEq)]
pub enum Token {
    #[regex(r"(\d{4})-(\d{2})-(\d{2})", date)]
    Date(NaiveDate),
    #[regex(r#""([^"\\]|\\["\\bnfrt])*""#, description)]
    Description(String),
    #[regex(r"[A-Za-z_][A-Za-z0-9_]*(:[A-Za-z_][A-Za-z0-9_]*)*", account)]
    Account(String),
    #[regex(r"-?\d+(\.\d+)?", number)]
    Number(f64),
    #[token(" ")]
    Space,
    #[token("\n")]
    NewLine,
}

fn date(lex: &mut LogosLexer<Token>) -> NaiveDate {
    NaiveDate::parse_from_str(lex.slice(), "%Y-%m-%d").expect("date should be valid")
}

fn description(lex: &mut LogosLexer<Token>) -> String {
    let token = lex.slice();
    token[1..token.len() - 1].to_owned()
}

fn account(lex: &mut LogosLexer<Token>) -> String {
    lex.slice().to_owned()
}

fn number(lex: &mut LogosLexer<Token>) -> f64 {
    lex.slice().parse().expect("number should be valid")
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
                Token::Date(NaiveDate::from_ymd_opt(2023, 1, 1).unwrap()),
                Token::Space,
                Token::Description("Mcdonald's".to_string()),
                Token::NewLine,
                Token::Space,
                Token::Space,
                Token::Account("expenses:fast_food".to_string()),
                Token::Space,
                Token::Space,
                Token::Space,
                Token::Space,
                Token::Number(10.91),
                Token::NewLine,
                Token::Space,
                Token::Space,
                Token::Account("liabilities:my_credit_card".to_string()),
            ],
        );
    }

    #[test]
    fn lex_dates() {
        assert_tokens_eq(
            "2023-01-01",
            &[Token::Date(NaiveDate::from_ymd_opt(2023, 1, 1).unwrap())],
        );
    }

    #[test]
    fn lex_descriptions() {
        assert_tokens_eq(r#""hello""#, &[Token::Description("hello".to_string())]);
        assert_tokens_eq(
            r#""hello_world""#,
            &[Token::Description("hello_world".to_string())],
        );
        assert_tokens_eq(
            r#""HeLLo_WoRLd""#,
            &[Token::Description("HeLLo_WoRLd".to_string())],
        );
    }

    #[test]
    fn lex_accounts() {
        assert_tokens_eq("expenses", &[Token::Account("expenses".to_string())]);

        assert_tokens_eq(
            "expenses:fast_food",
            &[Token::Account("expenses:fast_food".to_string())],
        );
    }

    #[test]
    fn lex_numbers() {
        assert_tokens_eq("3", &[Token::Number(3.0)]);
        assert_tokens_eq("3.15", &[Token::Number(3.15)]);
        assert_tokens_eq("10.00", &[Token::Number(10.0)]);
        assert_tokens_eq("-3", &[Token::Number(-3.0)]);
        assert_tokens_eq("-3.0", &[Token::Number(-3.0)]);
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
