use bonk_ast::{Ledger, SourceSpan};
use lsp_types::{Location, Position, Range, Url};

pub fn get_go_to_def_result(
    ledger: &Ledger,
    src: &str,
    uri: Url,
    pos: Position,
) -> Option<Location> {
    let account_name = find_account(ledger, src, pos)?;

    for acc in ledger.declare_accounts() {
        if let Some(acc) = acc.account() {
            if acc.value(src) == account_name {
                let span = acc.span();
                return Some(Location {
                    uri,
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
                });
            }
        }
    }

    None
}

fn find_account<'s>(ledger: &Ledger, src: &'s str, pos: Position) -> Option<&'s str> {
    // TODO: replace with a generic "find the most specific node covered by pos" algorithm
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

fn span_covers(span: SourceSpan, pos: Position) -> bool {
    let row = pos.line as usize;
    let col = pos.character as usize;

    row >= span.start_row && row <= span.end_row && col >= span.start_col && col <= span.end_col
}
