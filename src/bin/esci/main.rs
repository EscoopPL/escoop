use std::{fs, io::Error, path::PathBuf};

use clap::{Parser as ClapParser, ValueEnum};
use escoop::lexer;

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

fn main() -> Result<(), Error> {
    clang_log::init(log::Level::Trace, "esci");

    let args = Args::parse();
    let path = args
        .file
        .unwrap_or(PathBuf::from("tests/hello-world-simple/entrypoint.scp"));
    let file = fs::read_to_string(&path)?;
    let lexer = lexer::Lexer::new_with_path(file.as_str(), &path)?;

    if matches!(args.debug, Some(DebugMode::Lexer)) {
        for i in lexer {
            println!("{i:?}");
        }
        return Ok(());
    }
    Ok(())
}
