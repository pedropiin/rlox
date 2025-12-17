use std::env;
use std::process;
use std::fs;
use std::io;
use std::fmt;

#[derive(Debug)]
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

struct Token<'b> {
    token_type: TokenType,
    lexeme: &'b str,
    line: usize,
}

impl<'b> Token<'b> {
    fn new(token_type: TokenType, lexeme: &'b str, line: usize) -> Self {
        Token { token_type, lexeme, line }
    }
}

impl<'b> fmt::Display for Token<'b> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<{:?}, {}, {}>", self.token_type, self.lexeme, self.line)
    }
}

struct Scanner<'a> {
    source: &'a str,
    tokens: Vec<Token<'a>>,
}

impl<'a> Scanner<'a> {
    fn new(source: &'a str) -> Self {
        let mut tokens: Vec<Token> = Vec::new();
        Scanner { source, tokens }
    }

    fn scan_tokens(&self) -> Vec<Token> {
        todo!();
    }
}

fn run(source: &str) {
    let scanner: Scanner = Scanner::new(source);
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
