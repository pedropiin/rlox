use std::env;
use std::process;
use std::fs;
use std::io::{self, Write};

use crate::token::Token;
use crate::lexer::Lexer;
use crate::stmt::Stmt;
use crate::parser::Parser;
use crate::ast_pretty_printer::AstPrinter;
use crate::interpreter::Interpreter;

mod token;
mod lexer;
mod expr;
mod stmt;
mod parser;
mod errors;
mod utils;
mod lox_callable;
#[allow(warnings)]
mod ast_pretty_printer;
mod interpreter;
mod native_functions;

fn run<'a>(source: &'a str, interpreter: &'a mut Interpreter) -> bool {
    let mut tokens: Vec<Token> = Vec::new();
    let mut scanner: Lexer = Lexer::new(&source, &mut tokens);
    let scanning_error: bool = scanner.scan_tokens(); 

    let mut stmts: Vec<Box<Stmt>> = Vec::new();
    let mut parser: Parser = Parser::new(&mut tokens, &mut stmts);
    let parsing_error: bool = parser.parse();

    if scanning_error || parsing_error { return true }

    let had_runtime_error: bool = interpreter.interpret(&stmts, source);

    if had_runtime_error { return true }

    // let ast_printer: AstPrinter = AstPrinter::new(source, &tokens);
    // ast_printer.print(&stmts);

    false
}

fn run_file(path: &String) {
    let contents: String = fs::read_to_string(&path).expect("Could not read/open source lox file.");
    let mut interpreter: Interpreter = Interpreter::new(false);
    let had_error: bool = run(&contents, &mut interpreter);

    if had_error {
        process::exit(65);
    }
}

fn run_prompt() {
    let mut interpreter: Interpreter = Interpreter::new(true);
    let mut input_buf = String::from("");
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input_buf).unwrap();
        run(&input_buf, &mut interpreter);
        input_buf.clear();
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
