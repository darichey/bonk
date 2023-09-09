mod import_csv;

use std::{
    collections::HashMap,
    fs::{self, File},
};

use anyhow::{Context, Result};
use chrono::NaiveDate;
use csv::ReaderBuilder;
use serde::{Deserialize, Serialize};

use crate::db::{DollarAmount, Transaction};

use self::import_csv::{import_csv, TransactionRowParser};

type Importer = fn(&str) -> Result<Vec<Transaction>>;

lazy_static! {
    static ref IMPORTERS: HashMap<&'static str, Importer> = {
        let mut m: HashMap<&'static str, Importer> = HashMap::new();
        m.insert("IdCsvImporter", import_id_csv);
        m.insert("OldUsaaCsvImporter", import_old_usaa_csv);
        m.insert("NewUsaaCsvImporter", import_new_usaa_csv);
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

        if cfg.ignore {
            continue;
        }

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
    import_csv(
        &mut csv_reader,
        TransactionRowParser {
            date: (0, |s| Ok(NaiveDate::parse_from_str(s, "%Y-%m-%d")?)),
            description: (1, |s| Ok(s.to_string())),
            amount: (2, DollarAmount::parse),
        },
    )
}

fn import_old_usaa_csv(path: &str) -> Result<Vec<Transaction>> {
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

fn import_new_usaa_csv(path: &str) -> Result<Vec<Transaction>> {
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

#[derive(Serialize, Deserialize, Debug)]
struct ImporterConfig {
    importer: String,
    ignore: bool,
}
