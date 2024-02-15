mod account_ref;
mod balance;
mod import;
mod syntax;

use itertools::Itertools;
use nonempty::{nonempty, NonEmpty};
use std::path::{Path, PathBuf};

pub use account_ref::AccountRefError;
pub use balance::BalanceError;
pub use import::ImportError;
pub use syntax::SyntaxError;

#[derive(Debug, PartialEq, Eq)]
pub enum CheckError {
    AccountRefError(AccountRefError),
    BalanceError(BalanceError),
    ImportError(ImportError),
    SyntaxError(SyntaxError),
}

pub struct CheckUnit<T>(NonEmpty<(PathBuf, T)>);

impl<T> CheckUnit<T> {
    pub fn one(path: &Path, ledger: T) -> Self {
        Self(nonempty![(path.to_path_buf(), ledger)])
    }

    pub fn ledgers(&self) -> impl Iterator<Item = &(PathBuf, T)> + '_ {
        self.0.iter()
    }

    pub fn get_ledger(&self, path: &Path) -> Option<&T> {
        self.ledgers()
            .find_map(|(p, l)| if p == path { Some(l) } else { None })
    }

    pub fn push_ledger(&mut self, path: &Path, ledger: T) {
        self.0.push((path.to_path_buf(), ledger))
    }
}

impl CheckUnit<&bonk_ast::Ledger> {
    fn check_syntax(
        self,
        srcs: &CheckUnit<&str>,
    ) -> Result<CheckUnit<bonk_ast_errorless::Ledger>, Vec<SyntaxError>> {
        let (ledgers, errors): (Vec<_>, Vec<Vec<SyntaxError>>) = self
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
            Ok(CheckUnit(
                NonEmpty::from_vec(ledgers).expect("errors was empty but so was ledgers"),
            ))
        }
    }

    pub fn check(
        self,
        srcs: &CheckUnit<&str>,
    ) -> Result<CheckUnit<bonk_ast_errorless::Ledger>, Vec<CheckError>> {
        let errorless = self.check_syntax(srcs).map_err(|errs| {
            errs.into_iter()
                .map(CheckError::SyntaxError)
                .collect::<Vec<_>>()
        })?;

        errorless.check_imports().map_err(|errs| {
            errs.into_iter()
                .map(CheckError::ImportError)
                .collect::<Vec<_>>()
        })?;

        errorless.check_account_refs().map_err(|errs| {
            errs.into_iter()
                .map(CheckError::AccountRefError)
                .collect::<Vec<_>>()
        })?;

        errorless.check_balance().map_err(|errs| {
            errs.into_iter()
                .map(CheckError::BalanceError)
                .collect::<Vec<_>>()
        })?;

        Ok(errorless)
    }
}

impl CheckUnit<bonk_ast_errorless::Ledger> {
    fn check_imports(&self) -> Result<(), Vec<ImportError>> {
        let mut errors = vec![];

        for (path, ledger) in self.ledgers() {
            match import::check_imports(path, ledger, self) {
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

    fn check_account_refs(&self) -> Result<(), Vec<AccountRefError>> {
        let mut errors = vec![];

        for (_, ledger) in self.ledgers() {
            match account_ref::check_account_refs(ledger) {
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

    fn check_balance(&self) -> Result<(), Vec<BalanceError>> {
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
