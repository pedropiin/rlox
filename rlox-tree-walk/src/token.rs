use std::fmt;

#[derive(Clone, Copy)]
pub struct Token {
    pub token_type: TokenType,
    pub start: usize, 
    pub end: usize,
    pub line: usize,
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

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TokenType {
    // Single-character tokens
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
    
    // One or two character tokens
    Bang, 
    BangEqual, 
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals
    Identifier,
    Str,
    Number,

    // Keywords
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