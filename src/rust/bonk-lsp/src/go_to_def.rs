use std::path::PathBuf;

use bonk_parse::ast::{Ledger, SourceSpan};
use lsp_types::{Location, Position, Url};

use crate::{
    state::State,
    util::{find_account, SourceSpanExt},
};

pub fn get_go_to_def_result(
    state: &State,
    ledger: &Ledger,
    src: &str,
    pos: Position,
) -> Option<Location> {
    let goto_account_name = find_account(ledger, src, pos)?;

    // TODO: there should probably be a standard way to get all of the declared accounts. We also do this in the account_ref check. But there's not a clear way to share between here and there
    let declared_accounts = state
        .workspace
        .ledgers
        .iter()
        .flat_map(|(path, ledger)| {
            ledger
                .ledger
                .declare_accounts()
                .into_iter()
                .filter_map(|decl_acc| {
                    Some((
                        decl_acc.account()?.value(&ledger.src),
                        path.clone(),
                        decl_acc.span(),
                    ))
                })
        })
        .collect::<Vec<(&str, PathBuf, SourceSpan)>>();

    for (acc_name, path, span) in declared_accounts {
        if acc_name == goto_account_name {
            return Some(Location {
                uri: Url::from_file_path(path).unwrap(), // FIXME
                range: span.into_lsp_range(),
            });
        }
    }

    None
}
