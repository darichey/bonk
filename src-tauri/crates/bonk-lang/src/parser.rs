use std::iter::Peekable;

use cstree::{build::GreenNodeBuilder, Syntax};

use crate::lexer::{Lexer, Token};

#[derive(Syntax, Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
enum SyntaxKind {
    /* Tokens */
    Date,
    Word,
    Account,
    Amount,
    /* Nodes */
    Posting,
    Transaction,
    Ledger,
}

type Bonk = SyntaxKind;

pub struct Parser<'input> {
    lexer: Peekable<Lexer<'input>>,
    builder: GreenNodeBuilder<'static, 'static, Bonk>,
}

impl<'input> Parser<'input> {
    pub fn new(input: &'input str) -> Self {
        Self {
            // we get `peekable` from implementing `Iterator` on `Lexer`
            lexer: Lexer::new(input).peekable(),
            builder: GreenNodeBuilder::new(),
        }
    }

    pub fn bump(&mut self) -> Option<Token> {
        self.lexer.next().map(|t| t.unwrap()) // TODO
    }

    pub fn parse(&mut self) -> Result<(), ()> {
        self.builder.start_node(SyntaxKind::Ledger);
        // self.parse_expr()?;
        self.builder.finish_node();
        Ok(())
    }
}
