use std::{fs, path::Path};

use dashboard::Dashboard;
use serde::Deserialize;

pub mod dashboard;

#[derive(Deserialize)]
pub struct BonkCfg {
    pub include: String,

    #[serde(default)]
    pub dashboards: Vec<Dashboard>,
}

impl BonkCfg {
    // FIXME: errors
    pub fn read<T: AsRef<Path>>(cfg_path: T) -> Result<Self, String> {
        let s = fs::read_to_string(cfg_path).map_err(|err| err.to_string())?;
        toml::from_str(&s).map_err(|err| err.to_string())
    }
}
