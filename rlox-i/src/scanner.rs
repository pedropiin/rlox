use crate::token::Token;
use crate::token_type::TokenType;

pub struct Scanner<'a> {
    source: &'a str,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Self {
        let mut tokens: Vec<Token> = Vec::new();
        Scanner { source, tokens, start: 0, current: 0, line: 1 }
    }

    pub fn scan_tokens(&mut self) -> Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push(Token::new(TokenType::Eof, 0, 0, self.line));
        self.tokens
    }

    fn scan_token(&self) -> () {
        todo!();
    }

    fn is_at_end(&self) -> bool {
        todo!();
    }
}