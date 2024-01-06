use std::fmt::Display;

use crate::{Account, Amount, Ledger, Posting, Transaction};

impl Display for Ledger {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = self
            .transactions
            .iter()
            .map(|t| format!("{t}\n"))
            .collect::<Vec<_>>()
            .join("\n");

        f.write_str(&s)?;

        Ok(())
    }
}

impl Display for Transaction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let date = self.date.to_string();
        let description = format!("\"{}\"", &self.description);
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
    let amount = to_string_amount(&p.amount);
    format!("{account} {amount}")
}

fn to_string_account(account: &Account) -> String {
    account.path.join(":")
}

fn to_string_amount(amount: &Amount) -> String {
    format!("{}.{}", amount.cents / 100, (amount.cents % 100).abs())
}

#[cfg(test)]
mod tests {
    use chrono::NaiveDate;

    use crate::{Account, Amount, Ledger, Posting, Transaction};

    #[test]
    fn test() {
        let ledger = Ledger {
            transactions: vec![
                Transaction {
                    date: NaiveDate::from_ymd_opt(2023, 1, 1).unwrap(),
                    description: "some food".to_string(),
                    postings: vec![
                        Posting {
                            account: Account {
                                path: vec!["expenses".to_string(), "food".to_string()],
                            },
                            amount: Amount { cents: 1234 },
                        },
                        Posting {
                            account: Account {
                                path: vec!["liabilities".to_string(), "my_credit_card".to_string()],
                            },
                            amount: Amount { cents: -1234 },
                        },
                    ],
                },
                Transaction {
                    date: NaiveDate::from_ymd_opt(2023, 1, 2).unwrap(),
                    description: "paying credit card".to_string(),
                    postings: vec![
                        Posting {
                            account: Account {
                                path: vec!["liabilities".to_string(), "my_credit_card".to_string()],
                            },
                            amount: Amount { cents: 1234 },
                        },
                        Posting {
                            account: Account {
                                path: vec!["assets".to_string(), "my_checking".to_string()],
                            },
                            amount: Amount { cents: -1234 },
                        },
                    ],
                },
            ],
        };

        assert_eq!(
            ledger.to_string(),
            r#"2023-01-01 "some food"
  expenses:food 12.34
  liabilities:my_credit_card -12.34

2023-01-02 "paying credit card"
  liabilities:my_credit_card 12.34
  assets:my_checking -12.34
"#
        );
    }
}
