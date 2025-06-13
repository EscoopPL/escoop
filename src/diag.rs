#![deny(missing_docs)]
//! Implementation of `rustc`'s `Diag` diagnostic system.

use std::sync::{OnceLock, RwLock};

static DIAGS: RwLock<Vec<Diag>> = RwLock::new(vec![]);
static BUG_FOUND: OnceLock<()> = OnceLock::new();

/// Will return true if there is a bug found in the compiler. Useful for testing.
pub fn bug() -> bool {
    BUG_FOUND.get().is_some()
}

pub(crate) fn set_bug() {
    let _ = BUG_FOUND.set(()); // If there's already a bug found, we don't care.
}

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

    /// Emits a generic no-span one message fatal diagnostic.
    pub fn emit_fatal(msg: impl ToString) {
        Diag::emit_new(msg, DiagLevel::Fatal);
    }

    /// Emits a generic no-span diagnostic.
    pub fn emit_new(msg: impl ToString, level: DiagLevel) {
        let msg = msg.to_string();

        Diag {
            messages: vec![
                DiagMessage {
                    level,
                    span: None,
                    msg,
                }
            ],
            level,
            consumed: false,
        }.emit();
    }

    /// Flushes all emitted `Diag`s to the console.
    pub fn flush() {
        let drain: Vec<Diag> = DIAGS.try_write().unwrap().drain(..).collect();
        for i in drain {
            for j in &i.messages {
                let (name, color) = match j.level {
                    DiagLevel::Error => ("error", colored::Color::BrightRed),
                    DiagLevel::Warn => todo!(),
                    DiagLevel::Fatal => ("error", colored::Color::BrightRed),
                    DiagLevel::Note => todo!(),
                    DiagLevel::OnceNote => todo!(),
                    DiagLevel::Help => ("help", colored::Color::BrightCyan),
                    DiagLevel::OnceHelp => todo!(),
                    DiagLevel::Bug => {
                        ("bug", colored::Color::BrightCyan)
                    },
                    DiagLevel::DelayedBug => todo!(),
                };
                let name = name.bold().color(color);

                let j_msg = j.msg.replace('\n', format!("\n{}", " ".repeat(name.len() + 2)).as_str());
                eprintln!("{}{}{}", name, ": ".bold(), j_msg.bold());
                if let Some(span) = j.span {
                    span.display_span(color);
                }
            }
        }
    }

    /// Emits `Diag` to be printed to the console. If the `Diag` is fatal, then `emit` will flush immediately.
    pub fn emit(mut self) {
        let level = self.level;
        self.consumed = true;
        DIAGS.try_write().unwrap().push(self);
        match level {
            DiagLevel::Fatal => Diag::flush(),
            DiagLevel::Bug => set_bug(),
            _ => {}
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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
