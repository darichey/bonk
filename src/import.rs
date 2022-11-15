use std::{fs::File};

use anyhow::Result;
use sqlx::{Connection, SqliteConnection};

use crate::Record;

pub async fn import(conn: &mut SqliteConnection) -> Result<()> {
    let mut transaction = conn.begin().await?;

    sqlx::query(include_str!("../sql/schema.sql"))
        .execute(&mut transaction)
        .await?;

    sqlx::query(r#"insert into "account" values (0, 'Source')"#)
        .execute(&mut transaction)
        .await?;

    sqlx::query(r#"insert into "account" values (1, 'Destination')"#)
        .execute(&mut transaction)
        .await?;

    let mut rdr = csv::Reader::from_reader(File::open("./data/2021.csv")?);

    for result in rdr.deserialize() {
        let record: Record = result?;
        sqlx::query(r#"insert into "transaction" ("date", "description", "source", "destination", "amount") values (?, ?, ?, ?, ?)"#)
            .bind(record.date)
            .bind(record.description)
            .bind(0)
            .bind(1)
            .bind(record.amount)
            .execute(&mut transaction)
            .await?;
    }

    transaction.commit().await?;

    Ok(())
}
