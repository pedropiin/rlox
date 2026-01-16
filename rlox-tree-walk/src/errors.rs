// Main error handling method
pub fn lox_error(line: usize, error_type: LoxError) {
    fn report(line: usize, locale: &str, error_type: LoxError) {
        eprintln!("[line {}] Error {}: {}", line, locale, error_type.message());
    }
    report(line, "", error_type);
}

// "Super-struct" for all possible errors when interpreting Lox
pub enum LoxError {
    LexerErr(LexerError),
    ParserErr(ParserError),
    RuntimeErr(RuntimeError),
}

impl LoxError {
    pub fn message(&self) -> String {
        match &self {
            LoxError::LexerErr(l) => l.message(),
            LoxError::ParserErr(p) => p.message(),
            LoxError::RuntimeErr(r) => r.message(),
        }
    }
}

// All lexer/tokenizer errors
pub enum LexerError {
    InvalidChar(char),
    UnterminatedString,
}

impl LexerError {
    pub fn message(&self) -> String {
        match self {
            LexerError::InvalidChar(c) 
                => format!("Unexpected character '{}'.", c),
            LexerError::UnterminatedString 
                => "Unterminated string.".to_string(),
        }
    }
}

impl From<LexerError> for LoxError {
    fn from(err: LexerError) -> Self {
        LoxError::LexerErr(err)
    }
}

// All parser errors
pub enum ParserError {
    TokenPeekError,
    UnclosedParen,
    PrimaryExprExpected,
}

impl ParserError {
    pub fn message(&self) -> String {
        match self {
            ParserError::TokenPeekError 
                => "Error when trying to get token from internal Vec<Token>.".to_string(),
            ParserError::UnclosedParen  
                => "Expected ')' after expression.".to_string(),
            ParserError::PrimaryExprExpected 
                => "Expected primary expression.".to_string(),
        }
    }
}

impl From<ParserError> for LoxError {
    fn from(err: ParserError) -> Self {
        LoxError::ParserErr(err)
    }
}

// All runtime errors that may be evaluated during AST interpretation
pub enum RuntimeError {
    InvalidUnaryOperandError,
    InvalidBinaryOperandsError,
    InvalidSumOperandsError,
}

impl RuntimeError {
    pub fn message(&self) -> String {
        match self {
            RuntimeError::InvalidUnaryOperandError 
                => "Operand must be a number".to_string(),
            RuntimeError::InvalidBinaryOperandsError 
                => "Both operands must be a number".to_string(),
            RuntimeError::InvalidSumOperandsError 
                => "Both operands must be either a number or a string".to_string(),
        }
    }
}

impl From<RuntimeError> for LoxError {
    fn from(err: RuntimeError) -> Self {
        LoxError::RuntimeErr(err)
    }
}