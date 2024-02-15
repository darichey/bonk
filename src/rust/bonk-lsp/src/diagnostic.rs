use std::path::Path;

use bonk_ast::Ledger;
use bonk_check::{AccountRefError, BalanceError, CheckError, CheckUnit, ImportError, SyntaxError};
use lsp_types::{Diagnostic, DiagnosticSeverity};

use crate::util::SourceSpanExt;

pub fn get_diagnostics(ledger: &Ledger, src: &str, path: &Path) -> Vec<Diagnostic> {
    eprintln!("{}", src);
    eprintln!("{:?}", ledger);

    let check_unit = CheckUnit::new(vec![(path.to_path_buf(), ledger)]);

    match check_unit.check(&CheckUnit::new(vec![(path.to_path_buf(), src)])) {
        Ok(_) => vec![],
        Err(errs) => errs
            .into_iter()
            .map(|err| match err {
                CheckError::AccountRefError(AccountRefError(source)) => Diagnostic {
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
                CheckError::BalanceError(BalanceError(source)) => Diagnostic {
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
                CheckError::ImportError(ImportError(source)) => Diagnostic {
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
                CheckError::SyntaxError(SyntaxError(span)) => Diagnostic {
                    range: span.into_lsp_range(),
                    severity: Some(DiagnosticSeverity::ERROR),
                    code: None,
                    code_description: None,
                    source: Some("bonk".to_string()),
                    message: "syntax error".to_string(),
                    related_information: None,
                    tags: None,
                    data: None,
                },
            })
            .collect(),
    }
}
