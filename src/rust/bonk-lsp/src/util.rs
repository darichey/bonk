use bonk_ast::SourceSpan;
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
