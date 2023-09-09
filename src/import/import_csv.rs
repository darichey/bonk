use std::fs::File;

use anyhow::{Context, Result};
use chrono::NaiveDate;

use crate::db::{DollarAmount, Transaction};

pub type ColParser<T> = (usize, fn(&str) -> Result<T>);

pub struct TransactionRowParser {
    pub date: ColParser<NaiveDate>,
    pub description: ColParser<String>,
    pub amount: ColParser<DollarAmount>,
}

pub fn import_csv(
    csv_reader: &mut csv::Reader<File>,
    row_parser: TransactionRowParser,
) -> Result<Vec<Transaction>> {
    csv_reader
        .records()
        .map(|result| {
            let row = result?;
            Ok(Transaction {
                date: row_parser.date.1(row.get(row_parser.date.0).context("date not present")?)?,
                description: row_parser.description.1(
                    row.get(row_parser.description.0)
                        .context("description not present")?,
                )?,
                amount: row_parser.amount.1(
                    row.get(row_parser.amount.0).context("amount not present")?,
                )?,
            })
        })
        .collect()
}
