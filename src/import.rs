use std::{
    collections::HashMap,
    fs::{self, File},
};

use anyhow::{Context, Result};
use chrono::NaiveDate;
use csv::ReaderBuilder;
use serde::{Deserialize, Serialize};

use crate::db::{DollarAmount, Transaction};

type Importer = fn(&str) -> Result<Vec<Transaction>>;

lazy_static! {
    static ref IMPORTERS: HashMap<&'static str, Importer> = {
        let mut m: HashMap<&'static str, Importer> = HashMap::new();
        m.insert("IdCsvImporter", import_id_csv);
        m.insert("OldUsaaCsvImporter", import_old_usaa_csv);
        m
    };
}

pub fn import_all(path_to_data: &str) -> Result<Vec<Transaction>> {
    let cfg_glob = format!("{path_to_data}/**/.importercfg.json");
    let cfg_paths = glob::glob(&cfg_glob)?;

    let mut transactions = Vec::new();

    for cfg_path in cfg_paths {
        let cfg_path = cfg_path?;
        let cfg: ImporterConfig = serde_json::from_reader(File::open(&cfg_path)?)?;
        let importer = IMPORTERS
            .get(cfg.importer.as_str())
            .with_context(|| format!("Couldn't find importer with name: {}", cfg.importer))?;

        let parent_dir = cfg_path
            .parent()
            .context(".importcfg.json doesn't have a parent dir")?;

        for entry in fs::read_dir(parent_dir)? {
            let entry = entry?;

            if entry.file_name() == ".importercfg.json" {
                continue;
            }

            transactions.extend(importer(
                entry.path().to_str().context("Couldn't convert path")?,
            )?);
        }
    }

    Ok(transactions)
}

fn import_id_csv(path: &str) -> Result<Vec<Transaction>> {
    let mut csv_reader = csv::Reader::from_path(path)?;
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

fn import_old_usaa_csv(path: &str) -> Result<Vec<Transaction>> {
    println!("{path}");
    let mut csv_reader = ReaderBuilder::new().has_headers(false).from_path(path)?;

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

#[derive(Serialize, Deserialize, Debug)]
struct ImporterConfig {
    importer: String,
}
