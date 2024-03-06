use std::fmt::Display;

use crate::{Account, Amount, Ledger, Posting, Transaction};

impl Display for Ledger {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let transactions = self
            .transactions
            .iter()
            .map(|t| format!("{t}\n"))
            .collect::<Vec<_>>()
            .join("\n");

        f.write_str(&transactions)?;

        Ok(())
    }
}

impl Display for Transaction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let date = self.date.to_string();
        let description = format!("\"{}\"", escape(&self.description));
        let postings = self
            .postings
            .iter()
            .map(|p| format!("  {}", to_string_posting(p)))
            .collect::<Vec<_>>()
            .join("\n");
        let s = format!("{date} {description}\n{postings}");

        f.write_str(&s)?;

        Ok(())
    }
}

fn to_string_posting(p: &Posting) -> String {
    let account = to_string_account(&p.account);
    match &p.amount {
        Some(amount) => {
            let amount = to_string_amount(amount);
            format!("{account} {amount}")
        }
        None => account.to_string(),
    }
}

fn to_string_account(account: &Account) -> String {
    account.path_string()
}

fn to_string_amount(amount: &Amount) -> String {
    format!("{}.{:02}", amount.cents / 100, (amount.cents % 100).abs())
}

// TODO: test
fn escape(s: &str) -> String {
    s.replace('\"', "\\\"").replace('\n', "\\n")
}

#[cfg(test)]
mod tests {

    use crate::{Account, Amount, Date, DeclareAccount, Ledger, Posting, Transaction};

    #[test]
    fn test() {
        let ledger = Ledger {
            declare_accounts: vec![
                DeclareAccount {
                    account: Account::parse("expenses/fast_food", None),
                    source: None,
                },
                DeclareAccount {
                    account: Account::parse("liabilities/my_credit_card", None),
                    source: None,
                },
                DeclareAccount {
                    account: Account::parse("assets/my_checking", None),
                    source: None,
                },
            ],
            transactions: vec![
                Transaction {
                    date: Date::new(2023, 1, 1),
                    description: "some food".to_string(),
                    postings: vec![
                        Posting {
                            account: Account::parse("expenses/food", None),
                            amount: Some(Amount::from_dollars(12.34, None)),
                            source: None,
                        },
                        Posting {
                            account: Account::parse("liabilities/my_credit_card", None),
                            amount: Some(Amount::from_dollars(-12.34, None)),
                            source: None,
                        },
                    ],
                    source: None,
                },
                Transaction {
                    date: Date::new(2023, 1, 2),
                    description: "paying credit card".to_string(),
                    postings: vec![
                        Posting {
                            account: Account::parse("liabilities/my_credit_card", None),
                            amount: Some(Amount::from_dollars(12.05, None)),
                            source: None,
                        },
                        Posting {
                            account: Account::parse("assets/my_checking", None),
                            amount: None,
                            source: None,
                        },
                    ],
                    source: None,
                },
            ],
            source: None,
        };

        insta::assert_display_snapshot!(
            ledger,
            @r###"
        2023-01-01 "some food"
          expenses/food 12.34
          liabilities/my_credit_card -12.34

        2023-01-02 "paying credit card"
          liabilities/my_credit_card 12.05
          assets/my_checking
        "###
        );
    }
}
