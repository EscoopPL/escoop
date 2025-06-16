#![deny(deprecated)]

use std::{borrow::Cow, fmt::Debug, ops::Range, path::PathBuf};

use codespan_reporting::files::{self, Error, Files};

pub mod diag;
pub mod lexer;
pub mod parser;
pub mod span;

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Source<'src> {
    path: PathBuf,
    source: &'src str,
    line_starts: Vec<usize>,
}

impl<'src> Source<'src> {
    pub(crate) fn path(&self) -> &PathBuf {
        &self.path
    }

    pub fn new(source: &'src str, path: impl Into<PathBuf>) -> Self {
        let line_starts = files::line_starts(source).collect();
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
            Ordering::Equal => Ok(self.source.len()),
            Ordering::Greater => Err(files::Error::LineTooLarge {
                given: line_index,
                max: self.line_starts.len() - 1,
            }),
        }
    }
}

impl<'a> Files<'a> for Source<'a> {
    type FileId = ();

    type Name = Cow<'a, str>;

    type Source = &'a str;

    fn name(&'a self, _: Self::FileId) -> Result<Self::Name, codespan_reporting::files::Error> {
        Ok(self.path.as_os_str().to_string_lossy())
    }

    fn source(&'a self, _: Self::FileId) -> Result<Self::Source, codespan_reporting::files::Error> {
        Ok(self.source)
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

impl<'src> Debug for Source<'src> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<Source>")
    }
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
