use bonk_ast::Ledger;
use bonk_check::{check_balance, check_syntax, BalanceErrors, SyntaxErrors};
use lsp_types::{Diagnostic, DiagnosticSeverity, Position, Range};

pub fn get_diagnostics(ledger: &Ledger, src: &str) -> Vec<Diagnostic> {
    eprintln!("{}", src);
    eprintln!("{:?}", ledger);
    match check_syntax(ledger, src) {
        Ok(ledger) => match check_balance(ledger) {
            Ok(_) => vec![],
            Err(BalanceErrors(errs)) => errs
                .into_iter()
                .map(|span| Diagnostic {
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
                })
                .collect(),
        },
        Err(SyntaxErrors(errs)) => errs
            .into_iter()
            .map(|span| Diagnostic {
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
            })
            .collect(),
    }
}
