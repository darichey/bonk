use anyhow::{Context, Result};
use chrono::NaiveDate;
use csv::ReaderBuilder;

use crate::db::{DollarAmount, Transaction};

pub trait Importer {
    fn get_transactions(&self) -> Result<Vec<Transaction>>;
}

pub struct IdCsvImporter {
    pub path: String,
}

impl Importer for IdCsvImporter {
    fn get_transactions(&self) -> Result<Vec<Transaction>> {
        let mut csv_reader = csv::Reader::from_path(&self.path)?;
        csv_reader
            .records()
            .map(|result| {
                let row = result?;

                Ok(Transaction {
                    date: NaiveDate::parse_from_str(
                        row.get(0).context("Date not present")?,
                        "%Y-%m-%d",
                    )?,
                    description: row.get(1).context("Description not present")?.to_owned(),
                    amount: DollarAmount::parse(row.get(2).context("Amount not present")?)?,
                })
            })
            .collect::<Result<Vec<Transaction>>>()
    }
}

pub struct UsaaCsvImporter {
    pub path: String,
}

impl Importer for UsaaCsvImporter {
    fn get_transactions(&self) -> Result<Vec<Transaction>> {
        let mut csv_reader = ReaderBuilder::new()
            .has_headers(false)
            .from_path(&self.path)?;

        csv_reader
            .records()
            .map(|result| {
                let row = result?;

                Ok(Transaction {
                    date: NaiveDate::parse_from_str(
                        row.get(2).context("Date not present")?,
                        "%m/%d/%Y",
                    )?,
                    description: row.get(4).context("Description not present")?.to_owned(),
                    amount: DollarAmount::parse(row.get(6).context("Amount not present")?)?,
                })
            })
            .collect::<Result<Vec<Transaction>>>()
    }
}

pub struct MultiImporter {
    pub new_importer: fn(String) -> Box<dyn Importer>,
    pub paths: Vec<String>,
}

impl Importer for MultiImporter {
    fn get_transactions(&self) -> Result<Vec<Transaction>> {
        let mut transactions = Vec::new();
        for path in &self.paths {
            transactions.extend((self.new_importer)(path.clone()).get_transactions()?)
        }
        Ok(transactions)
    }
}
