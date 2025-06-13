#![deny(missing_docs)]
//! Module holding the `Span` type, which represents an area of a file.

use std::{
    collections::HashMap,
    fmt::Display,
    hash::{DefaultHasher, Hash, Hasher},
    sync::{LazyLock, RwLock},
};

use colored::Colorize;

static FILES: LazyLock<RwLock<HashMap<u64, (String, String)>>> =
    LazyLock::new(|| RwLock::new(HashMap::new()));

pub(crate) fn add_file(path: String, source: String) {
    let mut hasher = DefaultHasher::new();
    source.hash(&mut hasher);
    let hash = hasher.finish();
    FILES.try_write().unwrap().insert(hash, (path, source));
}

pub(crate) fn get_file_name(hash: u64) -> Option<String> {
    FILES.try_read().unwrap().get(&hash).map(|x| x.0.clone())
}

pub(crate) fn apply_span(span: Span) -> Option<String> {
    let lock = FILES.try_read().unwrap();
    let str = lock.get(&span.file).map(|x| &x.1)?;
    span.try_apply(str).map(|x| x.to_string())
}

pub(crate) fn get_code_pos(hash: u64, pos: usize) -> Option<(usize, usize)> {
    let lock = FILES.try_read().unwrap();
    let src = lock.get(&hash)?.1.chars();
    let mut column = 1;
    let mut line = 1;
    for i in src.enumerate() {
        if i.0 == pos {
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
    Some((line, column))
}

/// The `Span` type represents an area of a file.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span {
    pub(crate) start: usize,
    pub(crate) end: usize,
    pub(crate) file: u64,
}

impl Span {
    /// Creates a new `Span` from a file. This span will start and end at the 0th character, making it have a length of zero.
    ///
    /// # Examples
    /// ```
    /// use escoop::span::Span;
    ///
    /// let file = "foo bar baz";
    /// let mut span = Span::new(file);
    /// span.grow_front(3);
    /// assert_eq!(span.apply(file), "foo");
    /// ```
    pub fn new(file: &str) -> Self {
        let mut hasher = DefaultHasher::new();
        file.hash(&mut hasher);
        Span {
            start: 0,
            end: 0,
            file: hasher.finish(),
        }
    }

    /// Creates a new `Span` from a file and a pair of start and end indexes. These indexes are indexes into the file by characters.
    ///
    /// # Examples
    /// ```
    /// use escoop::span::Span;
    ///
    /// let file = "foo bar baz";
    /// let span = Span::new_from(file, 4, 7);
    /// assert_eq!(span.apply(file), "bar");
    /// ```
    pub fn new_from(file: &str, start: usize, end: usize) -> Self {
        let mut hasher = DefaultHasher::new();
        file.hash(&mut hasher);
        Span {
            start,
            end,
            file: hasher.finish(),
        }
    }

    pub(crate) fn update(&mut self) {
        self.start = self.end;
    }

    /// Grows the span from the front. This moves the end value up by `amount`.
    ///
    /// # Examples
    /// ```
    /// use escoop::span::Span;
    ///
    /// let file = "foo bar baz";
    /// let mut span = Span::new_from(file, 4, 5);
    /// assert_eq!(span.apply(file), "b");
    /// span.grow_front(2);
    /// assert_eq!(span.apply(file), "bar");
    /// ```
    pub fn grow_front(&mut self, amount: usize) {
        self.end += amount;
    }

    /// Grows the span from the back. This moves the start value back by `amount`.
    ///
    /// # Examples
    /// ```
    /// use escoop::span::Span;
    ///
    /// let file = "foo bar baz";
    /// let mut span = Span::new_from(file, 6, 7);
    /// assert_eq!(span.apply(file), "r");
    /// span.grow_back(2);
    /// assert_eq!(span.apply(file), "bar");
    /// ```
    pub fn grow_back(&mut self, amount: usize) {
        self.start -= amount;
    }

    /// Shrinks the span from the back. This moves the start value up by `amount`.
    ///
    /// # Panics
    /// This method will panic if the size of the `Span` is less than `amount`, since a `Span`'s size can't be negative.
    ///
    /// # Examples
    /// ```
    /// use escoop::span::Span;
    ///
    /// let file = "foo bar baz";
    /// let mut span = Span::new_from(file, 2, 7);
    /// assert_eq!(span.apply(file), "o bar");
    /// span.shrink_back(2);
    /// assert_eq!(span.apply(file), "bar");
    /// ```
    pub fn shrink_back(&mut self, amount: usize) {
        if self.len() < amount {
            panic!("cannot create negative-size span");
        }
        self.start += amount;
    }

    /// Shrinks the span from the front. This moves the end value back by `amount`.
    ///
    /// # Panics
    /// This method will panic if the size of the `Span` is less than `amount`, since a `Span`'s size can't be negative.
    ///
    /// # Examples
    /// ```
    /// use escoop::span::Span;
    ///
    /// let file = "foo bar baz";
    /// let mut span = Span::new_from(file, 4, 9);
    /// assert_eq!(span.apply(file), "bar b");
    /// span.shrink_front(2);
    /// assert_eq!(span.apply(file), "bar");
    /// ```
    pub fn shrink_front(&mut self, amount: usize) {
        if self.len() < amount {
            panic!("cannot create negative-size span");
        }
        self.end -= amount;
    }

    /// Checks if a `Span`'s size is 0. Returns true if 0, and false if anything else.
    ///
    /// # Examples
    /// ```
    /// use escoop::span::Span;
    ///
    /// let file = "foo bar baz";
    /// let span = Span::new_from(file, 4, 4);
    /// assert!(span.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Gets the length of a `Span`.
    ///
    /// # Examples
    /// ```
    /// use escoop::span::Span;
    ///
    /// let file = "foo bar baz";
    /// let span = Span::new_from(file, 4, 6);
    /// assert_eq!(span.len(), 2);
    /// ```
    pub fn len(&self) -> usize {
        self.end - self.start
    }

    /// Checks if two `Span`s are from the same file. If they are, returns true. Otherwise, returns false.
    ///
    /// # Examples
    /// ```
    /// use escoop::span::Span;
    ///
    /// let file = "foo bar baz";
    /// let span1 = Span::new_from(file, 0, 3);
    /// let span2 = Span::new_from(file, 4, 7);
    /// assert!(span1.from_same_file(&span2));
    /// ```
    pub fn from_same_file(&self, other: &Self) -> bool {
        self.file == other.file
    }

    /// Checks if a `Span` is from the given file. If it is, returns true. Otherwise, returns false.
    ///
    /// # Examples
    /// ```
    /// use escoop::span::Span;
    ///
    /// let file = "foo bar baz";
    /// let span = Span::new_from(file, 8, 11);
    /// assert!(span.from_file(file));
    /// ```
    pub fn from_file(&self, file: &str) -> bool {
        let mut hasher = DefaultHasher::new();
        file.hash(&mut hasher);
        hasher.finish() == self.file
    }

    /// Attempts to apply a `Span` to a given file. Returns the area that the `Span` is pointing to.
    ///
    /// # Panics
    /// Panics if the `Span` is not from the given file, or if the given file is too short to apply the span to.
    ///
    /// # Examples
    /// ```
    /// use escoop::span::Span;
    ///
    /// let file = "foo bar baz";
    /// let span = Span::new_from(file, 8, 11);
    /// assert_eq!(span.apply(file), "baz");
    /// ```
    ///
    /// ```should_panic
    /// use escoop::span::Span;
    ///
    /// let file = "foo bar baz";
    /// let span = Span::new_from(file, 8, 11);
    /// let file2 = "baz qux foo";
    /// span.apply(file2); // Panics
    /// ```
    ///
    /// ```should_panic
    /// use escoop::span::Span;
    ///
    /// let file = "foo bar";
    /// let span = Span::new_from(file, 8, 11);
    /// span.apply(file); // Panics
    /// ```
    pub fn apply<'a>(&self, file: &'a str) -> &'a str {
        if !self.from_file(file) {
            panic!("span should be applied to original file");
        }
        self.try_apply(file).expect("span is not contained in file")
    }

    /// Non-panicking version of [`apply`](Span::apply). Instead, it returns a `Some` value if successful.
    ///
    /// # Returns
    /// Returns `None` if the `Span` is not from the file, or if the file is too short to apply the span to.
    ///
    /// # Examples
    ///
    /// ```
    /// use escoop::span::Span;
    ///
    /// let file = "foo bar baz";
    /// let span = Span::new_from(file, 8, 11);
    /// assert_eq!(span.try_apply(file), Some("baz"));
    /// ```
    ///
    /// ```
    /// use escoop::span::Span;
    ///
    /// let file = "foo bar baz";
    /// let span = Span::new_from(file, 8, 11);
    /// let file2 = "baz qux foo";
    /// assert_eq!(span.try_apply(file2), None);
    /// ```
    ///
    /// ```
    /// use escoop::span::Span;
    ///
    /// let file = "foo bar";
    /// let span = Span::new_from(file, 8, 11);
    /// assert_eq!(span.try_apply(file), None);
    pub fn try_apply<'a>(&self, file: &'a str) -> Option<&'a str> {
        if self.from_file(file) {
            if file.len() >= self.end {
                Some(self.apply_unchecked(file))
            } else {
                None
            }
        } else {
            None
        }
    }

    fn apply_unchecked<'a>(&self, file: &'a str) -> &'a str {
        &file[self.start..self.end]
    }

    pub(crate) fn display_span(self, color: colored::Color) -> bool {
        let span_rep = self.to_string();
        if !span_rep.is_empty() {
            eprintln!(" {} {}", "-->".blue().bold(), self); // Print arrow ( --> tests/hello-world-simple/entrypoint.scp:7:15)
        }

        let lock = FILES.try_read().unwrap();
        if let Some(mut span_file) = lock.get(&self.file).map(|x| x.1.chars()) {
            // Get source of span as Chars<'_>
            let mut line_span = self; // Expand span to encompass the entire line of itself

            let mut column = 1;
            for i in span_file.by_ref().enumerate() {
                // Loop over the source until we reach the start of the span
                if i.0 == line_span.start {
                    break;
                }
                match i.1 {
                    '\n' => column = 1,
                    _ => column += 1,
                }
            }
            let line_start = line_span.start - column + 1; // Calculate the index into the string that provides the start of the line

            let old_column = column; // Save column for future use
            let mut end = false;
            for i in span_file.by_ref().enumerate() {
                if i.0 + line_span.start + 1 == line_span.end {
                    end = true; // If reached end, wait until new line
                }
                match i.1 {
                    '\n' => {
                        if end {
                            break;
                        } else {
                            column = 1;
                        }
                    }
                    _ => column += 1,
                }
            }
            let line_end = line_span.end + (column - old_column); // Calculate the index into the string that provides the end of the line

            line_span.start = line_start; // Update the span
            line_span.end = line_end; // Update the span

            if let Some(full_span) = apply_span(line_span) {
                // Get the text belonging to the line's span
                let msg_span_start = self.start - line_start; // Calculate the beginnning of the span relative to the beginning of the line
                let msg_span_len = self.end - self.start; // Calculate the length of the span

                eprintln!("  {}", "|".blue().bold()); // Show first pipe (   |)
                eprintln!("  {} {}", "|".blue().bold(), full_span); // Show message
                eprintln!(
                    "  {} {}{}",
                    "|".blue().bold(),
                    " ".repeat(msg_span_start),
                    "^".repeat(msg_span_len).color(color).bold()
                ); // Show carots under message to show where the span is
                true
            } else {
                false
            }
        } else {
            false
        }
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
