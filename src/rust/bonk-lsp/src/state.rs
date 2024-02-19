use bonk_parse::{ParsedLedger, ParsedWorkspace};
use lsp_types::{Range, TextDocumentContentChangeEvent, Url};

pub struct State {
    pub workspace: ParsedWorkspace,
}

impl State {
    pub fn new(workspace: ParsedWorkspace) -> Self {
        Self { workspace }
    }

    pub fn get_ledger(&self, uri: &Url) -> Option<&ParsedLedger> {
        let path = uri.to_file_path().unwrap();
        self.workspace.ledgers.get(&path)
    }

    pub fn on_open(&mut self, uri: Url, src: String) {
        let path = uri.to_file_path().unwrap();

        if self.workspace.ledgers.contains_key(&path) {
            self.workspace.parse_replaced(path, src)
        } else {
            self.workspace.parse_new(path, src)
        }
    }

    pub fn on_change(&mut self, uri: &Url, changes: Vec<TextDocumentContentChangeEvent>) {
        let path = uri.to_file_path().unwrap();

        for change in changes {
            if let Some(Range { start, end }) = change.range {
                let start_line = start.line as usize;
                let start_col = start.character as usize;
                let end_line = end.line as usize;
                let end_col = end.character as usize;

                self.workspace.parse_changed(
                    path.clone(),
                    change.text,
                    start_line,
                    start_col,
                    end_line,
                    end_col,
                )
            } else {
                self.workspace.parse_replaced(path.clone(), change.text)
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use bonk_parse::ParsedWorkspace;
    use lsp_types::{Position, Range, TextDocumentContentChangeEvent, Url};

    use super::State;

    fn assert_state_change(
        state: &mut State,
        uri: &Url,
        changes: Vec<TextDocumentContentChangeEvent>,
        new_src: &str,
    ) {
        state.on_change(uri, changes);

        let path = uri.to_file_path().unwrap();

        assert_eq!(state.workspace.ledgers.get(&path).unwrap().src, new_src);
    }

    #[test]
    fn test_on_change() {
        let uri = Url::from_file_path("/test.bonk").unwrap();
        let mut state = State::new(ParsedWorkspace::new());
        state.on_open(uri.clone(), "some\ntext\nin\nthe\ndocument".to_string());

        assert_state_change(
            &mut state,
            &uri,
            vec![TextDocumentContentChangeEvent {
                range: Some(Range {
                    start: Position {
                        line: 0,
                        character: 0,
                    },
                    end: Position {
                        line: 0,
                        character: 0,
                    },
                }),
                range_length: Some(0),
                text: "a".to_string(),
            }],
            "asome\ntext\nin\nthe\ndocument",
        );

        assert_state_change(
            &mut state,
            &uri,
            vec![TextDocumentContentChangeEvent {
                range: Some(Range {
                    start: Position {
                        line: 1,
                        character: 4,
                    },
                    end: Position {
                        line: 1,
                        character: 4,
                    },
                }),
                range_length: Some(0),
                text: "b".to_string(),
            }],
            "asome\ntextb\nin\nthe\ndocument",
        );

        assert_state_change(
            &mut state,
            &uri,
            vec![TextDocumentContentChangeEvent {
                range: Some(Range {
                    start: Position {
                        line: 2,
                        character: 1,
                    },
                    end: Position {
                        line: 2,
                        character: 1,
                    },
                }),
                range_length: Some(0),
                text: "c".to_string(),
            }],
            "asome\ntextb\nicn\nthe\ndocument",
        );

        assert_state_change(
            &mut state,
            &uri,
            vec![TextDocumentContentChangeEvent {
                range: Some(Range {
                    start: Position {
                        line: 3,
                        character: 0,
                    },
                    end: Position {
                        line: 3,
                        character: 3,
                    },
                }),
                range_length: Some(0),
                text: "d".to_string(),
            }],
            "asome\ntextb\nicn\nd\ndocument",
        );

        assert_state_change(
            &mut state,
            &uri,
            vec![TextDocumentContentChangeEvent {
                range: Some(Range {
                    start: Position {
                        line: 1,
                        character: 0,
                    },
                    end: Position {
                        line: 3,
                        character: 1,
                    },
                }),
                range_length: Some(11),
                text: "".to_string(),
            }],
            "asome\n\ndocument",
        );

        assert_state_change(
            &mut state,
            &uri,
            vec![
                TextDocumentContentChangeEvent {
                    range: Some(Range {
                        start: Position {
                            line: 2,
                            character: 1,
                        },
                        end: Position {
                            line: 2,
                            character: 2,
                        },
                    }),
                    range_length: Some(1),
                    text: "p".to_string(),
                },
                TextDocumentContentChangeEvent {
                    range: Some(Range {
                        start: Position {
                            line: 0,
                            character: 2,
                        },
                        end: Position {
                            line: 0,
                            character: 3,
                        },
                    }),
                    range_length: Some(1),
                    text: "p".to_string(),
                },
            ],
            "aspme\n\ndpcument",
        );

        assert_state_change(
            &mut state,
            &uri,
            vec![TextDocumentContentChangeEvent {
                range: Some(Range {
                    start: Position {
                        line: 2,
                        character: 8,
                    },
                    end: Position {
                        line: 2,
                        character: 8,
                    },
                }),
                range_length: Some(0),
                text: "\n".to_string(),
            }],
            "aspme\n\ndpcument\n",
        );

        assert_state_change(
            &mut state,
            &uri,
            vec![TextDocumentContentChangeEvent {
                range: Some(Range {
                    start: Position {
                        line: 0,
                        character: 0,
                    },
                    end: Position {
                        line: 3,
                        character: 0,
                    },
                }),
                range_length: Some(16),
                text: "foo".to_string(),
            }],
            "foo",
        );

        assert_state_change(
            &mut state,
            &uri,
            vec![TextDocumentContentChangeEvent {
                range: Some(Range {
                    start: Position {
                        line: 0,
                        character: 2,
                    },
                    end: Position {
                        line: 0,
                        character: 3,
                    },
                }),
                range_length: Some(1),
                text: "".to_string(),
            }],
            "fo",
        );
    }
}
