#![deny(missing_docs)]
//! Module for items related to lexical analysis of Escoop.

use std::{fmt::Display, iter::Peekable, str::Bytes};

use codespan_reporting::diagnostic::Label;

use crate::{Source, diag::Diag, span::Span};

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
    /// Opening parenthesis
    OpenParen,
    /// Closing parenthesis
    CloseParen,
    /// Opening square bracket
    OpenBracket,
    /// Closing square bracket
    CloseBracket,
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
    /// Colon (:)
    Colon,
}

/// Represents a value in the lexer that a token might have.
#[derive(Debug, Clone, PartialEq)]
pub enum LexerValue<'src> {
    /// String value
    String(&'src str),
    /// Numeric value
    Number(f32),
}

impl<'src> Display for LexerValue<'src> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LexerValue::String(val) => write!(f, "{val}"),
            LexerValue::Number(val) => write!(f, "{val}"),
        }
    }
}

/// Representation of a lexical token in Escoop.
#[derive(Debug, Clone, PartialEq)]
pub struct Token<'src> {
    token_type: TokenType,
    span: Span<'src>,
    value: Option<LexerValue<'src>>,
}

impl<'src> Token<'src> {
    /// Gets the span of a token.
    pub fn span(&self) -> Span<'src> {
        self.span
    }

    /// Gets the value of a token by borrowing it.
    pub fn value(&self) -> &Option<LexerValue> {
        &self.value
    }

    /// Gets the value of a token by moving it.
    pub fn move_value(self) -> Option<LexerValue<'src>> {
        self.value
    }

    /// Gets the type of a token.
    pub fn token_type(&self) -> TokenType {
        self.token_type
    }
}

impl<'src> Display for Token<'src> {
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
            TokenType::OpenBracket => write!(f, "["),
            TokenType::CloseBracket => write!(f, "]"),
            TokenType::Colon => write!(f, ":"),
        }
    }
}

macro_rules! make_token {
    ($self:ident, $ty:expr) => {{
        let span = $self.span;
        $self.update_span();
        Some(Token {
            token_type: $ty,
            span,
            value: None,
        })
    }};
    ($self:ident, $ty:expr, $val:expr) => {{
        let span = $self.span;
        $self.update_span();
        Some(Token {
            token_type: $ty,
            span,
            value: Some($val),
        })
    }};
}

/// Escoop lexical analyzer. Turns a source file into tokens.
pub struct Lexer<'src> {
    source: Peekable<Bytes<'src>>,
    span: Span<'src>,
    src: Source<'src>,
}

impl<'src> Lexer<'src> {
    /// Creates a new `Lexer`. `new_with_path` should be used instead of `new` if parsing a file,
    /// since `new_with_path` calls [`span::add_file`](crate::span::add_file) in addition to creating
    /// a `Lexer`.
    pub fn new(src: &'src Source<'src>) -> Self {
        Lexer {
            src: src.clone(),
            source: src.source.bytes().peekable(),
            span: Span::new(src),
        }
    }

    #[inline]
    fn next_char(&mut self) -> Option<u8> {
        let next = self.source.next();
        self.span.grow_front(1);
        next
    }

    #[inline]
    fn peek_char(&mut self) -> Option<u8> {
        self.source.peek().copied()
    }

    #[inline]
    fn update_span(&mut self) {
        self.span.update();
    }

    #[inline]
    fn skip_whitespace(&mut self) {
        let mut next = 0;
        while let Some(char) = self.peek_char() {
            if !char.is_ascii_whitespace() {
                break;
            }
            self.source.next();
            next += 1;
        }
        self.span.grow_front(next);
        self.span.update();
    }

    /// Checks if the `Lexer` is at the end of the source.
    #[inline]
    pub fn eof(&mut self) -> bool {
        self.source.peek().is_none()
    }
}

impl<'src> Iterator for Lexer<'src> {
    type Item = Token<'src>;

