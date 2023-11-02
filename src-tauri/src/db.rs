use std::{
    fmt::Display,
    fs,
    path::{Path, PathBuf},
};

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use sqlite::{Connection, State, Value};

use anyhow::{Context, Result};

use crate::{dashboard::Dashboard, import_transactions, metadata::Metadata};

pub struct Db {
    con: Connection,
    // TODO: hoist non-sql stuff out into new parent
    pub metadatas: Vec<Metadata>,
    pub dashboards: Vec<Dashboard>,
}

pub struct Statement<'a> {
    stmt: sqlite::Statement<'a>,
}

impl Statement<'_> {
    pub fn column_names(&self) -> Vec<String> {
        self.stmt.column_names().to_vec()
    }
}

impl Db {
    pub fn new(path_to_schema: impl AsRef<Path>, path_to_data: impl AsRef<Path>) -> Result<Db> {
        let mut db = Db {
            con: sqlite::open(":memory:").context("Failed to open sqlite connection")?,
            metadatas: Vec::new(),
            dashboards: Vec::new(),
        };

        db.import_transactions(path_to_schema, &path_to_data)
            .context("Failed to import transactions")?;
        db.import_metadata(&path_to_data)
            .context("Failed to import metadata")?;
        db.import_dashboards(&path_to_data)
            .context("Failed to import dashboards")?;

        Ok(db)
    }

    fn import_transactions(
        &self,
        path_to_schema: impl AsRef<Path>,
        path_to_data: impl AsRef<Path>,
    ) -> Result<()> {
        // create transactions table
        self.con.execute(fs::read_to_string(path_to_schema)?)?;

        // insert transactions
        self.con.execute("BEGIN TRANSACTION")?;

        for Transaction {
            id,
            date,
            description,
            amount,
            account,
        } in import_transactions::import_all(path_to_data)?
        {
            let mut stmt = self.con.prepare(
                "INSERT INTO transactions VAlUES (:id, :date, :description, :amount, :account)",
            )?;

            stmt.bind::<&[(_, Value)]>(
                &[
                    (":id", id.into()),
                    (":date", date.to_string().into()),
                    (":description", description.into()),
                    (":amount", amount.cents.into()),
                    (":account", account.into()),
                ][..],
            )?;

            while let Ok(State::Row) = stmt.next() {}
        }

        self.con.execute("COMMIT")?;

        Ok(())
    }

    fn import_metadata(&mut self, path_to_data: impl AsRef<Path>) -> Result<()> {
        let metadata_glob = format!("{}/metadata/**/*.csv", path_to_data.as_ref().display());
        let metadata_paths = glob::glob(&metadata_glob)?;

        for metadata_path in metadata_paths {
            let metadata_path = metadata_path?;

            let path_without_extension = metadata_path.with_extension("");

            let metadata_name = path_without_extension
                .file_name()
                .with_context(|| format!("{} has no name", metadata_path.display()))?
                .to_str()
                .with_context(|| {
                    format!("Couldn't convert path to str: {}", metadata_path.display())
                })?;

            self.metadatas.push(Metadata {
                name: metadata_name.to_string(),
            });

            self.con.execute("BEGIN TRANSACTION")?;

            let mut csv_reader = csv::Reader::from_path(metadata_path)?;

            let header = csv_reader.headers()?;
            let cols = header.iter().skip(1).collect::<Vec<_>>().join(",");

            self.con.execute(format!(
                "CREATE TABLE {}(id INTEGER PRIMARY KEY,{}, FOREIGN KEY(id) REFERENCES transactions(id))",
                metadata_name, cols
            ))?;

            for record in csv_reader.records() {
                let values = record?
                    .iter()
                    .map(|field| format!("\"{field}\""))
                    .collect::<Vec<_>>()
                    .join(",");

                self.con
                    .execute(format!("INSERT INTO {metadata_name} VALUES({values})"))?;
            }

            self.con.execute("COMMIT")?;
        }

        Ok(())
    }

    fn import_dashboards(&mut self, path_to_data: impl AsRef<Path>) -> Result<()> {
        let dashboard_glob = format!("{}/dashboards/**/*.toml", path_to_data.as_ref().display());
        let dashboard_paths = glob::glob(&dashboard_glob)?;

        for dashboard_path in dashboard_paths {
            let dashboard_path = dashboard_path?;
            let contents = fs::read_to_string(dashboard_path)?;
            let dashboard: Dashboard = toml::from_str(&contents)?;
            self.dashboards.push(dashboard);
        }

        Ok(())
    }

    pub fn get_transactions(&self, stmt: Statement<'_>) -> Result<Vec<Transaction>> {
        self.query(stmt)
            .map(|row| {
                let row = row?;
                Ok(Transaction {
                    id: row.try_read::<i64, _>("id")?,
                    date: NaiveDate::parse_from_str(row.try_read::<&str, _>("date")?, "%Y-%m-%d")?,
                    description: row.try_read::<&str, _>("description")?.to_string(),
                    amount: DollarAmount {
                        cents: row.try_read::<i64, _>("amount")?,
                    },
                    account: row.try_read::<&str, _>("account")?.to_string(),
                })
            })
            .collect()
    }

    pub fn get_metadata(&self, name: &str) -> Result<(Vec<String>, Vec<Vec<Value>>)> {
        let query = format!("SELECT * FROM {name}");
        let stmt = self.prepare(&query)?;

        let col_names = stmt.column_names();

        let data = self
            .query(stmt)
            .map(|row| {
                let values: Vec<Value> = row?.into();
                Ok(values)
            })
            .collect::<Result<Vec<Vec<Value>>>>()?;

        Ok((col_names, data))
    }

    pub fn prepare(&self, query: &str) -> Result<Statement<'_>> {
        Ok(Statement {
            stmt: self.con.prepare(query)?,
        })
    }

    // pub fn prepare_bind<T: sqlite::Bindable>(
    //     &self,
    //     query: &str,
    //     value: T,
    // ) -> Result<Statement<'_>> {
    //     let mut stmt = self.prepare(query)?;
    //     stmt.stmt.bind(value)?;
    //     Ok(stmt)
    // }

    pub fn query<'a>(
        &'a self,
        stmt: Statement<'a>,
    ) -> impl Iterator<Item = Result<sqlite::Row>> + 'a {
        stmt.stmt.into_iter().map(|row| Ok(row?))
    }
}

#[derive(Serialize, Deserialize)]
pub struct Transaction {
    pub id: i64,
    pub date: NaiveDate,
    pub description: String,
    pub amount: DollarAmount,
    pub account: String,
}

#[derive(Serialize, Deserialize, Hash)]
pub struct DollarAmount {
    pub cents: i64,
}

impl DollarAmount {
    // TODO: replace with TryFrom<&str>, need to enumerate error cases which is good anyways
    pub fn parse(s: &str) -> Result<DollarAmount> {
        let negative = s.starts_with('-');
        let (dollars, cents) = s.split_once('.').unwrap_or((s, "0"));
        let dollars: i64 = str::parse(&dollars.replace('-', ""))?;
        let cents: i64 = str::parse(cents)?;
        let cents = dollars * 100 + cents;
        let cents = if negative { -cents } else { cents };
        Ok(DollarAmount { cents })
    }
}

impl Display for DollarAmount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.cents)
    }
}
