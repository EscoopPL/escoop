use std::{fs, path::PathBuf};

use clap::{Parser as ClapParser, ValueEnum};
use escoop::{
    diag::{Diag},
    lexer,
};

#[derive(ClapParser)]
struct Args {
    #[arg(short, long, value_name = "FILE")]
    file: Option<PathBuf>,

    #[arg(short, long)]
    verbose: bool,

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
    let file = match fs::read_to_string(&path) {
        Ok(file) => {
            file
        }
        Err(err) => {
            let mut msg = format!("could not open `{}`: {}", path.to_string_lossy(), err);
            if args.verbose {
                msg += format!(" ({:?})", err.kind()).as_str();
            }
            Diag::emit_fatal(msg);
            return;
        }
        
    };

    let lexer = lexer::Lexer::new_with_path(file.as_str(), &path);

    if matches!(args.debug, Some(DebugMode::Lexer)) {
        for i in lexer {
            println!("{i:?}");
        }
    }
}
