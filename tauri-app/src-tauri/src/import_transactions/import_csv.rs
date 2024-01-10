use std::fs::File;

use anyhow::{Context, Result};
use chrono::NaiveDate;
use csv::StringRecord;

use crate::db::{DollarAmount, Transaction};

use super::get_transaction_id;

pub type ColParser<T> = (usize, fn(&str) -> Result<T>);

pub struct TransactionRowParser {
    pub date: ColParser<NaiveDate>,
    pub description: ColParser<String>,
    pub amount: ColParser<DollarAmount>,
}

pub fn import_csv_reader(
    csv_reader: &mut csv::Reader<File>,
    row_parser: TransactionRowParser,
    account: &str,
) -> Result<Vec<Transaction>> {
    import_csv_records(csv_reader.records(), row_parser, account)
}

pub fn import_csv_records(
    records: impl Iterator<Item = Result<StringRecord, csv::Error>>,
    row_parser: TransactionRowParser,
    account: &str,
) -> Result<Vec<Transaction>> {
    records
        .map(|result| {
            let row = result?;
            let row_index = row
                .position()
                .with_context(|| "Couldn't get row position")?
                .record();
            let date = parse_col("date", &row, &row_parser.date)?;
            let description = parse_col("description", &row, &row_parser.description)?;
            let amount = parse_col("amount", &row, &row_parser.amount)?;
            let account = account.to_string();
            Ok(Transaction {
                id: get_transaction_id(row_index, &date, &description, &amount, &account),
                date,
                description,
                amount,
                account,
            })
        })
        .collect()
}

fn parse_col<T>(name: &str, row: &StringRecord, parser: &ColParser<T>) -> Result<T> {
    let (idx, parser) = parser;
    parser(
        row.get(*idx)
            .with_context(|| format!("{name} not present"))?,
    )
}
