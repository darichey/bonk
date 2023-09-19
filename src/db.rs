use std::{fmt::Display, fs};

use chrono::NaiveDate;
use sqlite::{Connection, State, Value};

use anyhow::Result;

use crate::import::import_all;

pub struct Db {
    con: Connection,
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
        let query = "
            SELECT * FROM transactions ORDER BY date;
        ";
        let mut stmt = self.con.prepare(query)?;
        let mut vec = Vec::new();
        while let Ok(sqlite::State::Row) = stmt.next() {
            vec.push(Transaction {
                date: NaiveDate::parse_from_str(&stmt.read::<String, _>("date")?, "%Y-%m-%d")?,
                description: stmt.read::<String, _>("description")?,
                amount: DollarAmount {
                    cents: stmt.read::<i64, _>("amount")?,
                },
                account: stmt.read::<String, _>("account")?,
            })
        }
        Ok(vec)
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
