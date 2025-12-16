use std::env;
use std::process;
use std::fs;
use std::io;
use std::fmt;

enum TokenType {
    LeftParen,
    RightParen, 
    LeftBrace, 
    RightBrace, 
    Comma, 
    Dot, 
    Minus, 
    Plus, 
    Semicolon, 
    Slash, 
    Star,
    Bang, 
    BangEqual, 
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    Identifier,
    String,
    Number,
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,
    Eof,
}

enum LexerErrors {
    UsageError,
    SourceReadError,
}

impl LexerErrors {
    fn message(&self) -> &str {
        match self {
            LexerErrors::UsageError     => "Usage: rlox [filename.lox].",
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

struct Token {

}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // write!(f, "{}", self.content)
        todo!();
    }
}

struct Scanner<'a> {
    source: &'a str,
}

impl Scanner<'_> {
    fn scan_tokens(&self) -> Vec<Token> {
        todo!();
    }
}

fn run(source: &str) {
    let scanner: Scanner = Scanner { source: source };
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
        eprintln!("{}", LexerErrors::UsageError.message());
        process::exit(64);      // following UNIX "sysexits.h" header convention
    } else if args.len() == 1 {
        run_file(&args[0]);
    } else {
        run_prompt();
    }
}
