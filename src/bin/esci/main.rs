use std::{fs, path::PathBuf};

use clap::{Parser as ClapParser, ValueEnum};
use escoop::{
    diag::{DiagBuilder, DiagLevel},
    lexer,
};

#[derive(ClapParser)]
struct Args {
    #[arg(short, long, value_name = "FILE")]
    file: Option<PathBuf>,

    #[arg(short, long)]
    debug: Option<DebugMode>,
}

#[derive(Copy, Clone, PartialEq, Eq, ValueEnum)]
enum DebugMode {
    Lexer,
}

fn main() {
    let args = Args::parse();
    let path = args
        .file
        .unwrap_or(PathBuf::from("tests/hello-world-simple/entrypoint.scp"));
    let file = if let Ok(file) = fs::read_to_string(&path) {
        file
    } else {
        DiagBuilder::new(DiagLevel::Fatal)
            .message(
                DiagLevel::Fatal,
                format!("could not open file {}", path.to_string_lossy()),
            )
            .finish()
            .finish()
            .emit();
        return;
    };

    let lexer = lexer::Lexer::new_with_path(file.as_str(), &path).unwrap();

    if matches!(args.debug, Some(DebugMode::Lexer)) {
        for i in lexer {
            println!("{i:?}");
        }
    }
}
