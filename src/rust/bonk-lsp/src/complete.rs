use bonk_parse::ast::Ledger;
use lsp_types::{CompletionItem, Position};

use crate::{state::State, util::find_account};

pub fn get_completion_results(
    state: &State,
    ledger: &Ledger,
    src: &str,
    pos: Position,
) -> Vec<CompletionItem> {
    // FIXME: there's a bug here where completion doesn't work if the user has typed "assets/" I think because then there's no account node but a syntax error. I'm ok with this for now
    let Some(_) = find_account(ledger, src, pos) else {
        return vec![];
    };

    // TODO: deduplicate with go_to_def
    state
        .workspace
        .ledgers
        .values()
        .flat_map(|ledger| {
            ledger
                .ledger
                .declare_accounts()
                .into_iter()
                .filter_map(|decl_acc| Some(decl_acc.account()?.value(&ledger.src)))
        })
        .map(|acc| CompletionItem {
            label: acc.to_string(),
            ..Default::default()
        })
        .collect()
}
