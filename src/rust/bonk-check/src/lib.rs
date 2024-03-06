// TODO: this whole crate really wants a nicer solution for applicative error handling

mod account_ref;
mod balance;
pub mod cli;
mod single_infer;
mod syntax;
mod util;

use bonk_parse::{ast::Source, ParsedLedger, ParsedWorkspace};
use itertools::Itertools;
use std::path::{Path, PathBuf};
use util::normalize_path;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum CheckErrorCode {
    MultipleInfers,
    NoBalance,
    SyntaxError,
    UnknownAccount,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct CheckError {
    pub code: CheckErrorCode,
    pub source: Source,
}

pub struct CheckUnit<T>(Vec<(PathBuf, T)>);

impl<T> CheckUnit<T> {
    pub fn new(ledgers: Vec<(PathBuf, T)>) -> Self {
        Self(ledgers)
    }

    pub fn ledgers(&self) -> impl Iterator<Item = &(PathBuf, T)> + '_ {
        self.0.iter()
    }

    pub fn get_ledger(&self, path: &Path) -> Option<&T> {
        self.ledgers().find_map(|(p, l)| {
            // TODO: normalization of p should be done elsewhere since we have to redo it every time this way
            if normalize_path(p) == normalize_path(path) {
                Some(l)
            } else {
                None
            }
        })
    }

    pub fn push_ledger(&mut self, path: &Path, ledger: T) {
        self.0.push((path.to_path_buf(), ledger))
    }
}

impl CheckUnit<&bonk_parse::ast::Ledger> {
    fn check_syntax(
        self,
        srcs: &CheckUnit<&str>,
    ) -> Result<CheckUnit<bonk_ast_errorless::Ledger>, Vec<CheckError>> {
        let (ledgers, errors): (Vec<_>, Vec<Vec<CheckError>>) = self
            .0
            .into_iter()
            .map(|(path, ledger)| {
                let src = srcs.get_ledger(&path).unwrap(); // FIXME
                let ledger = syntax::check_syntax(ledger, src, Some(&path))?;
                Ok((path, ledger))
            })
            .partition_result();

        if !errors.is_empty() {
            Err(errors.into_iter().flatten().collect())
        } else {
            Ok(CheckUnit(ledgers))
        }
    }

    pub fn check(
        self,
        srcs: &CheckUnit<&str>,
    ) -> Result<CheckUnit<bonk_ast_errorless::Ledger>, Vec<CheckError>> {
        let errorless = self.check_syntax(srcs)?;

        // FIXME: don't short-circuit, these errors should accumulate
        errorless.check_single_infer()?;
        errorless.check_account_refs()?;
        errorless.check_balance()?;

        Ok(errorless)
    }
}

// TODO: deduplicate
impl CheckUnit<bonk_ast_errorless::Ledger> {
    fn check_single_infer(&self) -> Result<(), Vec<CheckError>> {
        let mut errors = vec![];

        for (_, ledger) in self.ledgers() {
            match single_infer::check_single_infer(ledger) {
                Ok(_) => {}
                Err(errs) => errors.extend(errs),
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    fn check_account_refs(&self) -> Result<(), Vec<CheckError>> {
        let mut errors = vec![];

        for (_, ledger) in self.ledgers() {
            match account_ref::check_account_refs(ledger, self) {
                Ok(_) => {}
                Err(errs) => errors.extend(errs),
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    fn check_balance(&self) -> Result<(), Vec<CheckError>> {
        let mut errors = vec![];

        for (_, ledger) in self.ledgers() {
            match balance::check_balance(ledger) {
                Ok(_) => {}
                Err(errs) => errors.extend(errs),
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

pub type CheckedWorkspace = CheckUnit<bonk_ast_errorless::Ledger>;

pub trait WorkspaceExt {
    fn check(&self) -> Result<CheckedWorkspace, Vec<CheckError>>;
}

impl WorkspaceExt for ParsedWorkspace {
    fn check(&self) -> Result<CheckedWorkspace, Vec<CheckError>> {
        let mut srcs = vec![];
        let mut ledgers = vec![];

        for (path, ParsedLedger { src, ledger, .. }) in &self.ledgers {
            srcs.push((path.clone(), src.as_str()));
            ledgers.push((path.clone(), ledger));
        }

        let srcs = CheckUnit::new(srcs);
        let check_unit = CheckUnit::new(ledgers);

        check_unit.check(&srcs)
    }
}
