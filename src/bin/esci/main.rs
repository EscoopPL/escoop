use std::{fs, path::PathBuf};

use clap::{Parser as ClapParser, ValueEnum};
use escoop::{lexer::Lexer, parser::Parser, Source};

#[derive(ClapParser)]
struct Args {
    #[arg(short, long, value_name = "FILE")]
    file: Option<PathBuf>,

    #[arg(short, long)]
    profiling: bool,

    #[arg(short, long)]
    verbose: bool,

    #[arg(short, long)]
    debug: Option<DebugMode>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, ValueEnum)]
enum DebugMode {
    Lexer,
    Parser,
}

fn main() {
    let args = Args::parse();
    let path = args
        .file
        .unwrap_or(PathBuf::from("escoop-tests/hello-world/entrypoint.scp"));
    let file = match fs::read_to_string(&path) {
        Ok(file) => file,
        Err(err) => {
            let mut msg = format!("could not open `{}`: {}", path.to_string_lossy(), err);
            if args.verbose {
                msg += format!(" ({:?})", err.kind()).as_str();
            }
            println!("{msg}");
            return;
        }
    };

    let src = Source::new(file.as_str(), path);

    if matches!(args.debug, Some(DebugMode::Lexer)) {
        if args.profiling {
            for _ in 0..1000000 {
                let lexer = Lexer::new(&src);
                for _ in lexer {
                    //println!("{:?}", i);
                }
            }
        } else {
            let lexer = Lexer::new(&src);
            for i in lexer {
                println!("{:?}", i);
            }
        }
    }

    if matches!(args.debug, Some(DebugMode::Parser)) {
        if args.profiling {
            for _ in 0..1000000 {
                let mut parser = Parser::new(&src);
                for _ in parser.parse() {
                    //println!("{:?}", i);
                }
            }
        } else {
            let mut parser = Parser::new(&src);
            for i in parser.parse() {
                println!("{:?}", i);
            }
        }
    }
}
