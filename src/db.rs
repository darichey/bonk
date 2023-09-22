use std::{fmt::Display, fs};

use chrono::NaiveDate;
use sqlite::{Connection, State, Value};

use anyhow::Result;

use crate::import::import_all;

pub struct Db {
    con: Connection,
}

pub struct Statement<'a> {
    stmt: sqlite::Statement<'a>,
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

    pub fn get_transactions(&self) -> Result<Vec<Transaction>> {
        Ok(self
            .query(
                self.prepare("SELECT * FROM transactions ORDER BY date;")?,
                |row| {
                    Ok(Transaction {
                        date: NaiveDate::parse_from_str(
                            row.try_read::<&str, _>("date")?,
                            "%Y-%m-%d",
                        )?,
                        description: row.try_read::<&str, _>("description")?.to_string(),
                        amount: DollarAmount {
                            cents: row.try_read::<i64, _>("amount")?,
                        },
                        account: row.try_read::<&str, _>("account")?.to_string(),
                    })
                },
            )?
            .collect())
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

    pub fn query<'a, T, F>(
        &'a self,
        stmt: Statement<'a>,
        row_mapper: F,
    ) -> Result<impl Iterator<Item = T> + 'a>
    where
        F: Fn(sqlite::Row) -> Result<T> + 'static,
    {
        Ok(stmt
            .stmt
            .into_iter()
            .map(move |row| row_mapper(row.unwrap()).unwrap())) // TODO: unwrap
    }
}

pub struct Transaction {
    pub date: NaiveDate,
    pub description: String,
    pub amount: DollarAmount,
    pub account: String,
}

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
