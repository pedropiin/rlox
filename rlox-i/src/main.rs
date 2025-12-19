use std::env;
use std::process;
use std::fs;
use std::io;

use crate::token::Token;
use crate::scanner::Scanner;

mod token;
mod scanner;
mod token_type;

enum LexerErrors {
    SourceReadError,
}

impl LexerErrors {
    fn message(&self) -> &str {
        match self {
            LexerErrors::SourceReadError  => "Could not read/open source lox file.",
        }
    }
}

fn error(line: usize, error_type: LexerErrors) {
    fn report(line: usize, locale: &str, error_type: LexerErrors) {
        eprintln!("[line + {}] Error {}: {}", line, locale, error_type.message());
    }

    report(line, "", error_type);
}

fn run(source: &str) {
    let mut scanner: Scanner = Scanner::new(source);
    let tokens: Vec<Token> = scanner.scan_tokens(); 

    for tok in tokens {
        println!("{}", tok);
    }
}

fn run_file(path: &String) {
    let contents: String = fs::read_to_string(&path).expect(LexerErrors::SourceReadError.message());
    run(&contents);
}

fn run_prompt() {
    let mut input_buf: String = String::from("");
    loop {
        println!("> ");
        io::stdin().read_line(&mut input_buf).unwrap();
        run(&input_buf);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        eprintln!("Usage: rlox [filename.lox].");
        process::exit(64);      // following UNIX "sysexits.h" header convention
    } else if args.len() == 1 {
        run_file(&args[0]);
    } else {
        run_prompt();
    }
}
