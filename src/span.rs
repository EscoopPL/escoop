use std::{
    fmt::Display,
    hash::{DefaultHasher, Hash, Hasher},
};

use crate::diag::{get_code_pos, get_file_name};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span {
    pub(crate) start: usize,
    pub(crate) end: usize,
    pub(crate) file: u64,
}

impl Span {
    pub fn new(file: &str) -> Self {
        let mut hasher = DefaultHasher::new();
        file.hash(&mut hasher);
        Span {
            start: 0,
            end: 0,
            file: hasher.finish(),
        }
    }

    pub fn update(&mut self) {
        self.start = self.end;
    }

    /// Grows the span from the front
    pub fn grow(&mut self) {
        self.end += 1;
    }

    /// Shrinks the span from the back
    pub fn shrink(&mut self) {
        self.start += 1;
    }

    pub fn from_same_file(&self, other: &Self) -> bool {
        self.file == other.file
    }

    pub fn from_file(&self, file: &str) -> bool {
        let mut hasher = DefaultHasher::new();
        file.hash(&mut hasher);
        hasher.finish() == self.file
    }

    pub fn apply<'a>(&self, file: &'a str) -> &'a str {
        self.try_apply(file)
            .expect("tried to apply span to non-original file")
    }

    pub fn try_apply<'a>(&self, file: &'a str) -> Option<&'a str> {
        if self.from_file(file) {
            Some(self.apply_unchecked(file))
        } else {
            None
        }
    }

    pub fn apply_unchecked<'a>(&self, file: &'a str) -> &'a str {
        &file[self.start..self.end]
    }
}

impl Display for Span {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(name) = get_file_name(self.file) {
            write!(f, "{name}:")?;
        }
        if let Some(pos) = get_code_pos(self.file, self.start) {
            write!(f, "{}:{}", pos.0, pos.1)?;
        }
        Ok(())
    }
}
