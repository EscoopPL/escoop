#![deny(deprecated)]

use std::{collections::VecDeque, path::PathBuf, str::Chars};

pub mod diag;
pub mod lexer;
pub mod parser;
pub mod span;

use ariadne::{Cache as AriadneCache, Source as AriadneSource};

struct Cursor<'a> {
    len_remaining: usize,
    source: Chars<'a>,
    peeks: VecDeque<Option<char>>,
}

impl<'a> Cursor<'a> {
    fn new(source: &'a str) -> Self {
        let chars = source.chars();
        Cursor {
            len_remaining: source.len(),
            source: chars,
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
            self.peeks.push_back(self.source.next());
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
            self.source.next()
        } else {
            self.peeks.pop_front().unwrap() // Just confirmed that self.peeks isn't empty
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Source<I: AsRef<str> = String>(PathBuf, AriadneSource<I>);

impl<I: AsRef<str>> Source<I> {
    pub(crate) fn path(&self) -> &PathBuf {
        &self.0
    }

    pub fn new(path: impl Into<PathBuf>, source: I) -> Self {
        Source(path.into(), source.into())
    }
}

impl<I: AsRef<str>> AriadneCache<()> for Source<I> {
    type Storage = I;

    fn fetch(&mut self, id: &()) -> Result<&AriadneSource<Self::Storage>, impl std::fmt::Debug> {
        self.1.fetch(id)
    }

    fn display<'a>(&self, id: &'a ()) -> Option<impl std::fmt::Display + 'a> {
        self.1.display(id)
    }
}

impl<I: AsRef<str>> AriadneCache<()> for &Source<I> {
    type Storage = I;

    fn fetch(&mut self, _: &()) -> Result<&AriadneSource<I>, impl std::fmt::Debug> {
        let res: Result<&AriadneSource<I>, ()> = Ok(&self.1);
        res
    }
    fn display<'a>(&self, id: &'a ()) -> Option<impl std::fmt::Display + 'a> {
        self.1.display(id)
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
