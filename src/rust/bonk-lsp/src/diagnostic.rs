use bonk_ast::{Ledger, SourceSpan};
use bonk_check::{check_syntax, SyntaxErrors};
use lsp_types::{Diagnostic, DiagnosticSeverity, Position, Range};

pub fn get_diagnostics(ledger: &Ledger, src: &str) -> Vec<Diagnostic> {
    match check_syntax(ledger, src) {
        Ok(_) => vec![],
        Err(SyntaxErrors(errs)) => errs
            .into_iter()
            .map(|SourceSpan(range)| Diagnostic {
                range: Range {
                    start: Position {
                        line: range.start_point.row as u32,
                        character: range.start_point.column as u32,
                    },
                    end: Position {
                        line: range.end_point.row as u32,
                        character: range.end_point.column as u32,
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
