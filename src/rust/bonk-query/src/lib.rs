use std::path::PathBuf;

use anyhow::anyhow;
use bonk_check::WorkspaceExt as _;
use bonk_db::{Db, QueryOutput};
use bonk_parse::WorkspaceExt as _;
use bonk_workspace::Workspace;
use prettytable::{format, Table};

pub mod cli;

pub fn exec_query(cfg: PathBuf, query: &str) -> anyhow::Result<()> {
    // TODO: can we dedupe this with bonk_check::cli::run ?
    let workspace = Workspace::from_cfg(cfg).map_err(|err| anyhow!(err))?;
    let workspace = workspace.parse()?;
    let workspace = workspace.check().map_err(|err| {
        // TODO: pretty print errors with miette or something like it
        anyhow!(err
            .into_iter()
            .map(|err| format!("{:?}", err))
            .collect::<Vec<_>>()
            .join("\n"))
    })?;

    let db = Db::new(&workspace, ":memory:")?;
    let QueryOutput { column_names, data } = db.query(query)?;

    // TOOD: dedupe with bonk-test
    let mut table = Table::new();
    table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);
    table.set_titles(column_names.into());
    for row in data {
        table.add_row(row.into());
    }
    let output = table.to_string();

    println!("{}", output);

    Ok(())
}
