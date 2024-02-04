use bonk_ast::Ledger;
use bonk_check::{check, BalanceError, CheckError, SyntaxError};
use lsp_types::{Diagnostic, DiagnosticSeverity, Position, Range};

pub fn get_diagnostics(ledger: &Ledger, src: &str) -> Vec<Diagnostic> {
    eprintln!("{}", src);
    eprintln!("{:?}", ledger);

    match check(ledger, src) {
        Ok(_) => vec![],
        Err(errs) => errs
            .into_iter()
            .map(|err| match err {
                CheckError::BalanceError(BalanceError(span)) => Diagnostic {
                    range: Range {
                        start: Position {
                            line: span.start_row as u32,
                            character: span.start_col as u32,
                        },
                        end: Position {
                            line: span.end_row as u32,
                            character: span.end_col as u32,
                        },
                    },
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
                    range: Range {
                        start: Position {
                            line: span.start_row as u32,
                            character: span.start_col as u32,
                        },
                        end: Position {
                            line: span.end_row as u32,
                            character: span.end_col as u32,
                        },
                    },
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
