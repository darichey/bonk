#![feature(iterator_try_collect)]

mod db;
mod import;

use anyhow::Result;
use db::Transaction;
use import::Importer;

fn main() -> Result<()> {
    let importers: Vec<Box<dyn Importer>> = vec![
        Box::new(import::UsaaCsvImporter {
            path: "./data/raw/2022-01-01 USAA Platinum Visa.csv".to_owned(),
        }),
        Box::new(import::IdCsvImporter {
            path: "./data/transactions.csv".to_owned(),
        }),
    ];
    let db = db::Db::new("./db/schema.sql", importers)?;
    for Transaction {
        date,
        description,
        amount,
    } in db.get_transactions()?
    {
        println!("{},{},{}", date, description, amount);
    }
    Ok(())
}
