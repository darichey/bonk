mod id;
mod import_csv;
mod usaa;
mod venmo;

use std::{
    collections::{hash_map::DefaultHasher, HashMap},
    fs::{self, File},
    hash::{Hash, Hasher},
    path::Path,
};

use anyhow::{Context, Result};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::db::{DollarAmount, Transaction};

type Importer = fn(account: &str, path: &str) -> Result<Vec<Transaction>>;

lazy_static! {
    static ref IMPORTERS: HashMap<&'static str, Importer> = {
        let mut m: HashMap<&'static str, Importer> = HashMap::new();
        m.insert("IdCsvImporter", id::import_id_csv);
        m.insert("OldUsaaCsvImporter", usaa::import_old_usaa_csv);
        m.insert("NewUsaaCsvImporter", usaa::import_new_usaa_csv);
        m.insert("VenmoCsvImporter", venmo::import_venmo_csv);
        m
    };
}

pub fn import_all(path_to_data: impl AsRef<Path>) -> Result<Vec<Transaction>> {
    let cfg_glob = format!(
        "{}/transactions/**/.importercfg.json",
        path_to_data.as_ref().display()
    );
    let cfg_paths = glob::glob(&cfg_glob)?;

    let mut transactions = Vec::new();

    for cfg_path in cfg_paths {
        let cfg_path = cfg_path?;
        let cfg: ImporterConfig = serde_json::from_reader(File::open(&cfg_path)?)
            .with_context(|| format!("Failed to read {}", cfg_path.display()))?;

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
                &cfg.account,
                entry.path().to_str().context("Couldn't convert path")?,
            )?);
        }
    }

    Ok(transactions)
}

#[derive(Serialize, Deserialize, Debug)]
struct ImporterConfig {
    importer: String,
    account: String,
    ignore: bool,
}

pub fn get_transaction_id(
    row_index: u64,
    date: &NaiveDate,
    description: &String,
    amount: &DollarAmount,
    account: &String,
) -> i64 {
    let mut hasher = DefaultHasher::new();
    row_index.hash(&mut hasher);
    date.hash(&mut hasher);
    description.hash(&mut hasher);
    amount.hash(&mut hasher);
    account.hash(&mut hasher);
    hasher.finish() as i64
}
