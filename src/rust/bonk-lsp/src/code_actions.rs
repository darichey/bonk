use bonk_parse::ast::Ledger;
use lsp_types::{CodeAction, Range};

use crate::state::State;

pub fn get_code_actions(
    _state: &State,
    _ledger: &Ledger,
    _src: &str,
    _pos: Range,
) -> Vec<CodeAction> {
    vec![]
}
