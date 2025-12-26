use crate::token::Token;
use crate::token::TokenType::{self, *};
use crate::errors::{LexerError, error};



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
        Lexer { source, tokens, start: 0, current: 0, line: 1, had_error: false }
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
                let is_multiline_comment = self.advance_on_match('*');
                if is_comment {
                    while self.peek() != '\n' && !self.is_at_end() { 
                        self.advance(); 
                    }
                } else if is_multiline_comment {
                    loop {
                        if !self.is_at_end() && self.peek() == '*' {
                            self.advance();
                            if !self.is_at_end() && self.peek() == '/' { self.advance(); break; }
                        } 
                        self.advance();
                    }
                } else { self.add_token(Slash); }
            }

            // Whitespace
            ' ' | '\r' | '\t' => (),
            '\n' => self.line += 1,

            // String literals
            '"' => self.string(),

            // Number literals
            c if c.is_ascii_digit() => self.number(),

            // Identifiers and keywords
            c if self.is_alpha(c) => self.identifier(),

            // Invalid characters
            c   => {
                error(self.line, LexerError::InvalidChar(c).into());
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
            error(self.line, LexerError::UnterminatedString.into());
            return;
        }

        self.advance(); // consume closing "
        self.add_token(Str);
    }

    fn number(&mut self) -> () {
        while self.peek().is_ascii_digit() {
            self.advance();
        }

        if self.peek() == '.' && self.peek_next().is_ascii_digit() {
            self.advance();
            while self.peek().is_ascii_digit() {
                self.advance();
            }
        }

        self.add_token(Number);
    }

    fn identifier(&mut self) -> () {
        while self.is_alpha_numeric(self.peek()) {
            self.advance();
        }

        let token_type: TokenType = match &self.source[self.start..self.current] {
            "and"      => And,
            "class"    => Class,
            "else"     => Else,
            "false"    => False,
            "for"      => For,
            "fun"      => Fun,
            "if"       => If,
            "nil"      => Nil,
            "or"       => Or,
            "print"    => Print,
            "return"   => Return,
            "super"    => Super,
            "this"     => This,
            "true"     => True,
            "var"      => Var,
            "while"    => While,
            _          => Identifier,
        };

        self.add_token(token_type);
    }

    fn peek(&self) -> char {
        if self.is_at_end() { '\0' }
        else { self.source.as_bytes()[self.current] as char }
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() { '\0' }
        else { self.source.as_bytes()[self.current + 1] as char }
    }

    fn is_alpha(&self, c: char) -> bool {
        (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || (c == '_')
    }

    fn is_alpha_numeric(&self, c: char) -> bool {
        self.is_alpha(c) || c.is_ascii_digit()
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