use bonk_parse::ast::SourceSpan;
use lsp_types::{Position, Range};

pub trait SourceSpanExt {
    fn into_lsp_range(self) -> Range;
}

impl SourceSpanExt for SourceSpan {
    fn into_lsp_range(self) -> Range {
        Range {
            start: Position {
                line: self.start_row as u32,
                character: self.start_col as u32,
            },
            end: Position {
                line: self.end_row as u32,
                character: self.end_col as u32,
            },
        }
    }
}

// TODO: replace with a generic "find the most specific node covered by pos" algorithm
pub fn find_account_in_postings<'s>(
    ledger: &bonk_parse::ast::Ledger,
    src: &'s str,
    pos: Position,
) -> Option<&'s str> {
    for transaction in ledger.transactions() {
        for posting in transaction.postings() {
            if let Some(acc) = posting.account() {
                if span_covers(acc.span(), pos) {
                    return Some(acc.value(src));
                }
            }
        }
    }

    None
}

// TODO: replace with a generic "find the most specific node covered by pos" algorithm
pub fn find_declared_account<'s>(
    ledger: &bonk_parse::ast::Ledger,
    src: &'s str,
    pos: Position,
) -> Option<&'s str> {
    for declare_account in ledger.declare_accounts() {
        if let Some(acc) = declare_account.account() {
            if span_covers(acc.span(), pos) {
                return Some(acc.value(src));
            }
        }
    }

    None
}

pub fn span_covers(span: SourceSpan, pos: Position) -> bool {
    let row = pos.line as usize;
    let col = pos.character as usize;

    row >= span.start_row && row <= span.end_row && col >= span.start_col && col <= span.end_col
}
