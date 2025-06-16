use peek_again::Peekable;

use crate::{lexer::Lexer, Source};

pub struct Parser<'src> {
    lexer: Peekable<Lexer<'src>>
}

#[derive(Debug)]
pub enum Declaration {

}

impl<'src> Parser<'src> {
    pub fn new(src: &'src Source<'src>) -> Self {
        Self::new_from_lexer(Lexer::new(src))
    }

    pub fn new_from_lexer(lexer: Lexer<'src>) -> Self {
        Parser {
            lexer: Peekable::new(lexer),
        }
    }

    pub fn parse(&mut self) -> Vec<Declaration> {
        todo!()
    }
}