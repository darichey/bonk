pub mod cli;

use core::fmt;
use std::path::{Path, PathBuf};

use anyhow::{anyhow, bail};
use bonk_ast_errorless::{Posting, Transaction};
use bonk_check::{CheckedWorkspace, WorkspaceExt as _};
use bonk_parse::WorkspaceExt as _;
use bonk_workspace::Workspace;
use sqlite::{Connection, State, Value};

pub fn create_db(cfg: PathBuf, database: PathBuf) -> anyhow::Result<Db> {
    // TODO: can we dedupe this with bonk_check::cli::run ?
    let workspace = Workspace::from_cfg(cfg).map_err(|err| anyhow!(err))?;
    let workspace = workspace.parse()?;
    let workspace = workspace.check().map_err(|err| {
        // TODO: pretty print errors with miette or something like it
        anyhow!(err
            .into_iter()
            .map(|err| format!("{:?}", err))
            .collect::<Vec<_>>()
            .join("\n"))
    })?;

    Db::new(&workspace, database)
}

pub struct Db {
    pub con: Connection,
}

impl Db {
    pub fn new<T: AsRef<Path>>(workspace: &CheckedWorkspace, path: T) -> anyhow::Result<Db> {
        let con = Connection::open(path)?;

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
        )?;

        con.execute("BEGIN TRANSACTION")?;

        {
            let mut insert_transaction =
                con.prepare(r#"INSERT INTO "transaction" VALUES (:id, :date, :description)"#)?;

            let mut insert_posting =
                con.prepare(r#"INSERT INTO "posting" VALUES (:transaction, :account, :amount)"#)?;

            for (_, ledger) in workspace.ledgers() {
                for Transaction {
                    date,
                    description,
                    postings,
                    ..
                } in ledger.transactions.iter()
                {
                    insert_transaction.bind::<&[(_, Value)]>(
                        &[
                            (":date", date.to_string().into()),
                            (":description", description.to_string().into()),
                        ][..],
                    )?;

                    let Ok(State::Done) = insert_transaction.next() else {
                        bail!("unexpected state while inserting transaction. Expected Done")
                    };

                    insert_transaction.reset()?;

                    // TODO: https://github.com/stainless-steel/sqlite/pull/84
                    let transaction_id =
                        unsafe { sqlite3_sys::sqlite3_last_insert_rowid(con.as_raw()) };

                    for Posting {
                        account, amount, ..
                    } in postings
                    {
                        let account = account.path_string();
                        let amount = amount
                            .as_ref()
                            .map(|a| a.cents)
                            .unwrap_or_else(|| infer_amount(postings))
                            .to_string();

                        insert_posting.bind::<&[(_, Value)]>(
                            &[
                                (":transaction", transaction_id.into()),
                                (":account", account.into()),
                                (":amount", amount.into()),
                            ][..],
                        )?;

                        let Ok(State::Done) = insert_posting.next() else {
                            bail!("unexpected state while inserting posting. Expected Done")
                        };

                        insert_posting.reset()?;
                    }
                }
            }
        }

        con.execute("COMMIT")?;

        Ok(Self { con })
    }

    pub fn query(&self, query: &str) -> anyhow::Result<QueryOutput> {
        let stmt = self.con.prepare(query)?;

        let column_names = stmt.column_names().to_vec();

        let data = stmt
            .into_iter()
            .map(|row| {
                let values: Vec<sqlite::Value> = row?.into();
                let values: Vec<SqlValue> = values.into_iter().map(SqlValue).collect();
                Ok(values)
            })
            .collect::<anyhow::Result<Vec<Vec<SqlValue>>>>()?;

        Ok(QueryOutput { column_names, data })
    }
}

// TODO: this shouldn't live here. There should be a phase in checking that infers amounts
fn infer_amount(postings: &[Posting]) -> i32 {
    let sum: i32 = postings
        .iter()
        .filter_map(|posting| posting.amount.as_ref().map(|a| a.cents))
        .sum();

    -sum
}

pub struct QueryOutput {
    pub column_names: Vec<String>,
    pub data: Vec<Vec<SqlValue>>,
}

/// A wrapper for seralizing sqlite Values
pub struct SqlValue(pub sqlite::Value);

impl serde::Serialize for SqlValue {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match &self.0 {
            sqlite::Value::Binary(b) => serializer.serialize_bytes(b),
            sqlite::Value::Float(f) => serializer.serialize_f64(*f),
            sqlite::Value::Integer(i) => serializer.serialize_i64(*i),
            sqlite::Value::String(s) => serializer.serialize_str(s),
            sqlite::Value::Null => serializer.serialize_unit(),
        }
    }
}

impl fmt::Display for SqlValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.0 {
            Value::Binary(b) => write!(f, "{:?}", b),
            Value::Float(fl) => write!(f, "{}", fl),
            Value::Integer(i) => write!(f, "{}", i),
            Value::String(s) => write!(f, "{}", s),
            Value::Null => write!(f, "null"),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{path::PathBuf, str::FromStr};

    use crate::Db;
    use bonk_ast_errorless::*;
    use bonk_check::CheckUnit;

    #[test]
    fn test() {
        let ledger = Ledger {
            declare_accounts: vec![],
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
                            amount: Some(Amount::from_dollars(12.34, None)),
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

        let expected_transactions = [
            ("2023-01-01", "some food"),
            ("2023-01-02", "paying credit card"),
        ];

        let expected_postings = [
            (1, "expenses/food", 1234),
            (1, "liabilities/my_credit_card", -1234),
            (2, "liabilities/my_credit_card", 1234),
            (2, "assets/my_checking", -1234),
        ];

        let db = Db::new(
            &CheckUnit::new(vec![(PathBuf::from_str("ledger.bonk").unwrap(), ledger)]),
            ":memory:",
        )
        .unwrap();

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
