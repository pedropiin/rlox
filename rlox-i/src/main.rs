use std::env;
use std::process;
use std::fs;
use std::io::{self, Write};

use crate::token::Token;
use crate::lexer::Lexer;
use crate::lexer::LexerErrors;

mod token;
mod lexer;
mod token_type;

pub fn error(line: usize, error_type: LexerErrors) {
    fn report(line: usize, locale: &str, error_type: LexerErrors) {
        eprintln!("[line {}] Error {}: {}", line, locale, error_type.message());
    }

    report(line, "", error_type);
}

fn run(source: &str) -> bool {
    let mut tokens: Vec<Token> = Vec::new();
    let mut scanner: Lexer = Lexer::new(source, &mut tokens);
    let had_error: bool = scanner.scan_tokens(); 

    for tok in &tokens {
        println!("{}", tok);
    }

    had_error
}

fn run_file(path: &String) {
    let contents: String = fs::read_to_string(&path).expect(LexerErrors::SourceReadError.message().as_str());
    let had_error: bool = run(&contents);

    if had_error {
        process::exit(65);
    }
}

fn run_prompt() {
    let mut input_buf: String = String::from("");
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input_buf).unwrap();
        run(&input_buf);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 2 {
        eprintln!("Usage: rlox [filename.lox].");
        process::exit(64);      // following UNIX "sysexits.h" header convention
    } else if args.len() == 2 {
        run_file(&args[0]);
    } else {
        run_prompt();
    }
}
