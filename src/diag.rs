#![deny(missing_docs)]
//! Implementation of `rustc`'s `Diag` diagnostic system.

use std::{mem, sync::OnceLock};

use codespan_reporting::{diagnostic::{Diagnostic, Label, Severity}, term};
use termcolor::{ColorChoice, StandardStream};

use crate::Source;

static BUG_FOUND: OnceLock<()> = OnceLock::new();

/// Will return true if there is a bug found in the compiler. Useful for testing.
pub fn bug() -> bool {
    BUG_FOUND.get().is_some()
}

pub(crate) fn set_bug() {
    let _ = BUG_FOUND.set(()); // If there's already a bug found, we don't care.
}

static ERROR_HIT: OnceLock<()> = OnceLock::new();

/// Will return true if an error was found while compiling.
pub fn error() -> bool {
    ERROR_HIT.get().is_some()
}

fn set_error() {
    let _ = ERROR_HIT.set(()); // If there's already a error, we don't care.
}

/// Custom Diagnostic message type as a wrapper around [`codespan_reporting::Diagnostic`](Diagnostic)
pub struct Diag<'src> {
    report: Diagnostic<()>,
    src: &'src Source<&'src str>,
}

impl<'src> Diag<'src> {
    /// Creates a [`DiagBuilder`] using the `src` source and a severity of `severity`.
    pub fn build(src: &'src Source<&'src str>, severity: Severity) -> DiagBuilder<'src> {
        DiagBuilder {
            inner: Diag {
                report: Diagnostic::new(severity),
                src,
            }
        }
    }

    /// Creates a [`DiagBuilder`] using the `src` source and a severity of [`Error`](Severity::Error).
    pub fn error(src: &'src Source<&'src str>) -> DiagBuilder<'src> {
        Self::build(src, Severity::Error)
    }

    /// Creates a [`DiagBuilder`] using the `src` source and a severity of [`Warning`](Severity::Warning).
    pub fn warn(src: &'src Source<&'src str>) -> DiagBuilder<'src> {
        Self::build(src, Severity::Warning)
    }

    /// Creates a [`DiagBuilder`] using the `src` source and a severity of [`Bug`](Severity::Bug).
    pub fn bug(src: &'src Source<&'src str>) -> DiagBuilder<'src> {
        Self::build(src, Severity::Bug)
    }

    /// Creates a [`DiagBuilder`] using the `src` source and a severity of [`Help`](Severity::Help).
    pub fn help(src: &'src Source<&'src str>) -> DiagBuilder<'src> {
        Self::build(src, Severity::Help)
    }

    /// Creates a [`DiagBuilder`] using the `src` source and a severity of [`Note`](Severity::Note).
    pub fn note(src: &'src Source<&'src str>) -> DiagBuilder<'src> {
        Self::build(src, Severity::Note)
    }

    /// Emit the `Diag`
    pub fn emit(self) {} // Drop self to trigger the emission
}

impl<'src> Drop for Diag<'src> {
    fn drop(&mut self) {
        if matches!(self.report.severity, Severity::Bug) {
            set_bug();
        } else if matches!(self.report.severity, Severity::Error) {
            set_error();
        };
        let writer = StandardStream::stdout(ColorChoice::Always);
        let config = codespan_reporting::term::Config::default();
        term::emit(&mut writer.lock(), &config, self.src, &self.report).expect("bug");
    }
}

/// A builder for type [`Diag`].
pub struct DiagBuilder<'src> {
    inner: Diag<'src>,
}

impl<'src> DiagBuilder<'src> {
    /// Calls [`codespan_reporting::Diagnostic::with_code`](Diagnostic::with_code)
    pub fn with_code(mut self, code: impl ToString) -> Self {
        let report = mem::replace(&mut self.inner.report, Diagnostic::new(Severity::Bug))
            .with_code(code); // Using a cool mem::replace trick from GitHub Copilot
        self.inner.report = report;
        self
    }

    /// Calls [`codespan_reporting::Diagnostic::with_label`](Diagnostic::with_label)
    pub fn with_label(mut self, label: Label<()>) -> Self {
        let report = mem::replace(&mut self.inner.report, Diagnostic::new(Severity::Bug))
            .with_label(label); // Using a cool mem::replace trick from GitHub Copilot
        self.inner.report = report;
        self
    }

    /// Calls [`codespan_reporting::Diagnostic::with_labels`](Diagnostic::with_labels)
    pub fn with_labels(mut self, labels: Vec<Label<()>>) -> Self {
        let report = mem::replace(&mut self.inner.report, Diagnostic::new(Severity::Bug))
            .with_labels(labels); // Using a cool mem::replace trick from GitHub Copilot
        self.inner.report = report;
        self
    }

    /// Calls [`codespan_reporting::Diagnostic::with_labels_iter`](Diagnostic::with_labels_iter)
    pub fn with_labels_iter(mut self, labels: impl IntoIterator<Item = Label<()>>) -> Self {
        let report = mem::replace(&mut self.inner.report, Diagnostic::new(Severity::Bug))
            .with_labels_iter(labels); // Using a cool mem::replace trick from GitHub Copilot
        self.inner.report = report;
        self
    }

    /// Calls [`codespan_reporting::Diagnostic::with_message`](Diagnostic::with_message)
    pub fn with_message(mut self, message: impl ToString) -> Self {
        let report = mem::replace(&mut self.inner.report, Diagnostic::new(Severity::Bug))
            .with_message(message); // Using a cool mem::replace trick from GitHub Copilot
        self.inner.report = report;
        self
    }

    /// Calls [`codespan_reporting::Diagnostic::with_note`](Diagnostic::with_note)
    pub fn with_note(mut self, note: impl ToString) -> Self {
        let report = mem::replace(&mut self.inner.report, Diagnostic::new(Severity::Bug))
            .with_note(note); // Using a cool mem::replace trick from GitHub Copilot
        self.inner.report = report;
        self
    }

    /// Calls [`codespan_reporting::Diagnostic::with_notes`](Diagnostic::with_notes)
    pub fn with_notes(mut self, notes: Vec<String>) -> Self {
        let report = mem::replace(&mut self.inner.report, Diagnostic::new(Severity::Bug))
            .with_notes(notes); // Using a cool mem::replace trick from GitHub Copilot
        self.inner.report = report;
        self
    }

    /// Calls [`codespan_reporting::Diagnostic::with_notes_iter`](Diagnostic::with_notes_iter)
    pub fn with_notes_iter(mut self, notes: impl IntoIterator<Item = String>) -> Self {
        let report = mem::replace(&mut self.inner.report, Diagnostic::new(Severity::Bug))
            .with_notes_iter(notes); // Using a cool mem::replace trick from GitHub Copilot
        self.inner.report = report;
        self
    }

    /// Finishes the `DiagBuilder`
    pub fn finish(self) -> Diag<'src> {
        self.inner
    }
}