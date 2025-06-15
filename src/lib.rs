#![deny(deprecated)]

use std::{collections::VecDeque, fmt::Debug, str::Bytes};

use codespan_reporting::files::{self};

use crate::query::{__Source, Database, QueryValue};

pub mod diag;
pub mod lexer;
pub mod parser;
pub mod query;
pub mod span;

struct Cursor<'a> {
    len_remaining: usize,
    source: Bytes<'a>,
    peeks: VecDeque<Option<char>>,
}

impl<'a> Cursor<'a> {
    fn new(source: &'a str) -> Self {
        let bytes = source.bytes();
        Cursor {
            len_remaining: source.len(),
            source: bytes,
            peeks: VecDeque::new(),
        }
    }

    fn eof(&self) -> bool {
        self.len_remaining == 0
    }

    fn peek(&mut self) -> Option<char> {
        if let Some(char) = self.peeks.front() {
            *char
        } else {
            self.peeks.push_back(self.source.next().map(|x| x as char));
            self.peeks[0] // Should be able to be used
        }
    }
}

impl<'a> Iterator for Cursor<'a> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        if self.len_remaining == 0 {
            return None;
        }

        self.len_remaining -= 1;
        if self.peeks.is_empty() {
            self.source.next().map(|x| x as char)
        } else {
            self.peeks.pop_front().unwrap() // Just confirmed that self.peeks isn't empty
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Source(u32);

impl Source {
    pub(crate) fn name<'db>(&self, db: &'db Database) -> &'db str {
        match db.get(self.0).unwrap() {
            QueryValue::Source(val) => &val.name,
            _ => panic!(),
        }
    }

    pub(crate) fn contents<'db>(&self, db: &'db Database) -> &'db str {
        match db.get(self.0).unwrap() {
            QueryValue::Source(val) => &val.contents,
            _ => panic!(),
        }
    }

    pub(crate) fn line_starts<'db>(&self, db: &'db Database) -> &'db Vec<usize> {
        match db.get(self.0).unwrap() {
            QueryValue::Source(val) => &val.line_starts,
            _ => panic!(),
        }
    }

    pub fn new(db: &mut Database, source: impl ToString, path: impl ToString) -> Self {
        let line_starts = files::line_starts(source.to_string().as_str()).collect();
        let __source = __Source {
            name: path.to_string(),
            contents: source.to_string(),
            line_starts,
        };
        Source(db.create(QueryValue::Source(__source)))
    }
}

impl Debug for Source {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<Source>")
    }
}

#[test]
fn peek_test() {
    let source = "Hello, this is a test!";
    let mut cursor = Cursor::new(source);
    assert_eq!(cursor.peek(), Some('H'));
    assert_eq!(cursor.peek(), Some('H'));
    assert_eq!(cursor.next(), Some('H'));
    assert_eq!(cursor.peek(), Some('e'));
    assert_eq!(cursor.next(), Some('e'));
    assert_eq!(cursor.next(), Some('l'));
    assert_eq!(cursor.next(), Some('l'));
    assert_eq!(cursor.collect::<String>(), "o, this is a test!".to_owned());
}

#[test]
fn eof_test() {
    let source = "Test";
    let mut cursor = Cursor::new(source);
    assert!(!cursor.eof());
    cursor.next(); // T
    assert!(!cursor.eof());
    cursor.next(); // e
    assert!(!cursor.eof());
    cursor.next(); // s
    assert!(!cursor.eof());
    cursor.next(); // t
    assert!(cursor.eof());
    assert!(cursor.next().is_none()); // None
}

#[test]
fn hash_test() {
    use std::hash::{DefaultHasher, Hash, Hasher};

    let string = String::from("Test string");
    let mut hasher = DefaultHasher::new();
    string.hash(&mut hasher);
    let string_hash = hasher.finish();
    hasher = DefaultHasher::new();
    string.as_str().hash(&mut hasher);
    let str_hash = hasher.finish();
    assert_eq!(string_hash, str_hash);
}
