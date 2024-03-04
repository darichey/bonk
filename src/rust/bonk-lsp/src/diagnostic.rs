use std::{
    collections::{hash_map::Entry, HashMap},
    path::PathBuf,
};

use bonk_check::{CheckErrorCode, WorkspaceExt};
use bonk_parse::ParsedWorkspace;
use lsp_types::{Diagnostic, DiagnosticSeverity, Url};

use crate::{state::State, util::SourceSpanExt};

pub fn get_doc_diagnostics(state: &State, uri: &Url) -> Vec<Diagnostic> {
    let path = uri.to_file_path().unwrap();
    let mut all_diagnostics = get_diagnostics(&state.workspace);

    all_diagnostics.remove(&path).unwrap_or_default()
}

pub fn get_diagnostics(workspace: &ParsedWorkspace) -> HashMap<PathBuf, Vec<Diagnostic>> {
    match workspace.check() {
        Ok(_) => HashMap::new(),
        Err(errs) => {
            let mut map: HashMap<PathBuf, Vec<Diagnostic>> = HashMap::new();

            for err in errs {
                let path = err.source.path.unwrap();
                let diagnostic = Diagnostic {
                    range: err.source.span.into_lsp_range(),
                    severity: Some(DiagnosticSeverity::ERROR),
                    code: None,
                    code_description: None,
                    source: Some("bonk".to_string()),
                    message: match err.code {
                        CheckErrorCode::MultipleInfers => {
                            "transaction has multiple inferred amounts".to_string()
                        }
                        CheckErrorCode::NoBalance => "transaction doesn't balance".to_string(),
                        CheckErrorCode::SyntaxError => "syntax error".to_string(),
                        CheckErrorCode::UnknownAccount => "unknown account".to_string(),
                    },
                    related_information: None,
                    tags: None,
                    data: None,
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
