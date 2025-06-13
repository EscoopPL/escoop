use std::{
    collections::HashMap,
    hash::{DefaultHasher, Hash, Hasher},
    sync::{LazyLock, RwLock},
};

use colored::Colorize;

use crate::span::Span;

static DIAGS: RwLock<Vec<Diag>> = RwLock::new(vec![]);
static FILES: LazyLock<RwLock<HashMap<u64, (String, String)>>> =
    LazyLock::new(|| RwLock::new(HashMap::new()));

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

pub(crate) fn display_span(span: Span, color: colored::Color) -> bool {
    let span_rep = span.to_string();
    if !span_rep.is_empty() {
        println!(" {} {}", "-->".blue().bold(), span); // Print arrow ( --> tests/hello-world-simple/entrypoint.scp:7:15)
    }

    let lock = FILES.try_read().unwrap();
    if let Some(mut span_file) = lock.get(&span.file).map(|x| x.1.chars()) {
        // Get source of span as Chars<'_>
        let mut line_span = span; // Expand span to encompass the entire line of itself

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
            let msg_span_start = span.start - line_start; // Calculate the beginnning of the span relative to the beginning of the line
            let msg_span_len = span.end - span.start; // Calculate the length of the span

            println!("  {}", "|".blue().bold()); // Show first pipe (   |)
            println!("  {} {}", "|".blue().bold(), full_span); // Show message
            println!(
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

pub struct Diag {
    consumed: bool,
    level: DiagLevel,
    messages: Vec<DiagMessage>,
}

pub struct DiagBuilder {
    inner: Diag,
}

impl DiagBuilder {
    pub fn new(level: DiagLevel) -> Self {
        DiagBuilder {
            inner: Diag::new(level),
        }
    }

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

    pub fn finish(self) -> Diag {
        self.inner
    }
}

pub struct DiagMessage {
    level: DiagLevel,
    msg: String,
    span: Option<Span>,
}

pub struct DiagMessageBuilder {
    inner: DiagMessage,
    builder: DiagBuilder,
}

impl DiagMessageBuilder {
    pub fn set_span(mut self, span: Span) -> Self {
        self.inner.span = Some(span);
        self
    }

    pub fn finish(mut self) -> DiagBuilder {
        self.builder.inner.messages.push(self.inner);
        self.builder
    }
}

impl Diag {
    pub fn add_file(path: String, source: String) {
        let mut hasher = DefaultHasher::new();
        source.hash(&mut hasher);
        let hash = hasher.finish();
        FILES.try_write().unwrap().insert(hash, (path, source));
    }

    fn new(level: DiagLevel) -> Self {
        Diag {
            messages: vec![],
            level,
            consumed: false,
        }
    }

    pub fn flush() {
        for i in &*DIAGS.try_read().unwrap() {
            for i in &i.messages {
                let (name, color) = match i.level {
                    DiagLevel::Error => ("error", colored::Color::Red),
                    DiagLevel::Warn => todo!(),
                    DiagLevel::Fatal => ("error", colored::Color::Red),
                    DiagLevel::Note => todo!(),
                    DiagLevel::OnceNote => todo!(),
                    DiagLevel::Help => todo!(),
                    DiagLevel::OnceHelp => todo!(),
                    DiagLevel::Bug => todo!(),
                    DiagLevel::DelayedBug => todo!(),
                };
                let name = name.bold().color(color);

                println!("{}{}{}", name, ": ".bold(), i.msg.bold());
                if let Some(span) = i.span {
                    display_span(span, color);
                }
            }
        }
    }

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

pub enum DiagLevel {
    Error,
    Warn,
    Fatal,
    Note,
    OnceNote,
    Help,
    OnceHelp,

    /// Bug in compiler, not code
    Bug,
    /// If other errors are emitted before this is dropped, ignore. otherwise, emit as a `Bug`
    DelayedBug,
}
