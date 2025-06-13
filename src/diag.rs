#![deny(missing_docs)]
//! Implementation of `rustc`'s `Diag` diagnostic system.

use std::sync::RwLock;

static DIAGS: RwLock<Vec<Diag>> = RwLock::new(vec![]);

use colored::Colorize;

use crate::span::Span;

/// Diagnostic. Use [`DiagBuilder`] to create.
///
/// # Examples
/// ```
/// use escoop::diag::{DiagBuilder, DiagLevel};
///
/// DiagBuilder::new(DiagLevel::Fatal)
/// .message(DiagLevel::Fatal, "unknown character '`'")
/// .finish()
/// .finish()
/// .emit();
/// ```
///
/// If a `Diag` is dropped without being consumed, then it will panic.
/// ```should_panic
/// use escoop::diag::{DiagBuilder, DiagLevel};
///
/// DiagBuilder::new(DiagLevel::Fatal)
/// .message(DiagLevel::Fatal, "unknown character '`'")
/// .finish()
/// .finish();
/// ```
pub struct Diag {
    consumed: bool,
    level: DiagLevel,
    messages: Vec<DiagMessage>,
}

/// Builder struct for [`Diag`].
pub struct DiagBuilder {
    inner: Diag,
}

impl DiagBuilder {
    /// Creates a `DiagBuilder` with `level` as the [`DiagLevel`].
    #[must_use]
    pub fn new(level: DiagLevel) -> Self {
        DiagBuilder {
            inner: Diag::new(level),
        }
    }

    /// Creates a new [`DiagMessageBuilder`] with `level` as the [`DiagLevel`], and `msg` as the message.
    #[must_use]
    pub fn message(self, level: DiagLevel, msg: impl ToString) -> DiagMessageBuilder {
        DiagMessageBuilder {
            inner: DiagMessage {
                level,
                msg: msg.to_string(),
                span: None,
            },
            builder: self,
        }
    }

    /// Finish the `DiagBuilder` to create the [`Diag`].
    #[must_use]
    pub fn finish(self) -> Diag {
        self.inner
    }
}

/// Diagnostic message used for [`Diag`]. Create using [`DiagMessageBuilder::finish`].
pub struct DiagMessage {
    level: DiagLevel,
    msg: String,
    span: Option<Span>,
}

/// Builder struct for [`DiagMessage`]. Create using [`DiagBuilder::message`].
pub struct DiagMessageBuilder {
    inner: DiagMessage,
    builder: DiagBuilder,
}

impl DiagMessageBuilder {
    /// Sets the `Span` for the current [`DiagMessage`].
    #[must_use]
    pub fn set_span(mut self, span: Span) -> Self {
        self.inner.span = Some(span);
        self
    }

    /// Finish the `DiagMessageBuilder` to create the [`DiagMessage`].
    #[must_use]
    pub fn finish(mut self) -> DiagBuilder {
        self.builder.inner.messages.push(self.inner);
        self.builder
    }
}

impl Diag {
    fn new(level: DiagLevel) -> Self {
        Diag {
            messages: vec![],
            level,
            consumed: false,
        }
    }

    /// Flushes all emitted `Diag`s to the console.
    pub fn flush() {
        for i in DIAGS.try_write().unwrap().drain(..) {
            for i in &i.messages {
                let (name, color) = match i.level {
                    DiagLevel::Error => ("error", colored::Color::BrightRed),
                    DiagLevel::Warn => todo!(),
                    DiagLevel::Fatal => ("error", colored::Color::BrightRed),
                    DiagLevel::Note => todo!(),
                    DiagLevel::OnceNote => todo!(),
                    DiagLevel::Help => ("help", colored::Color::BrightCyan),
                    DiagLevel::OnceHelp => todo!(),
                    DiagLevel::Bug => todo!(),
                    DiagLevel::DelayedBug => todo!(),
                };
                let name = name.bold().color(color);

                eprintln!("{}{}{}", name, ": ".bold(), i.msg.bold());
                if let Some(span) = i.span {
                    span.display_span(color);
                }
            }
        }
    }

    /// Emits `Diag` to be printed to the console. If the `Diag` is fatal, then `emit` will flush immediately.
    pub fn emit(mut self) {
        let fatal = matches!(self.level, DiagLevel::Fatal);
        self.consumed = true;
        DIAGS.try_write().unwrap().push(self);
        if fatal {
            Diag::flush();
        }
    }
}

impl Drop for Diag {
    fn drop(&mut self) {
        if !self.consumed {
            panic!("diag was dropped without being consumed");
        }
    }
}

/// Level of severity for `Diag`
pub enum DiagLevel {
    /// Error that causes the compiler to stop compiling.
    Error,
    /// Warning that the compiler can continue with, but usually indicates a mistake.
    Warn,
    /// Error that causes the compiler to stop compiling immediately.
    Fatal,
    /// Information possibly useful to the user that doesn't necessarily need to be acted upon.
    Note,
    /// `Note`, but is guaranteed to only be emitted once.
    OnceNote,
    /// Information possibly useful to the user that should be acted upon.
    Help,
    /// `Help`, but is guaranteed to only be emitted once.
    OnceHelp,

    /// Bug in compiler, not code
    Bug,
    /// If other errors are emitted before this is dropped, ignore. otherwise, emit as a `Bug`
    DelayedBug,
}
