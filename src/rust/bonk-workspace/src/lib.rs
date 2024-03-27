use bonk_cfg::BonkCfg;
use glob::glob;
use std::{
    cell::OnceCell,
    fs, io,
    path::{Path, PathBuf},
};

pub struct Workspace {
    pub cfg_path: PathBuf,
    pub cfg: BonkCfg,
    included_paths: OnceCell<Vec<PathBuf>>,
}

impl Workspace {
    pub fn from_cfg<T: AsRef<Path>>(cfg_path: T) -> Result<Self, String> {
        let cfg = BonkCfg::read(&cfg_path)?;
        Ok(Workspace {
            cfg_path: cfg_path.as_ref().to_path_buf(),
            cfg,
            included_paths: OnceCell::new(),
        })
    }

    pub fn included_paths(&self) -> &Vec<PathBuf> {
        self.included_paths.get_or_init(|| {
            let root = PathBuf::from("/");
            let parent = self.cfg_path.parent().unwrap_or(&root);
            let pattern = format!("{}/{}", parent.display(), &self.cfg.include);

            glob(pattern.as_str())
                .expect("failed to read glob pattern") // TODO: validate the glob pattern in read()
                .filter_map(|entry| entry.ok())
                .collect()
        })
    }

    pub fn includes_path(&self, path: &PathBuf) -> bool {
        // TODO: can I just check if the path matches the pattern?
        self.included_paths().contains(path)
    }

    pub fn read_ledgers(&self) -> impl Iterator<Item = (&PathBuf, Result<String, io::Error>)> + '_ {
        self.included_paths().iter().map(|path| {
            let src = fs::read_to_string(path);
            (path, src)
        })
    }
}
