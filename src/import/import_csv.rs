use std::fs::File;

use anyhow::{Context, Result};
use chrono::NaiveDate;
use csv::StringRecord;

use crate::db::{DollarAmount, Transaction};

pub enum ColParser<T> {
    Field(usize, fn(&str) -> Result<T>),
    Row(fn(&StringRecord) -> Result<T>),
}

pub struct TransactionRowParser {
    pub date: ColParser<NaiveDate>,
    pub description: ColParser<String>,
    pub amount: ColParser<DollarAmount>,
}

pub fn import_csv_reader(
    csv_reader: &mut csv::Reader<File>,
    row_parser: TransactionRowParser,
) -> Result<Vec<Transaction>> {
    import_csv_records(csv_reader.records(), row_parser)
}

pub fn import_csv_records(
    records: impl Iterator<Item = Result<StringRecord, csv::Error>>,
    row_parser: TransactionRowParser,
) -> Result<Vec<Transaction>> {
    records
        .map(|result| {
            let row = result?;
            Ok(Transaction {
                date: parse_col("date", &row, &row_parser.date)?,
                description: parse_col("description", &row, &row_parser.description)?,
                amount: parse_col("amount", &row, &row_parser.amount)?,
            })
        })
        .collect()
}

fn parse_col<T>(name: &str, row: &StringRecord, parser: &ColParser<T>) -> Result<T> {
    match parser {
        ColParser::Field(idx, parser) => parser(
            row.get(*idx)
                .with_context(|| format!("{name} not present"))?,
        ),
        ColParser::Row(parser) => parser(row),
    }
}
