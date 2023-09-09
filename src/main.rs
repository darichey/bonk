#![feature(iterator_try_collect)]

mod db;
mod import;

use anyhow::Result;
use db::Transaction;
use glob::glob;
use import::{Importer, MultiImporter, UsaaCsvImporter};

fn main() -> Result<()> {
    // let importers: Vec<Box<dyn Importer>> = vec![
    //     Box::new(import::UsaaCsvImporter {
    //         path: "./data/raw/2022-01-01 USAA Platinum Visa.csv".to_owned(),
    //     }),
    //     Box::new(import::IdCsvImporter {
    //         path: "./data/transactions.csv".to_owned(),
    //     }),
    // ];

    let importers: Vec<Box<dyn Importer>> = vec![Box::new(MultiImporter {
        new_importer: |path| Box::new(UsaaCsvImporter { path }),
        paths: glob("./data/raw/*.csv")?
            .map(|path| Ok(String::from(path?.to_string_lossy())))
            .collect::<Result<Vec<_>>>()?,
    })];
    let db = db::Db::new("./db/schema.sql", importers)?;
    // for Transaction {
    //     date,
    //     description,
    //     amount,
    // } in db.get_transactions()?
    // {
    //     println!("{},{},{}", date, description, amount);
    // }
    println!("{}", db.get_transactions()?.len());
    Ok(())
}
