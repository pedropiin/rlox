use std::env;
use std::process;
use std::fs;
use std::io::{self, Write};

use crate::token::Token;
use crate::lexer::Lexer;
use crate::parser::Parser;

mod token;
mod lexer;
mod expr;
mod parser;
mod errors;

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
    let contents: String = fs::read_to_string(&path).expect("Could not read/open source lox file.");
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
        run_file(&args[1]);
    } else {
        run_prompt();
    }
}
