use bonk_ast::SourceSpan;
use bonk_ast_errorless::Date;
use itertools::Itertools;

#[derive(Debug)]
pub struct SyntaxErrors(Vec<SourceSpan>);

pub fn check_syntax(ledger: bonk_ast::Ledger) -> Result<bonk_ast_errorless::Ledger, SyntaxErrors> {
    let errors = ledger.errors();
    if errors.is_empty() {
        Ok(convert_ledger(ledger))
    } else {
        Err(SyntaxErrors(errors))
    }
}

fn convert_ledger(ledger: bonk_ast::Ledger) -> bonk_ast_errorless::Ledger {
    bonk_ast_errorless::Ledger {
        transactions: ledger
            .transactions()
            .into_iter()
            .map(convert_transaction)
            .collect_vec(),
    }
}

fn convert_transaction(transaction: bonk_ast::Transaction) -> bonk_ast_errorless::Transaction {
    bonk_ast_errorless::Transaction {
        date: transaction
            .date()
            .and_then(Date::parse)
            .expect("Couldn't parse date (but tree had no errors?)"),
        description: transaction
            .description()
            .expect("Couldn't get description (but tree had no errors?)")
            .to_string(),
        postings: transaction
            .postings()
            .into_iter()
            .map(convert_posting)
            .collect_vec(),
    }
}

fn convert_posting(posting: bonk_ast::Posting) -> bonk_ast_errorless::Posting {
    bonk_ast_errorless::Posting {
        account: posting
            .account()
            .map(convert_account)
            .expect("Couldn't get account (but tree had no errors?)"),
        amount: posting
            .amount()
            .map(convert_amount)
            .expect("Couldn't get amount (but tree had no errors?)"),
    }
}

fn convert_account(account: bonk_ast::Account) -> bonk_ast_errorless::Account {
    bonk_ast_errorless::Account {
        path: account
            .value()
            .split(':')
            .map(|s| s.to_string())
            .collect_vec(),
    }
}

fn convert_amount(amount: bonk_ast::Amount) -> bonk_ast_errorless::Amount {
    bonk_ast_errorless::Amount {
        cents: amount
            .value()
            .replace('.', "")
            .parse()
            .expect("Couldn't parse amount (but tree had no errors?)"),
    }
}

#[cfg(test)]
mod tests {
    use bonk_ast::SourceSpan;
    use tree_sitter::{Point, Range};

    use crate::check_syntax;

    #[test]
    fn test_no_errors() {
        let src = r#"2023-01-01 "Mcdonald's"
    expenses:fast_food         10.91
    liabilities:my_credit_card -10.91"#;

        let ledger = bonk_ast::Parser::new().parse(src, None);
        let ledger = check_syntax(ledger);

        assert!(ledger.is_ok());
    }

    #[test]
    fn test_error() {
        let src = r#"2023-01-01abc "Mcdonald's"
expenses:fast_food         10.91
liabilities:my_credit_card -10.91"#;

        let ledger = bonk_ast::Parser::new().parse(src, None);
        let ledger = check_syntax(ledger);

        let errors = ledger.err().unwrap().0;
        assert_eq!(
            errors,
            vec![SourceSpan(Range {
                start_byte: 10,
                end_byte: 13,
                start_point: Point { row: 0, column: 10 },
                end_point: Point { row: 0, column: 13 }
            })]
        )
    }
}
