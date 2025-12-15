use std::env;
use std::process;


enum LexerErrors {
    UsageError,
}

impl LexerErrors {
    fn message(&self) -> &str {
        match self {
            LexerErrors::UsageError => "Usage: rlox [filename.lox].",
        }
    }
}

fn run_file(path: &String) {
    todo!();
}

fn run_prompt() {
    todo!();
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        eprintln!("{}", LexerErrors::UsageError.message());
        process::exit(64);      // following UNIX "sysexits.h" header convention
    } else if args.len() == 1 {
        run_file(&args[0]);
    } else {
        run_prompt();
    }
}
