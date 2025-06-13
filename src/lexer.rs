#![deny(missing_docs)]
//! Module for items related to lexical analysis of Escoop.

use std::{fmt::Display, io, path::Path};

use crate::{
    Cursor,
    diag::{DiagBuilder, DiagLevel},
    span::Span,
};

/// Enumeration of every possible type of [`Token`]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TokenType {
    /// Identifier of items (e.g. foo). The value of [`Token`] should be a `String`.
    Identifier,
    /// String literal (e.g. 'bar'). The value of [`Token`] should be a `String`.
    StringLit,
    /// Number literal (e.g. 5.4). The value of [`Token`] should be a `Number`.
    NumberLit,
    /// Identifier keyword
    IdentifierKey,
    /// Extern keyword
    Extern,
    /// Func keyword
    Func,
    /// Void keyword
    Void,
    /// Is keyword
    Is,
    /// End keyword
    End,
    /// Comma
    Comma,
    /// Closing parenthesis
    CloseParen,
    /// Opening parenthesis
    OpenParen,
    /// Dot/Period
    Dot,
    /// Equals sign
    Equals,
    /// Plus sign
    Plus,
    /// Minus sign
    Minus,
    /// Star (*)
    Star,
    /// Slash (/)
    Slash,
}

/// Represents a value in the lexer that a token might have.
#[derive(Debug, Clone, PartialEq)]
pub enum LexerValue {
    /// String value
    String(String),
    /// Numeric value
    Number(f32),
}

impl Display for LexerValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LexerValue::String(val) => write!(f, "{val}"),
            LexerValue::Number(val) => write!(f, "{val}"),
        }
    }
}

/// Representation of a lexical token in Escoop.
#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    token_type: TokenType,
    span: Span,
    value: Option<LexerValue>,
}

impl Token {
    /// Gets the span of a token.
    pub fn span(&self) -> Span {
        self.span
    }

    /// Gets the value of a token by borrowing it.
    pub fn value(&self) -> &Option<LexerValue> {
        &self.value
    }

    /// Gets the value of a token by moving it.
    pub fn move_value(self) -> Option<LexerValue> {
        self.value
    }

    /// Gets the type of a token.
    pub fn token_type(&self) -> TokenType {
        self.token_type
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.token_type {
            TokenType::Identifier => write!(f, "{}", self.value.as_ref().unwrap()),
            TokenType::StringLit => write!(f, "'{}'", self.value.as_ref().unwrap()),
            TokenType::NumberLit => write!(f, "{}", self.value.as_ref().unwrap()),
            TokenType::IdentifierKey => write!(f, "identifier"),
            TokenType::Extern => write!(f, "extern"),
            TokenType::Func => write!(f, "func"),
            TokenType::Void => write!(f, "void"),
            TokenType::Is => write!(f, "is"),
            TokenType::End => write!(f, "end"),
            TokenType::Comma => write!(f, ","),
            TokenType::CloseParen => write!(f, ")"),
            TokenType::OpenParen => write!(f, "("),
            TokenType::Dot => write!(f, "."),
            TokenType::Equals => write!(f, "="),
            TokenType::Plus => write!(f, "+"),
            TokenType::Minus => write!(f, "-"),
            TokenType::Star => write!(f, "/"),
            TokenType::Slash => write!(f, "*"),
        }
    }
}

macro_rules! make_token {
    ($self:ident, $ty:expr) => {{
        let span = $self.update_span();
        Some(Token {
            token_type: $ty,
            span,
            value: None,
        })
    }};
    ($self:ident, $ty:expr, $val:expr) => {{
        let span = $self.update_span();
        Some(Token {
            token_type: $ty,
            span,
            value: Some($val),
        })
    }};
}

/// Escoop lexical analyzer. Turns a source file into tokens.
pub struct Lexer<'a> {
    source: Cursor<'a>,
    span: Span,
}

impl<'a> Lexer<'a> {
    /// Creates a new `Lexer`.
    /// [`new_with_path`](Lexer::new_with_path) should be used instead,
    /// since `new` doesn't call [`span::add_file`](crate::span::add_file),
    /// which should be called if lexing a file.
    /// However, if not lexing a file, `new` may be used.
    pub fn new(source: &'a str) -> Self {
        Lexer {
            source: Cursor::new(source),
            span: Span::new(source),
        }
    }

    /// Creates a new `Lexer`. `new_with_path` should be used instead of `new` if parsing a file,
    /// since `new_with_path` calls [`span::add_file`](crate::span::add_file) in addition to creating
    /// a `Lexer`.
    pub fn new_with_path(source: &'a str, path: impl AsRef<Path>) -> Result<Self, io::Error> {
        Ok(Lexer {
            source: Cursor::new_with_path(source, path)?,
            span: Span::new(source),
        })
    }

