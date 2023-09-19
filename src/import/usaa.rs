use anyhow::Result;
use chrono::NaiveDate;
use csv::ReaderBuilder;

use crate::db::{DollarAmount, Transaction};

use super::import_csv::{import_csv_reader, ColParser, TransactionRowParser};

/// Imports the old usaa csv format that doesn't include a header
pub fn import_old_usaa_csv(account: &str, path: &str) -> Result<Vec<Transaction>> {
    let mut csv_reader = ReaderBuilder::new().has_headers(false).from_path(path)?;

    import_csv_reader(
        &mut csv_reader,
        TransactionRowParser {
            date: ColParser::Field(2, |s| Ok(NaiveDate::parse_from_str(s, "%m/%d/%Y")?)),
            description: ColParser::Field(4, |s| Ok(s.to_string())),
            amount: ColParser::Field(6, DollarAmount::parse),
        },
        account,
    )
}

/// Imports the new usaa csv format that does include a header
pub fn import_new_usaa_csv(account: &str, path: &str) -> Result<Vec<Transaction>> {
    let mut csv_reader = csv::Reader::from_path(path)?;

    import_csv_reader(
        &mut csv_reader,
        TransactionRowParser {
            date: ColParser::Field(0, |s| Ok(NaiveDate::parse_from_str(s, "%Y-%m-%d")?)),
            description: ColParser::Field(1, |s| Ok(s.to_string())),
            amount: ColParser::Field(4, DollarAmount::parse),
        },
        account,
    )
}
