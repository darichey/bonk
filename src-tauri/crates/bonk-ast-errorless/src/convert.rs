use chrono::NaiveDate;

#[derive(Debug)]
pub struct AstHasErrors;

impl TryFrom<bonk_ast::Ledger<'_>> for crate::Ledger {
    type Error = AstHasErrors;

    fn try_from(value: bonk_ast::Ledger<'_>) -> Result<Self, Self::Error> {
        Ok(crate::Ledger {
            transactions: value
                .transactions()
                .into_iter()
                .map(|t| t.try_into())
                .collect::<Result<Vec<_>, _>>()?,
        })
    }
}

impl TryFrom<bonk_ast::Transaction<'_>> for crate::Transaction {
    type Error = AstHasErrors;

    fn try_from(value: bonk_ast::Transaction<'_>) -> Result<Self, Self::Error> {
        Ok(crate::Transaction {
            date: NaiveDate::parse_from_str(value.date().ok_or(AstHasErrors)?, "%Y-%m-%d")
                .map_err(|_| AstHasErrors)?,
            description: value.description().ok_or(AstHasErrors)?.to_string(),
            postings: value
                .postings()
                .into_iter()
                .map(|p| p.try_into())
                .collect::<Result<Vec<_>, _>>()?,
        })
    }
}

impl TryFrom<bonk_ast::Posting<'_>> for crate::Posting {
    type Error = AstHasErrors;

    fn try_from(value: bonk_ast::Posting<'_>) -> Result<Self, Self::Error> {
        Ok(crate::Posting {
            account: value.account().ok_or(AstHasErrors)?.try_into()?,
            amount: value.amount().ok_or(AstHasErrors)?.try_into()?,
        })
    }
}

impl TryFrom<bonk_ast::Account<'_>> for crate::Account {
    type Error = AstHasErrors;

    fn try_from(value: bonk_ast::Account<'_>) -> Result<Self, Self::Error> {
        Ok(crate::Account {
            path: value.path().iter().map(|s| s.to_string()).collect(),
        })
    }
}

impl TryFrom<bonk_ast::Amount<'_>> for crate::Amount {
    type Error = AstHasErrors;

    fn try_from(value: bonk_ast::Amount<'_>) -> Result<Self, Self::Error> {
        Ok(crate::Amount {
            cents: value.cents(),
        })
    }
}
