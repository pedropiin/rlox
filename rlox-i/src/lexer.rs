use crate::token::Token;
use crate::token_type::TokenType::{self, *};
use crate::error;

pub enum LexerErrors {
    SourceReadError,
    InvalidChar(char),
    UnterminatedString,
}

impl LexerErrors {
    pub fn message(&self) -> String {
        match self {
            LexerErrors::SourceReadError  => "Could not read/open source lox file.".to_string(),
            LexerErrors::InvalidChar(c)    => format!("Unexpected character '{}'.", c),
            LexerErrors::UnterminatedString => "Unterminated string.".to_string(),
        }
    }
}

pub struct Lexer<'a> {
    source: &'a str,
    tokens: &'a mut Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
    had_error: bool,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str, tokens: &'a mut Vec<Token>) -> Self {
        Lexer { source, tokens, start: 0, current: 0, line: 1, had_error: false}
    }

    pub fn scan_tokens(&mut self) -> bool {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push(Token::new(Eof, 0, 0, self.line));
        self.had_error
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn scan_token(&mut self) {
        let chr: char = self.advance();
        match chr {
            // Single-character tokens
            '(' => self.add_token(LeftParen),
            ')' => self.add_token(RightParen),
            '{' => self.add_token(LeftBrace),
            '}' => self.add_token(RightBrace),
            ',' => self.add_token(Comma),
            '.' => self.add_token(Dot),
            '-' => self.add_token(Minus),
            '+' => self.add_token(Plus),
            ';' => self.add_token(Semicolon),
            '*' => self.add_token(Star),

            // Ambiguous single-character tokens
            '!' => {
                let is_bang_equal = self.advance_on_match('=');
                self.add_token(if is_bang_equal { BangEqual } else { Bang });
            }
            '=' => {
                let is_equal_equal = self.advance_on_match('=');
                self.add_token(if is_equal_equal { EqualEqual } else { Equal });
            }
            '<' => {
                let is_less_equal = self.advance_on_match('=');
                self.add_token(if is_less_equal { LessEqual } else { Less });
            }
            '>' => {
                let is_greater_equal = self.advance_on_match('=');
                self.add_token(if is_greater_equal { GreaterEqual } else { Greater });
            }
            '/' => {
                let is_comment = self.advance_on_match('/');
                if is_comment {
                    while self.peek() != '\n' && !self.is_at_end() { 
                        self.advance(); 
                    }
                } else { self.add_token(Slash); }
            }

            // Whitespace
            ' ' | '\r' | '\t' => (),
            '\n' => self.line += 1,

            // String literals
            '"' => self.string(),

            // Invalid characters
            c   => {
                error(self.line, LexerErrors::InvalidChar(c));
                self.had_error = true
            }
        }
    }

    fn string(&mut self) -> () {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' { self.line += 1 }
            self.advance();
        }

        if self.is_at_end() {
            error(self.line, LexerErrors::UnterminatedString);
            return;
        }

        self.advance(); // consume closing "
        self.add_token(Str);
    }

    fn peek(&self) -> char {
        if self.is_at_end() { '\0' }
        else { self.source.as_bytes()[self.current] as char }
    }

    fn advance(&mut self) -> char {
        let c: char = self.source.as_bytes()[self.current] as char;
        self.current += 1;
        c
    }

    fn advance_on_match(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.source.as_bytes()[self.current] as char != expected {
            return false;
        }
        self.current += 1;
        true
    }

    fn add_token(&mut self, token_type: TokenType) -> () {
        match token_type {
            Str => self.tokens.push(Token::new(token_type, self.start + 1, self.current - 1, self.line)),
            _   => self.tokens.push(Token::new(token_type, self.start, self.current, self.line)),
        }
    }
}