use std::{fs::File, str::FromStr};

use anyhow::Result;
use serde::Deserialize;
use sqlx::{sqlite::SqliteConnectOptions, ConnectOptions, Connection, Row};

mod import;
mod model;

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

    import::import(&mut conn).await?;

    let x = sqlx::query(r#"select count(*) as "count" from "transaction""#)
        .fetch_one(&mut conn)
        .await?;

    let count: i64 = x.get_unchecked("count");

    println!("{}", count);

    Ok(())
}
