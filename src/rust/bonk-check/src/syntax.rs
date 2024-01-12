use bonk_ast_errorless::Date;

#[derive(Debug)]
pub struct AstHasErrors;

pub fn check_syntax(ledger: bonk_ast::Ledger) -> Result<bonk_ast_errorless::Ledger, AstHasErrors> {
    Ok(bonk_ast_errorless::Ledger {
        transactions: ledger
            .transactions()
            .into_iter()
            .map(check_syntax_transaction)
            .collect::<Result<Vec<_>, _>>()?,
    })
}

fn check_syntax_transaction(
    value: bonk_ast::Transaction<'_>,
) -> Result<bonk_ast_errorless::Transaction, AstHasErrors> {
    Ok(bonk_ast_errorless::Transaction {
        date: value.date().and_then(Date::parse).ok_or(AstHasErrors)?,
        description: value.description().ok_or(AstHasErrors)?.to_string(),
        postings: value
            .postings()
            .into_iter()
            .map(check_syntax_posting)
            .collect::<Result<Vec<_>, _>>()?,
    })
}

fn check_syntax_posting(
    value: bonk_ast::Posting<'_>,
) -> Result<bonk_ast_errorless::Posting, AstHasErrors> {
    Ok(bonk_ast_errorless::Posting {
        account: value
            .account()
            .ok_or(AstHasErrors)
            .and_then(check_syntax_account)?,
        amount: value
            .amount()
            .ok_or(AstHasErrors)
            .and_then(check_syntax_amount)?,
    })
}

fn check_syntax_account(
    value: bonk_ast::Account<'_>,
) -> Result<bonk_ast_errorless::Account, AstHasErrors> {
    Ok(bonk_ast_errorless::Account {
        path: value.path().iter().map(|s| s.to_string()).collect(),
    })
}

fn check_syntax_amount(
    value: bonk_ast::Amount<'_>,
) -> Result<bonk_ast_errorless::Amount, AstHasErrors> {
    Ok(bonk_ast_errorless::Amount {
        cents: value.cents(),
    })
}
