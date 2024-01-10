use std::path::Path;

use bonk_ast_errorless::{Ledger, Posting, Transaction};
use sqlite::{Connection, State, Value};

pub struct Db {
    pub con: Connection,
}

impl Db {
    pub fn new<T: AsRef<Path>>(ledger: &Ledger, path: T) -> Result<Db, String> {
        let con = Connection::open(path).map_err(|err| err.to_string())?;

        con.execute(
            r#"CREATE TABLE "transaction" (
            "id" INTEGER PRIMARY KEY,
            "date" DATE NOT NULL,
            "description" TEXT NOT NULL
        );
        
        create table "posting" (
            "transaction" INTEGER,
            "account" TEXT NOT NULL,
            "amount" INTEGER NOT NULL,
            FOREIGN KEY("transaction") REFERENCES "transaction"("id")
        );"#,
        )
        .map_err(|err| err.to_string())?;

        con.execute("BEGIN TRANSACTION")
            .map_err(|err| err.to_string())?;

        {
            let mut insert_transaction = con
                .prepare(r#"INSERT INTO "transaction" VALUES (:id, :date, :description)"#)
                .map_err(|err| err.to_string())?;

            let mut insert_posting = con
                .prepare(r#"INSERT INTO "posting" VALUES (:transaction, :account, :amount)"#)
                .map_err(|err| err.to_string())?;

            for Transaction {
                date,
                description,
                postings,
            } in ledger.transactions.iter()
            {
                insert_transaction
                    .bind::<&[(_, Value)]>(
                        &[
                            (":date", date.to_string().into()),
                            (":description", description.to_string().into()),
                        ][..],
                    )
                    .map_err(|err| err.to_string())?;

                let Ok(State::Done) = insert_transaction.next() else {
                    return Err(
                        "unexpected state while inserting transaction. Expected Done".to_string(),
                    );
                };

                insert_transaction.reset().map_err(|err| err.to_string())?;

                // TODO: https://github.com/stainless-steel/sqlite/pull/84
                let transaction_id =
                    unsafe { sqlite3_sys::sqlite3_last_insert_rowid(con.as_raw()) };

                for Posting { account, amount } in postings {
                    let account = account.path.join(":");
                    let amount = amount.cents.to_string();

                    insert_posting
                        .bind::<&[(_, Value)]>(
                            &[
                                (":transaction", transaction_id.into()),
                                (":account", account.into()),
                                (":amount", amount.into()),
                            ][..],
                        )
                        .map_err(|err| err.to_string())?;

                    let Ok(State::Done) = insert_posting.next() else {
                        return Err(
                            "unexpected state while inserting posting. Expected Done".to_string()
                        );
                    };

                    insert_posting.reset().map_err(|err| err.to_string())?;
                }
            }
        }

        con.execute("COMMIT").map_err(|err| err.to_string())?;

        Ok(Self { con })
    }
}

#[cfg(test)]
mod tests {
    use bonk_ast_errorless::*;
    use chrono::NaiveDate;
    use sqlite::{Error, Row};

    use crate::Db;

    #[test]
    fn test() {
        let ledger = Ledger {
            transactions: vec![
                Transaction {
                    date: NaiveDate::from_ymd_opt(2023, 1, 1).unwrap(),
                    description: "some food".to_string(),
                    postings: vec![
                        Posting {
                            account: Account::parse("expenses:food"),
                            amount: Amount { cents: 1234 },
                        },
                        Posting {
                            account: Account::parse("liabilities:my_credit_card"),
                            amount: Amount { cents: -1234 },
                        },
                    ],
                },
                Transaction {
                    date: NaiveDate::from_ymd_opt(2023, 1, 2).unwrap(),
                    description: "paying credit card".to_string(),
                    postings: vec![
                        Posting {
                            account: Account::parse("liabilities:my_credit_card"),
                            amount: Amount { cents: 1234 },
                        },
                        Posting {
                            account: Account::parse("assets:my_checking"),
                            amount: Amount { cents: -1234 },
                        },
                    ],
                },
            ],
        };

        let expected_transactions = [
            ("2023-01-01", "some food"),
            ("2023-01-02", "paying credit card"),
        ];

        let expected_postings = [
            (1, "expenses:food", 1234),
            (1, "liabilities:my_credit_card", -1234),
            (2, "liabilities:my_credit_card", 1234),
            (2, "assets:my_checking", -1234),
        ];

        let db = Db::new(&ledger, ":memory:").unwrap();

        let mut stmt = db.con.prepare(r#"SELECT * FROM "transaction""#).unwrap();

        for (row, (date, description)) in stmt.iter().zip(expected_transactions) {
            let row = row.unwrap();
            assert_eq!(row.read::<&str, &str>("date"), date);
            assert_eq!(row.read::<&str, &str>("description"), description);
        }

        let mut stmt = db.con.prepare(r#"SELECT * FROM "posting""#).unwrap();

        for (row, (transaction, account, amount)) in stmt.iter().zip(expected_postings) {
            let row = row.unwrap();
            assert_eq!(row.read::<i64, &str>("transaction"), transaction);
            assert_eq!(row.read::<&str, &str>("account"), account);
            assert_eq!(row.read::<i64, &str>("amount"), amount);
        }
    }
}
