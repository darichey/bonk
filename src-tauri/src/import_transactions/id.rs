use anyhow::Result;
use chrono::NaiveDate;

use crate::db::{DollarAmount, Transaction};

use super::import_csv::{import_csv_reader, TransactionRowParser};

/// Imports fake test data that is just a csv that matches the db schema
pub fn import_id_csv(account: &str, path: &str) -> Result<Vec<Transaction>> {
    let mut csv_reader = csv::Reader::from_path(path)?;
    import_csv_reader(
        &mut csv_reader,
        TransactionRowParser {
            date: (0, |s| Ok(NaiveDate::parse_from_str(s, "%Y-%m-%d")?)),
            description: (1, |s| Ok(s.to_string())),
            amount: (2, DollarAmount::parse),
        },
        account,
    )
}
