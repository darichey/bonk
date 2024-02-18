pub mod ast;
mod util;

use std::{collections::HashMap, path::PathBuf};

use bonk_workspace::Workspace;
use util::{byte_offset_to_position, edit_ledger, position_to_byte_offset};

pub struct Parser {
    ts_parser: tree_sitter::Parser,
}

impl Parser {
    pub fn new() -> Self {
        let mut ts_parser = tree_sitter::Parser::new();
        ts_parser
            .set_language(tree_sitter_bonk::language())
            .unwrap();
        Self { ts_parser }
    }

    pub fn parse(&mut self, src: &str, old_ledger: Option<&ast::Ledger>) -> ast::Ledger {
        let tree = self
            .ts_parser
            .parse(src, old_ledger.map(|old_ledger| &old_ledger.0))
            .unwrap();
        ast::Ledger::new(tree)
    }
}

impl Default for Parser {
    fn default() -> Self {
        Self::new()
    }
}

pub struct ParsedLedger {
    pub src: String,
    pub ledger: ast::Ledger,
    pub parser: Parser,
}

pub struct ParsedWorkspace {
    pub ledgers: HashMap<PathBuf, ParsedLedger>,
}

impl ParsedWorkspace {
    pub fn new() -> Self {
        Self {
            ledgers: HashMap::new(),
        }
    }

    pub fn parse_new(&mut self, path: PathBuf, src: String) {
        if self.ledgers.contains_key(&path) {
            panic!("TODO");
        }

        let mut parser = Parser::new();
        let ledger = parser.parse(&src, None);
        self.ledgers.insert(
            path,
            ParsedLedger {
                src,
                ledger,
                parser,
            },
        );
    }

    pub fn parse_replaced(&mut self, path: PathBuf, new_src: String) {
        let ParsedLedger {
            src,
            ledger,
            parser,
        } = self.ledgers.get_mut(&path).expect("TODO");

        let old_src = src.clone();

        *src = new_src;
        let (end_line, end_col) = byte_offset_to_position(src, src.len());
        edit_ledger(ledger, &old_src, src, 0, 0, end_line, end_col, src.len());

        *ledger = parser.parse(src, Some(ledger));
    }

    pub fn parse_changed(
        &mut self,
        path: PathBuf,
        new_src: String,
        start_line: usize,
        start_col: usize,
        end_line: usize,
        end_col: usize,
    ) {
        let ParsedLedger {
            src,
            ledger,
            parser,
        } = self.ledgers.get_mut(&path).expect("TODO");

        let old_src = src.clone();

        let start_byte = position_to_byte_offset(src, start_line, start_col);
        let end_byte = position_to_byte_offset(src, end_line, end_col);

        src.replace_range(start_byte..end_byte, &new_src);
        edit_ledger(
            ledger,
            &old_src,
            src,
            start_line,
            start_col,
            end_line,
            end_col,
            new_src.len(),
        );

        *ledger = parser.parse(src, Some(ledger));
    }

    pub fn remove(&mut self, path: &PathBuf) -> Option<ParsedLedger> {
        self.ledgers.remove(path)
    }
}

impl Default for ParsedWorkspace {
    fn default() -> Self {
        Self::new()
    }
}

pub trait WorkspaceExt {
    fn parse(&self) -> Result<ParsedWorkspace, ()>;
}

impl WorkspaceExt for Workspace {
    fn parse(&self) -> Result<ParsedWorkspace, ()> {
        todo!()
    }
}
