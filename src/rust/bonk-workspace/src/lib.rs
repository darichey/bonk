use glob::glob;
use serde::Deserialize;
use std::{
    fs, io,
    path::{Path, PathBuf},
};

#[derive(Deserialize)]
pub struct BonkCfg {
    include: String,
}

impl BonkCfg {
    // FIXME: errors
    pub fn read(cfg_path: &Path) -> Result<Self, String> {
        let s = fs::read_to_string(cfg_path).map_err(|err| err.to_string())?;
        toml::from_str(&s).map_err(|err| err.to_string())
    }
}

pub struct Workspace {
    cfg_path: PathBuf,
    cfg: BonkCfg,
}

impl Workspace {
    pub fn from_cfg(cfg_path: &Path) -> Result<Self, String> {
        let cfg = BonkCfg::read(cfg_path)?;
        Ok(Workspace {
            cfg_path: cfg_path.to_path_buf(),
            cfg,
        })
    }

    pub fn included_paths(&self) -> impl Iterator<Item = PathBuf> + '_ {
        let root = PathBuf::from("/");
        let parent = self.cfg_path.parent().unwrap_or(&root);
        let pattern = format!("{}/{}", parent.display(), &self.cfg.include);

        glob(pattern.as_str())
            .expect("failed to read glob pattern") // TODO: validate the glob pattern in read()
            .filter_map(|entry| entry.ok())
    }

    pub fn read_ledgers(&self) -> impl Iterator<Item = (PathBuf, Result<String, io::Error>)> + '_ {
        self.included_paths().map(|path| {
            let src = fs::read_to_string(&path);
            (path, src)
        })
    }
}
