#![deny(missing_docs)]
//! Module holding the `Span` type, which represents an area of a file.

use std::{fmt::Display, ops::Range};

/// The `Span` type represents an area of a file. `'src` represents the lifetime of the source.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span<'src> {
    pub(crate) start: u32,
    pub(crate) end: u32,
    pub(crate) src: &'src str,
}

impl<'src> Span<'src> {
    /// Creates a new `Span` from a source. This span will start and end at the 0th character, making it have a length of zero.
    pub fn new(src: &'src str) -> Self {
        Self::new_from(src, 0, 0)
    }

    /// Creates a new `Span` from a file and a pair of start and end indexes. These indexes are indexes into the file by characters.
    pub fn new_from(src: &'src str, start: u32, end: u32) -> Self {
        Span { start, end, src }
    }

    pub(crate) fn update(&mut self) {
        self.start = self.end;
    }

    /// Grows the span from the front. This moves the end value up by `amount`.
    pub fn grow_front(&mut self, amount: u32) {
        self.end += amount;
    }

    /// Grows the span from the back. This moves the start value back by `amount`.
    pub fn grow_back(&mut self, amount: u32) {
        self.start -= amount;
    }

    /// Shrinks the span from the back. This moves the start value up by `amount`.
    ///
    /// # Panics
    /// This method will panic if the size of the `Span` is less than `amount`, since a `Span`'s size can't be negative.
    pub fn shrink_back(&mut self, amount: u32) {
        if self.len() < amount {
            panic!("cannot create negative-size span");
        }
        self.start += amount;
    }

    /// Shrinks the span from the front. This moves the end value back by `amount`.
    ///
    /// # Panics
    /// This method will panic if the size of the `Span` is less than `amount`, since a `Span`'s size can't be negative.
    pub fn shrink_front(&mut self, amount: u32) {
        if self.len() < amount {
            panic!("cannot create negative-size span");
        }
        self.end -= amount;
    }

    /// Checks if a `Span`'s size is 0. Returns true if 0, and false if anything else.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Gets the length of a `Span`.
    pub fn len(&self) -> u32 {
        self.end - self.start
    }

    /// Attempts to apply a `Span` to a given file. Returns the area that the `Span` is pointing to.
    ///
    /// # Panics
    /// Panics if the `Span` is not from the given file, or if the given file is too short to apply the span to.
    pub fn apply(&self) -> &'src str {
        self.try_apply().expect("span is not contained in file")
    }

    /// Non-panicking version of [`apply`](Span::apply). Instead, it returns a `Some` value if successful.
    ///
    /// # Returns
    /// Returns `None` if the `Span` is not from the file, or if the file is too short to apply the span to.
    pub fn try_apply(&self) -> Option<&'src str> {
        if self.src.len() >= self.end as usize {
            Some(self.apply_unchecked())
        } else {
            None
        }
    }

    fn apply_unchecked(&self) -> &'src str {
        &self.src[self.start as usize..self.end as usize]
    }

    /// Gets a tuple of `(line, column)` for the start of the `Span`.
    pub fn get_start_code_pos(&self) -> (u32, u32) {
        let src = self.src.chars();
        let mut column = 1;
        let mut line = 1;
        for i in src.enumerate() {
            if i.0 == self.start as usize {
                break;
            }
            match i.1 {
                '\n' => {
                    line += 1;
                    column = 1
                }
                _ => column += 1,
            }
        }
        (line, column)
    }

    /// Gets a tuple of `(line, column)` for the end of the `Span`.
    pub fn get_end_code_pos(&self) -> (u32, u32) {
        let src = self.src.chars();
        let mut column = 1;
        let mut line = 1;
        for i in src.enumerate() {
            if i.0 == self.end as usize {
                break;
            }
            match i.1 {
                '\n' => {
                    line += 1;
                    column = 1
                }
                _ => column += 1,
            }
        }
        (line, column)
    }
}

impl<'src> From<Span<'src>> for Range<usize> {
    fn from(val: Span<'src>) -> Self {
        val.start as usize..val.end as usize
    }
}

impl<'src> Display for Span<'src> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (line, column) = self.get_start_code_pos();
        write!(f, "{line}:{column}")
    }
}
