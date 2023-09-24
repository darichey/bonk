use std::{fmt::Display, fs};

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use sqlite::{Connection, State, Value};

use anyhow::Result;

use crate::import::import_all;

pub struct Db {
    con: Connection,
}

pub struct Statement<'a> {
    stmt: sqlite::Statement<'a>,
}

impl Statement<'_> {
    pub fn column_names(&self) -> Vec<String> {
        self.stmt
            .column_names()
            .into_iter()
            .map(|name| name.clone())
            .collect()
    }
}

impl Db {
    pub fn new(path_to_schema: &str, path_to_data: &str) -> Result<Db> {
        // open db connection
        let con = sqlite::open(":memory:")?;

        // create transactions table
        let query = fs::read_to_string(path_to_schema)?;
        con.execute(query)?;

        // insert some transactions
        con.execute("BEGIN TRANSACTION")?;

        for Transaction {
            date,
            description,
            amount,
            account,
        } in import_all(path_to_data)?
        {
            let mut stmt = con.prepare(
                "INSERT INTO transactions VAlUES (:date, :description, :amount, :account)",
            )?;

            stmt.bind::<&[(_, Value)]>(
                &[
                    (":date", date.to_string().into()),
                    (":description", description.into()),
                    (":amount", amount.cents.into()),
                    (":account", account.into()),
                ][..],
            )?;

            while let Ok(State::Row) = stmt.next() {}
        }

        con.execute("COMMIT")?;

        Ok(Db { con })
    }

    pub fn get_transactions(&self, stmt: Statement<'_>) -> Result<Vec<Transaction>> {
        self.query(stmt)
            .map(|row| {
                let row = row?;
                Ok(Transaction {
                    date: NaiveDate::parse_from_str(row.try_read::<&str, _>("date")?, "%Y-%m-%d")?,
                    description: row.try_read::<&str, _>("description")?.to_string(),
                    amount: DollarAmount {
                        cents: row.try_read::<i64, _>("amount")?,
                    },
                    account: row.try_read::<&str, _>("account")?.to_string(),
                })
            })
            .collect()
    }

    pub fn prepare(&self, query: &str) -> Result<Statement<'_>> {
        Ok(Statement {
            stmt: self.con.prepare(query)?,
        })
    }

    pub fn prepare_bind<T: sqlite::Bindable>(
        &self,
        query: &str,
        value: T,
    ) -> Result<Statement<'_>> {
        let mut stmt = self.prepare(query)?;
        stmt.stmt.bind(value)?;
        Ok(stmt)
    }

    pub fn query<'a>(
        &'a self,
        stmt: Statement<'a>,
    ) -> impl Iterator<Item = Result<sqlite::Row>> + 'a {
        stmt.stmt.into_iter().map(|row| Ok(row?))
    }
}

#[derive(Serialize, Deserialize)]
pub struct Transaction {
    pub date: NaiveDate,
    pub description: String,
    pub amount: DollarAmount,
    pub account: String,
}

#[derive(Serialize, Deserialize)]
pub struct DollarAmount {
    pub cents: i64,
}

impl DollarAmount {
    // TODO: replace with TryFrom<&str>, need to enumerate error cases which is good anyways
    pub fn parse(s: &str) -> Result<DollarAmount> {
        let negative = s.starts_with('-');
        let (dollars, cents) = s.split_once('.').unwrap_or((s, "0"));
        let dollars: i64 = str::parse(&dollars.replace('-', ""))?;
        let cents: i64 = str::parse(cents)?;
        let cents = dollars * 100 + cents;
        let cents = if negative { -cents } else { cents };
        Ok(DollarAmount { cents })
    }
}

impl Display for DollarAmount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.cents)
    }
}
