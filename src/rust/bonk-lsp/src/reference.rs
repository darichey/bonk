use bonk_parse::ast::Ledger;
use lsp_types::{Location, Position, Url};

use crate::{
    state::State,
    util::{find_declared_account, SourceSpanExt as _},
};

#[allow(unreachable_code)]
pub fn get_references(state: &State, ledger: &Ledger, src: &str, pos: Position) -> Vec<Location> {
    let Some(account) = find_declared_account(ledger, src, pos) else {
        return vec![];
    };

    state
        .workspace
        .ledgers
        .iter()
        .flat_map(|(path, ledger)| {
            ledger
                .ledger
                .transactions()
                .iter()
                .flat_map(|txn| txn.postings())
                .filter_map(|p| {
                    p.account().and_then(|acc| {
                        if acc.value(&ledger.src) == account {
                            Some(Location {
                                uri: Url::from_file_path(path).unwrap(), // FIXME
                                range: acc.span().into_lsp_range(),
                            })
                        } else {
                            None
                        }
                    })
                })
                .collect::<Vec<_>>()
        })
        .collect()
}
