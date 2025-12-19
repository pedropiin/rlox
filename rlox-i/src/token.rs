use std::fmt;

use crate::token_type::TokenType;

pub struct Token {
    token_type: TokenType,
    start: usize, 
    end: usize,
    line: usize,
}

impl Token {
    pub fn new(token_type: TokenType, start: usize, end: usize, line: usize) -> Self {
        Token { token_type, start, end, line }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<{:?}, {}, {}, {}>", self.token_type, self.start, self.end, self.line)
    }
}