use anyhow::Result;
use chrono::NaiveDate;
use csv::ReaderBuilder;

use crate::db::{DollarAmount, Transaction};

use super::import_csv::{import_csv, TransactionRowParser};

/// Imports the old usaa csv format that doesn't include a header
pub fn import_old_usaa_csv(path: &str) -> Result<Vec<Transaction>> {
    let mut csv_reader = ReaderBuilder::new().has_headers(false).from_path(path)?;

    import_csv(
        &mut csv_reader,
        TransactionRowParser {
            date: (2, |s| Ok(NaiveDate::parse_from_str(s, "%m/%d/%Y")?)),
            description: (4, |s| Ok(s.to_string())),
            amount: (6, DollarAmount::parse),
        },
    )
}

/// Imports the new usaa csv format that does include a header
pub fn import_new_usaa_csv(path: &str) -> Result<Vec<Transaction>> {
    let mut csv_reader = csv::Reader::from_path(path)?;

    import_csv(
        &mut csv_reader,
        TransactionRowParser {
            date: (0, |s| Ok(NaiveDate::parse_from_str(s, "%Y-%m-%d")?)),
            description: (1, |s| Ok(s.to_string())),
            amount: (4, DollarAmount::parse),
        },
    )
}
