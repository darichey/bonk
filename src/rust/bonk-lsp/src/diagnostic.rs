use std::{
    collections::{hash_map::Entry, HashMap},
    path::{Path, PathBuf},
};

use bonk_check::{
    AccountRefError, BalanceError, CheckError, CheckUnit, ImportError, SyntaxError, WorkspaceExt,
};
use bonk_parse::{ast::Ledger, ParsedWorkspace};
use lsp_types::{Diagnostic, DiagnosticSeverity, Url};

use crate::{state::State, util::SourceSpanExt};

pub fn get_doc_diagnostics(state: &State, uri: &Url) -> Vec<Diagnostic> {
    let path = PathBuf::from(uri.as_ref());
    let mut all_diagnostics = get_diagnostics(&state.workspace);

    all_diagnostics.remove(&path).unwrap_or_default()
}

pub fn get_diagnostics(workspace: &ParsedWorkspace) -> HashMap<PathBuf, Vec<Diagnostic>> {
    match workspace.check() {
        Ok(_) => HashMap::new(),
        Err(errs) => {
            let mut map: HashMap<PathBuf, Vec<Diagnostic>> = HashMap::new();

            for err in errs {
                let (path, diagnostic) = match err {
                    CheckError::AccountRefError(AccountRefError(source)) => (
                        source.path.unwrap(),
                        Diagnostic {
                            range: source.span.into_lsp_range(),
                            severity: Some(DiagnosticSeverity::ERROR),
                            code: None,
                            code_description: None,
                            source: Some("bonk".to_string()),
                            message: "can't find account".to_string(),
                            related_information: None,
                            tags: None,
                            data: None,
                        },
                    ),
                    CheckError::BalanceError(BalanceError(source)) => (
                        source.path.unwrap(),
                        Diagnostic {
                            range: source.span.into_lsp_range(),
                            severity: Some(DiagnosticSeverity::ERROR),
                            code: None,
                            code_description: None,
                            source: Some("bonk".to_string()),
                            message: "transaction doesn't balance".to_string(),
                            related_information: None,
                            tags: None,
                            data: None,
                        },
                    ),
                    CheckError::ImportError(ImportError(source)) => (
                        source.path.unwrap(),
                        Diagnostic {
                            range: source.span.into_lsp_range(),
                            severity: Some(DiagnosticSeverity::ERROR),
                            code: None,
                            code_description: None,
                            source: Some("bonk".to_string()),
                            message: "can't import".to_string(),
                            related_information: None,
                            tags: None,
                            data: None,
                        },
                    ),
                    CheckError::SyntaxError(SyntaxError(source)) => (
                        source.path.unwrap(),
                        Diagnostic {
                            range: source.span.into_lsp_range(),
                            severity: Some(DiagnosticSeverity::ERROR),
                            code: None,
                            code_description: None,
                            source: Some("bonk".to_string()),
                            message: "syntax error".to_string(),
                            related_information: None,
                            tags: None,
                            data: None,
                        },
                    ),
                };

                match map.entry(path) {
                    Entry::Occupied(mut e) => {
                        e.get_mut().push(diagnostic);
                    }
                    Entry::Vacant(e) => {
                        e.insert(vec![diagnostic]);
                    }
                }
            }

            map
        }
    }
}