    fn next(&mut self) -> Option<Self::Item> {
        self.skip_whitespace();
        if self.eof() {
            return None;
        }

        match self.next_char().unwrap() {
            b'\'' => {
                let mut found = false;
                let mut next = 0;
                for c in self.source.by_ref() {
                    next += 1;
                    if c == b'\'' {
                        found = true;
                        break;
                    }
                    if c == b'\n' {
                        break;
                    }
                }

                self.span.grow_front(next);

                if !found {
                    let mut span = self.span;
                    span.shrink_front(1);
                    Diag::error(&self.src)
                        .with_message("unterminated string")
                        .with_label(Label::primary((), span))
                        .finish()
                        .emit();
                }
                let mut new_span = self.span;
                new_span.shrink_back(1); // Remove quotes
                new_span.shrink_front(1);
                let string = new_span.apply();
                make_token!(self, TokenType::StringLit, LexerValue::String(string))
            }
            b'(' => {
                make_token!(self, TokenType::OpenParen)
            }
            b')' => {
                make_token!(self, TokenType::CloseParen)
            }
            b',' => {
                make_token!(self, TokenType::Comma)
            }
            b'.' => {
                make_token!(self, TokenType::Dot)
            }
            b'=' => {
                make_token!(self, TokenType::Equals)
            }
            b'+' => {
                make_token!(self, TokenType::Plus)
            }
            b'-' => {
                make_token!(self, TokenType::Minus)
            }
            b'*' => {
                make_token!(self, TokenType::Star)
            }
            b'/' => {
                make_token!(self, TokenType::Slash)
            }
            b':' => {
                make_token!(self, TokenType::Colon)
            }
            b'[' => {
                make_token!(self, TokenType::OpenBracket)
            }
            b']' => {
                make_token!(self, TokenType::CloseBracket)
            }
            i if i.is_ascii_digit() => {
                let mut dot = false;
                while let Some(c) = self.peek_char() {
                    if !(c.is_ascii_digit() || (c == b'.' && !dot)) {
                        break;
                    }
                    if c == b'.' {
                        dot = true;
                    }
                    self.next_char();
                }
                let string = self.span.apply();
                make_token!(
                    self,
                    TokenType::NumberLit,
                    LexerValue::Number(string.parse().unwrap())
                )
            }
            i if i.is_ascii_alphabetic() => {
                while let Some(c) = self.peek_char() {
                    if c.is_ascii_alphanumeric() || c == b'-' || c == b'_' {
                        self.next_char();
                        continue;
                    }
                    break;
                }
                let string = self.span.apply();
                match string {
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
                Diag::error(&self.src)
                    .with_message(format!("unknown character `{}`", c as char))
                    .with_label(Label::primary((), self.span))
                    .finish()
                    .emit();
                None
            }
        }
    }
}

#[test]
fn whitespace_test() {
    let src = Source::new("Hello,  \nthis is a test.", "test.txt");
    let mut lexer = Lexer::new(&src);
    lexer.source.next();
    lexer.source.next();
    lexer.source.next();
    lexer.source.next();
    lexer.source.next();
    lexer.source.next(); // '  \nthis is a test.'
    lexer.skip_whitespace();
    assert_eq!(lexer.source.next(), Some(b't'));
    assert_eq!(lexer.source.next(), Some(b'h'));
}

#[test]
fn eof_test() {
    let src = Source::new("", "empty.txt");
    let mut lexer = Lexer::new(&src);
    assert!(lexer.next().is_none());
}

#[test]
fn span_test() {
    let file = "identifier test\n\nidentifier2 test3 test9 2 5 5553";
    let src = Source::new(file, "test.scp");
    let mut lexer = Lexer::new(&src);
    let token = lexer.next().unwrap();
    assert_eq!(token.span().apply(), "identifier");
    let token = lexer.next().unwrap();
    assert_eq!(token.span().apply(), "test");
    let token = lexer.next().unwrap();
    assert_eq!(token.span().apply(), "identifier2");
    let token = lexer.next().unwrap();
    assert_eq!(token.span().apply(), "test3");
    let token = lexer.next().unwrap();
    assert_eq!(token.span().apply(), "test9");
    let token = lexer.next().unwrap();
    assert_eq!(token.span().apply(), "2");
    let token = lexer.next().unwrap();
    assert_eq!(token.span().apply(), "5");
    let token = lexer.next().unwrap();
    assert_eq!(token.span().apply(), "5553");
}
