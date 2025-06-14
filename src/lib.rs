#![deny(deprecated)]

use std::{borrow::Cow, collections::VecDeque, fmt::Debug, ops::Range, path::PathBuf, str::Bytes};

use codespan_reporting::files::{self, Error, Files};

pub mod diag;
pub mod lexer;
pub mod parser;
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

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Source<I: AsRef<str> = String> {
    path: PathBuf,
    source: I,
    line_starts: Vec<usize>,
}

impl<I: AsRef<str>> Source<I> {
    pub(crate) fn path(&self) -> &PathBuf {
        &self.path
    }

    pub fn new(source: I, path: impl Into<PathBuf>) -> Self {
        let line_starts = files::line_starts(source.as_ref()).collect();
        Source {
            path: path.into(),
            source,
            line_starts,
        }
    }

    fn line_start(&self, line_index: usize) -> Result<usize, files::Error> {
            use core::cmp::Ordering;
    
            match line_index.cmp(&self.line_starts.len()) {
                Ordering::Less => Ok(self
                    .line_starts
                    .get(line_index)
                    .cloned()
                    .expect("failed despite previous check")),
                Ordering::Equal => Ok(self.source.as_ref().len()),
                Ordering::Greater => Err(files::Error::LineTooLarge {
                    given: line_index,
                    max: self.line_starts.len() - 1,
                }),
            }
    }
}

impl<'a, I: AsRef<str> + 'a> Files<'a> for Source<I> {
    type FileId = ();

    type Name = Cow<'a, str>;

    type Source = &'a I;

    fn name(&'a self, _: Self::FileId) -> Result<Self::Name, codespan_reporting::files::Error> {
        Ok(self.path.as_os_str().to_string_lossy())
    }

    fn source(&'a self, _: Self::FileId) -> Result<Self::Source, codespan_reporting::files::Error> {
        Ok(&self.source)
    }

    fn line_index(
        &'a self,
        _: Self::FileId,
        byte_index: usize,
    ) -> Result<usize, codespan_reporting::files::Error> {
        Ok(self
            .line_starts
            .binary_search(&byte_index)
            .unwrap_or_else(|next_line| next_line - 1))
    }

    fn line_range(&self, (): (), line_index: usize) -> Result<Range<usize>, Error> {
        let line_start = self.line_start(line_index)?;
        let next_line_start = self.line_start(line_index + 1)?;

        Ok(line_start..next_line_start)

    }
}

impl<I: AsRef<str>> Debug for Source<I> {
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