    fn next_char(&mut self) -> Option<char> {
        let next = self.source.next();
        self.span.grow_front(1);
        next
    }

    fn peek_char(&mut self) -> Option<char> {
        self.source.peek()
    }

    fn update_span(&mut self) -> Span {
        let span = self.span;
        self.span.update();
        span
    }

    fn skip_whitespace(&mut self) {
        while let Some(char) = self.peek_char() {
            if !char.is_whitespace() {
                break;
            }
            self.next_char();
            self.update_span();
        }
    }

    /// Checks if the `Lexer` is at the end of the source.
    pub fn eof(&self) -> bool {
        self.source.eof()
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.skip_whitespace();
        if self.eof() {
            return None;
        }

        match self.next_char().unwrap() {
            '\'' => {
                let mut string = String::new();
                while let Some(c) = self.peek_char() {
                    if c == '\'' {
                        break;
                    }
                    string.push(c);
                    self.next_char();
                }
                self.next_char();
                make_token!(self, TokenType::StringLit, LexerValue::String(string))
            }
            '(' => {
                make_token!(self, TokenType::OpenParen)
            }
            ')' => {
                make_token!(self, TokenType::CloseParen)
            }
            ',' => {
                make_token!(self, TokenType::Comma)
            }
            '.' => {
                make_token!(self, TokenType::Dot)
            }
            '=' => {
                make_token!(self, TokenType::Equals)
            }
            '+' => {
                make_token!(self, TokenType::Plus)
            }
            '-' => {
                make_token!(self, TokenType::Minus)
            }
            '*' => {
                make_token!(self, TokenType::Star)
            }
            '/' => {
                make_token!(self, TokenType::Slash)
            }
            i if i.is_numeric() => {
                let mut string = i.to_string();
                let mut dot = false;
                while let Some(c) = self.peek_char() {
                    if !(c.is_numeric() || (c == '.' && !dot)) {
                        break;
                    }
                    if c == '.' {
                        dot = true;
                    }
                    string.push(c);
                    self.next_char();
                }
                make_token!(
                    self,
                    TokenType::NumberLit,
                    LexerValue::Number(string.parse().unwrap())
                )
            }
            i if i.is_alphabetic() => {
                let mut string = i.to_string();
                while let Some(c) = self.peek_char() {
                    if !c.is_alphanumeric() && c != '-' && c != '_' {
                        break;
                    }
                    string.push(c);
                    self.next_char();
                }
                match string.as_str() {
                    "identifier" => make_token!(self, TokenType::IdentifierKey),
                    "is" => make_token!(self, TokenType::Is),
                    "end" => make_token!(self, TokenType::End),
                    "extern" => make_token!(self, TokenType::Extern),
                    "func" => make_token!(self, TokenType::Func),
                    "void" => make_token!(self, TokenType::Void),
                    _ => make_token!(self, TokenType::Identifier, LexerValue::String(string)),
                }
            }
            c => {
                DiagBuilder::new(DiagLevel::Fatal)
                    .message(DiagLevel::Fatal, format!("unexpected character `{c}`"))
                    .set_span(self.span)
                    .finish()
                    .finish()
                    .emit();
                None
            }
        }
    }
}

#[test]
fn whitespace_test() {
    let mut lexer = Lexer::new("Hello,  \nthis is a test.");
    lexer.source.next();
    lexer.source.next();
    lexer.source.next();
    lexer.source.next();
    lexer.source.next();
    lexer.source.next(); // '  \nthis is a test.'
    lexer.skip_whitespace();
    assert_eq!(lexer.source.next(), Some('t'));
    assert_eq!(lexer.source.next(), Some('h'));
}

#[test]
fn eof_test() {
    let mut lexer = Lexer::new("");
    assert!(lexer.next().is_none());
}

#[test]
fn span_test() {
    let file = "identifier test\n\nidentifier2 test3 test9";
    let mut lexer = Lexer::new(file);
    let token = lexer.next().unwrap();
    assert_eq!(token.span().apply(file), "identifier");
    let token = lexer.next().unwrap();
    assert_eq!(token.span().apply(file), "test");
    let token = lexer.next().unwrap();
    assert_eq!(token.span().apply(file), "identifier2");
    let token = lexer.next().unwrap();
    assert_eq!(token.span().apply(file), "test3");
    let token = lexer.next().unwrap();
    assert_eq!(token.span().apply(file), "test9");
}

#[test]
#[should_panic]
fn span_origin_test() {
    let file = "Hello!";
    let file2 = "Goodbye!";
    let span2 = Span::new(file2);
    span2.apply(file);
}
