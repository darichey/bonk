use std::{
    fs::{self, OpenOptions},
    io::Write,
    path::PathBuf,
};

use anyhow::{anyhow, Context};
use bonk_check::WorkspaceExt as _;
use bonk_db::{Db, QueryOutput};
use bonk_parse::WorkspaceExt as _;
use bonk_workspace::Workspace;
use prettytable::{format, Table};

pub mod cli;

pub fn run_tests(cfg: PathBuf, update: bool) -> anyhow::Result<()> {
    // TODO: can we dedupe this with bonk_check::cli::run ?
    let workspace = Workspace::from_cfg(cfg).map_err(|err| anyhow!(err))?;
    let parsed_workspace = workspace.parse()?;
    let checked_workspace = parsed_workspace.check().map_err(|err| {
        // TODO: pretty print errors with miette or something like it
        anyhow!(err
            .into_iter()
            .map(|err| format!("{:?}", err))
            .collect::<Vec<_>>()
            .join("\n"))
    })?;

    let db = Db::new(&checked_workspace, ":memory:")?;

    let snapshot_dir = workspace
        .cfg
        .snapshot_dir
        .context("must configure snapshot_dir to run tests")?;

    for test in workspace.cfg.tests {
        let QueryOutput { column_names, data } = db.query(&test.query)?;

        let mut table = Table::new();
        table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);
        table.set_titles(column_names.into());
        for row in data {
            table.add_row(row.into());
        }
        let expected = table.to_string();

        let snapshot_file_path = snapshot_dir.join(format!("{}.snapshot.txt", test.name)); // TODO: sanitize file name
        let actual = if snapshot_file_path.exists() {
            fs::read_to_string(&snapshot_file_path)?
        } else {
            "".to_string()
        };

        if expected != actual {
            if update {
                println!("\u{1f504} Updating: {}", test.name);

                let mut snapshot_file = OpenOptions::new()
                    .write(true)
                    .create(true)
                    .truncate(true)
                    .open(snapshot_file_path)?;

                write!(snapshot_file, "{}", expected)?;
            } else {
                // TODO: print diff
                // TODO: if any tests fail, exit with non-zero status
                println!("\u{274c} Fail: {}", test.name);
            }
        } else {
            println!("\u{2705} Pass: {}", test.name);
        }
    }

    Ok(())
}
