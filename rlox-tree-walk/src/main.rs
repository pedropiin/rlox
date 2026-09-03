use std::cell::Ref;
use std::env;
use std::fs;
use std::io::{self, Write};
use std::process;
use std::rc::Rc;
use std::cell::RefCell;

use crate::ast_pretty_printer::AstPrinter;
use crate::interpreter::Interpreter;
use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::resolver::Resolver;
use crate::stmt::Stmt;
use crate::token::Token;

mod errors;
mod expr;
mod lexer;
mod lox_callable;
mod parser;
mod resolver;
mod stmt;
mod token;
mod utils;
#[allow(warnings)]
mod ast_pretty_printer;
mod interpreter;
mod native_functions;

fn run<'a>(source: &'a str, interpreter: Rc<RefCell<Interpreter>>) -> bool {
    let mut tokens: Vec<Token> = Vec::new();
    let mut scanner: Lexer = Lexer::new(&source, &mut tokens);
    let scanning_error: bool = scanner.scan_tokens(); 

    let mut stmts: Vec<Box<Stmt>> = Vec::new();
    let mut parser: Parser = Parser::new(&mut tokens, &mut stmts);
    let parsing_error: bool = parser.parse();

    if scanning_error || parsing_error { return true }

    let mut resolver: Resolver = Resolver::new(interpreter.clone());
    let resolver_error: bool = resolver.resolve(&stmts);

    if resolver_error { return true }

    let runtime_error: bool = interpreter.borrow_mut().interpret(&stmts);

    if runtime_error { return true }

    // let ast_printer: AstPrinter = AstPrinter::new(source, &tokens);
    // ast_printer.print(&stmts);

    false
}

fn run_file(path: &String) {
    let contents: String = fs::read_to_string(&path).expect("Could not read/open source lox file.");
    let interpreter: Rc<RefCell<Interpreter>> = Rc::new(RefCell::new((Interpreter::new(false))));
    let had_error: bool = run(&contents, interpreter);

    if had_error {
        process::exit(65);
    }
}

fn run_prompt() {
    let interpreter: Rc<RefCell<Interpreter>> = Rc::new(RefCell::new(Interpreter::new(true)));
    // let mut interpreter: Interpreter = Interpreter::new(true);
    let mut input_buf = String::from("");
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input_buf).unwrap();
        run(&input_buf, interpreter.clone()); // !!! One new Rc every iteration... probably not the most optimal approach
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
