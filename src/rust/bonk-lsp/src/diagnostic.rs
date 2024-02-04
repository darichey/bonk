use bonk_ast::Ledger;
use bonk_check::{check, AccountRefError, BalanceError, CheckError, SyntaxError};
use lsp_types::{Diagnostic, DiagnosticSeverity};

use crate::util::SourceSpanExt;

pub fn get_diagnostics(ledger: &Ledger, src: &str) -> Vec<Diagnostic> {
    eprintln!("{}", src);
    eprintln!("{:?}", ledger);

    match check(ledger, src) {
        Ok(_) => vec![],
        Err(errs) => errs
            .into_iter()
            .map(|err| match err {
                CheckError::AccountRefError(AccountRefError(span)) => Diagnostic {
                    range: span.into_lsp_range(),
                    severity: Some(DiagnosticSeverity::ERROR),
                    code: None,
                    code_description: None,
                    source: Some("bonk".to_string()),
                    message: "can't find account".to_string(),
                    related_information: None,
                    tags: None,
                    data: None,
                },
                CheckError::BalanceError(BalanceError(span)) => Diagnostic {
                    range: span.into_lsp_range(),
                    severity: Some(DiagnosticSeverity::ERROR),
                    code: None,
                    code_description: None,
                    source: Some("bonk".to_string()),
                    message: "transaction doesn't balance".to_string(),
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
