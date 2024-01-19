use std::collections::HashMap;

use bonk_ast::{position_to_byte_offset, Ledger, Parser};
use lsp_types::{Position, Range, TextDocumentContentChangeEvent};

pub struct Document {
    pub src: String,
    pub ledger: Ledger,
    parser: Parser,
}

pub struct State {
    files: HashMap<String, Document>,
}

impl State {
    pub fn new() -> Self {
        Self {
            files: HashMap::new(),
        }
    }

    pub fn get_doc(&self, file: &str) -> Option<&Document> {
        self.files.get(file)
    }

    pub fn on_open(&mut self, file: String, src: String) {
        let mut parser = Parser::new();
        let ledger = parser.parse(&src, None);

        self.files.insert(
            file,
            Document {
                src,
                ledger,
                parser,
            },
        );
    }

    pub fn on_change(&mut self, file: &str, changes: Vec<TextDocumentContentChangeEvent>) {
        let Document {
            src,
            ledger,
            parser,
        } = self
            .files
            .get_mut(file)
            .expect("we don't know about the file");

        for change in changes {
            let old_src = src.clone();

            if let Some(Range { start, end }) = change.range {
                let start_line = start.line as usize;
                let start_col = start.character as usize;
                let end_line = end.line as usize;
                let end_col = end.character as usize;

                let start_byte = position_to_byte_offset(src, start_line, start_col);
                let end_byte = position_to_byte_offset(src, end_line, end_col);

                src.replace_range(start_byte..end_byte, &change.text);
                ledger.edit(
                    &old_src,
                    src,
                    start_line,
                    start_col,
                    end_line,
                    end_col,
                    change.text.len(),
                );
            } else {
                *src = change.text;
                // TODO
                ledger.edit(&old_src, src, 0, 0, todo!(), todo!(), change.text.len());
            }
        }

        *ledger = parser.parse(src, Some(ledger));
    }

    pub fn on_close(&mut self, file: &str) {
        self.files.remove(file);
    }
}

#[cfg(test)]
mod tests {
    use bonk_ast::position_to_byte_offset;
    use lsp_types::{Position, Range, TextDocumentContentChangeEvent};

    use super::State;

    #[test]
    fn test_on_change() {
        let mut state = State::new();
        state.on_open(
            "test".to_string(),
            "some\ntext\nin\nthe\ndocument".to_string(),
        );

        state.on_change(
            "test",
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
        );

        assert_eq!(
            state.files.get("test").unwrap().src,
            "asome\ntext\nin\nthe\ndocument"
        );

        state.on_change(
            "test",
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
        );

        assert_eq!(
            state.files.get("test").unwrap().src,
            "asome\ntextb\nin\nthe\ndocument"
        );

        state.on_change(
            "test",
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
        );

        assert_eq!(
            state.files.get("test").unwrap().src,
            "asome\ntextb\nicn\nthe\ndocument"
        );

        state.on_change(
            "test",
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
        );

        assert_eq!(
            state.files.get("test").unwrap().src,
            "asome\ntextb\nicn\nd\ndocument"
        );

        state.on_change(
            "test",
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
        );

        assert_eq!(state.files.get("test").unwrap().src, "asome\n\ndocument");

        state.on_change(
            "test",
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
        );

        assert_eq!(state.files.get("test").unwrap().src, "aspme\n\ndpcument");

        state.on_change(
            "test",
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
        );

        assert_eq!(state.files.get("test").unwrap().src, "aspme\n\ndpcument\n");

        state.on_change(
            "test",
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
        );

        assert_eq!(state.files.get("test").unwrap().src, "foo");

        state.on_change(
            "test",
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
        );

        assert_eq!(state.files.get("test").unwrap().src, "fo");
    }

    #[test]
    fn test_position_to_bye_offset() {
        let s = "foo\nbars\nbazzz";

        assert_eq!(position_to_byte_offset(s, 0, 0), 0);

        assert_eq!(position_to_byte_offset(s, 0, 2), 2);

        assert_eq!(position_to_byte_offset(s, 1, 0), 4);

        assert_eq!(position_to_byte_offset(s, 2, 3), 12);

        assert_eq!(position_to_byte_offset(s, 2, 5), 14)
    }
}
