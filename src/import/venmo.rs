use crate::{
    db::{DollarAmount, Transaction},
    import::import_csv::{import_csv_records, TransactionRowParser},
};
use anyhow::{Context, Result};
use chrono::NaiveDateTime;
use csv::ReaderBuilder;

use super::import_csv::ColParser;

pub fn import_venmo_csv(account: &str, path: &str) -> Result<Vec<Transaction>> {
    let mut csv_reader = ReaderBuilder::new().has_headers(false).from_path(path)?;
    let records = {
        let mut records: Vec<_> = csv_reader.records().skip(4).collect();
        records.truncate(records.len() - 1);
        records.into_iter()
    };

    import_csv_records(
        records,
        TransactionRowParser {
            date: ColParser::Field(2, |s| {
                Ok(NaiveDateTime::parse_from_str(s, "%Y-%m-%dT%H:%M:%S")
                    .with_context(|| format!("Couldn't parse: {s}"))?
                    .date())
            }),
            description: ColParser::Field(5, |s| Ok(s.to_string())),
            amount: ColParser::Field(8, |s| DollarAmount::parse(&s.replace(" $", ""))),
        },
        account,
    )
}
