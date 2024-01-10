use std::{error::Error, io};

use bonk_ast_errorless::{Account, Amount, Date, Ledger, Posting, Transaction};
use csv::Reader;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct CsvTransaction {
    date: String,
    description: String,
    amount: f64,
}

pub fn do_import<D: io::Read>(
    account: &str,
    reader: &mut Reader<D>,
) -> Result<Ledger, Box<dyn Error>> {
    let account = Account::parse(account);

    let transactions = reader
        .deserialize()
        .map(|result| {
            let CsvTransaction {
                date,
                description,
                amount,
            } = result?;

            Ok(Transaction {
                date: Date::parse(&date).ok_or("Couldn't parse date")?,
                description,
                postings: vec![Posting {
                    account: account.clone(),
                    amount: Amount::from_dollars(amount),
                }],
            })
        })
        .collect::<Result<Vec<_>, Box<dyn Error>>>()?;

    Ok(Ledger { transactions })
}

#[cfg(test)]
mod tests {
    use bonk_ast_errorless::{Account, Amount, Date, Ledger, Posting, Transaction};

    use crate::do_import;

    #[test]
    fn test() {
        let input = br#"date,description,amount
2023-01-01,Salary Deposit,2500.00
2023-01-02,Grocery Shopping,-120.50"#;
        let mut reader = csv::Reader::from_reader(&input[..]);
        let ledger = do_import("assets:my_checking", &mut reader).unwrap();
        assert_eq!(
            ledger,
            Ledger {
                transactions: vec![
                    Transaction {
                        date: Date::new(2023, 1, 1),
                        description: "Salary Deposit".to_string(),
                        postings: vec![Posting {
                            account: Account::parse("assets:my_checking"),
                            amount: Amount::from_dollars(2500.00),
                        }]
                    },
                    Transaction {
                        date: Date::new(2023, 1, 2),
                        description: "Grocery Shopping".to_string(),
                        postings: vec![Posting {
                            account: Account::parse("assets:my_checking"),
                            amount: Amount::from_dollars(-120.50),
                        }]
                    }
                ]
            }
        )
    }
}
