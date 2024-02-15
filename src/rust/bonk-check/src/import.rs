use std::{
    path::{Path, PathBuf},
    str::FromStr,
};

use bonk_ast::Source;

use crate::CheckUnit;

#[derive(Debug, PartialEq, Eq)]
pub struct ImportError(pub Source);

// TODO: handle import cycles
pub fn check_imports(
    path: &Path,
    ledger: &bonk_ast_errorless::Ledger,
    check_unit: &CheckUnit<bonk_ast_errorless::Ledger>,
) -> Result<(), Vec<ImportError>> {
    let mut errors = vec![];

    for import in &ledger.imports {
        let import_path = PathBuf::from_str(&import.path).unwrap(); // TODO: this should just be a PathBuf to begin with

        if path == import_path {
            errors.push(ImportError(import.source.clone().expect(
                "foo ast passed to check_balance isn't annotated with source spans", // TODO: I really want to encode this in the types
            )))
        }

        if check_unit.get_ledger(&import_path).is_none() {
            errors.push(ImportError(import.source.clone().expect(
                "bar ast passed to check_balance isn't annotated with source spans", // TODO: I really want to encode this in the types
            )))
        }
    }

    if !errors.is_empty() {
        Err(errors)
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::{path::PathBuf, str::FromStr};

    use bonk_ast::{Source, SourceSpan};
    use bonk_ast_errorless::{Import, Ledger};

    use crate::{import::check_imports, CheckUnit};

    #[test]
    fn test_no_errors() {
        let path_a = "ledger_a.bonk";
        let ledger_a = Ledger {
            imports: vec![],
            declare_accounts: vec![],
            transactions: vec![],
            source: None,
        };

        let path_b = "ledger_b.bonk";
        let ledger_b = Ledger {
            imports: vec![Import {
                path: path_a.to_string(),
                source: None,
            }],
            declare_accounts: vec![],
            transactions: vec![],
            source: None,
        };

        let mut check_unit = CheckUnit::one(&PathBuf::from_str(path_a).unwrap(), ledger_a);
        check_unit.push_ledger(&PathBuf::from_str(path_b).unwrap(), ledger_b.clone());

        assert!(check_imports(&PathBuf::from_str(path_b).unwrap(), &ledger_b, &check_unit).is_ok());
    }

    #[test]
    fn test_error_self_import() {
        let path_a = "ledger_a.bonk";
        let ledger_a = Ledger {
            imports: vec![Import {
                path: path_a.to_string(),
                source: Some(Source {
                    path: Some(PathBuf::from("ledger_a.bonk")),
                    span: SourceSpan {
                        start_byte: 0,
                        end_byte: 1,
                        start_row: 2,
                        start_col: 3,
                        end_row: 4,
                        end_col: 5,
                    },
                }),
            }],
            declare_accounts: vec![],
            transactions: vec![],
            source: None,
        };

        let check_unit = CheckUnit::one(&PathBuf::from_str(path_a).unwrap(), ledger_a.clone());

        let checked_ledger =
            check_imports(&PathBuf::from_str(path_a).unwrap(), &ledger_a, &check_unit);

        insta::assert_debug_snapshot!(checked_ledger, @r###"
        Err(
            [
                ImportError(
                    Source {
                        path: Some(
                            "ledger_a.bonk",
                        ),
                        span: SourceSpan {
                            start_byte: 0,
                            end_byte: 1,
                            start_row: 2,
                            start_col: 3,
                            end_row: 4,
                            end_col: 5,
                        },
                    },
                ),
            ],
        )
        "###);
    }

    #[test]
    fn test_error_unknown_import() {
        let path_a = "ledger_a.bonk";
        let ledger_a = Ledger {
            imports: vec![Import {
                path: path_a.to_string(),
                source: Some(Source {
                    path: Some(PathBuf::from("ledger_b.bonk")),
                    span: SourceSpan {
                        start_byte: 0,
                        end_byte: 1,
                        start_row: 2,
                        start_col: 3,
                        end_row: 4,
                        end_col: 5,
                    },
                }),
            }],
            declare_accounts: vec![],
            transactions: vec![],
            source: None,
        };

        let check_unit = CheckUnit::one(&PathBuf::from_str(path_a).unwrap(), ledger_a.clone());

        let checked_ledger =
            check_imports(&PathBuf::from_str(path_a).unwrap(), &ledger_a, &check_unit);

        insta::assert_debug_snapshot!(checked_ledger, @r###"
        Err(
            [
                ImportError(
                    Source {
                        path: Some(
                            "ledger_b.bonk",
                        ),
                        span: SourceSpan {
                            start_byte: 0,
                            end_byte: 1,
                            start_row: 2,
                            start_col: 3,
                            end_row: 4,
                            end_col: 5,
                        },
                    },
                ),
            ],
        )
        "###);
    }
}
