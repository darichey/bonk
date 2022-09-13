use std::{fs::File, str::FromStr};

use anyhow::Result;
use chrono::NaiveDate;
use futures_util::TryFutureExt;
use serde::Deserialize;
use sqlx::{sqlite::SqliteConnectOptions, ConnectOptions, Connection, Row, SqliteConnection};

#[derive(Debug, sqlx::Type)]
#[repr(transparent)]
struct AccountId(i64);

#[derive(Debug, sqlx::FromRow)]
#[allow(dead_code)]
struct Account {
    id: AccountId,
    name: String,
}

#[derive(Debug, sqlx::Type)]
#[repr(transparent)]
struct TransactionId(i64);

#[derive(Debug, sqlx::FromRow)]
#[allow(dead_code)]
struct Transaction {
    id: TransactionId,
    date: NaiveDate,
    description: String,
    from: AccountId,
    to: AccountId,
    amount: u64,
}

#[derive(Debug, Deserialize)]
struct Record {
    date: String,
    description: String,
    amount: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let mut conn = SqliteConnectOptions::from_str("sqlite::memory:")?
        .create_if_missing(true)
        .connect()
        .await?;

    sqlx::query(include_str!("../sql/schema.sql"))
        .execute(&mut conn)
        .await?;

    sqlx::query(r#"insert into "account" values (0, 'From')"#)
        .execute(&mut conn)
        .await?;

    sqlx::query(r#"insert into "account" values (1, 'To')"#)
        .execute(&mut conn)
        .await?;

    let mut transaction = conn.begin().await?;

    let mut rdr = csv::Reader::from_reader(File::open("./data/2021.csv")?);

    for result in rdr.deserialize() {
        let record: Record = result?;
        sqlx::query(r#"insert into "transaction" ("date", "description", "from", "to", "amount") values (?, ?, ?, ?, ?)"#)
            .bind(record.date)
            .bind(record.description)
            .bind(0)
            .bind(1)
            .bind(record.amount)
            .execute(&mut transaction)
            .await?;
    }

    transaction.commit().await?;

    let x = sqlx::query(r#"select count(*) as "count" from "transaction""#)
        .fetch_one(&mut conn)
        .await?;

    let count: i64 = x.get_unchecked("count");

    println!("{}", count);

    Ok(())
}
