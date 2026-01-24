use crate::parser::Parser;

// Main error handling method
pub fn lox_error(line: usize, error_type: LoxError) {
    fn report(line: usize, locale: &str, error_type: LoxError) {
        eprintln!("[line {}] {}{}: {}", line, error_type.name(), locale, error_type.message());
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

    pub fn name(&self) -> String {
        match &self {
            LoxError::LexerErr(l) => l.name(),
            LoxError::ParserErr(p) => p.name(),
            LoxError::RuntimeErr(r) => r.name(),
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

    pub fn name(&self) -> String {
        match self {
            LexerError::InvalidChar(_) 
                => "InvalidCharError".to_string(),
            LexerError::UnterminatedString
                => "UnterminatedStringError".to_string(),
        }
    }
}

impl From<LexerError> for LoxError {
    fn from(err: LexerError) -> Self {
        LoxError::LexerErr(err)
    }
}

// All parser errors
#[derive(Clone)]
pub enum ParserError {
    TokenPeekError,
    UnclosedParen,
    PrimaryExprExpected,
    SemicolonExpected,
    NamelessVarDeclaration,
    InvalidAssignment,
    RightBraceExpected,
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
            ParserError::SemicolonExpected
                => "Expected ';' after expression.".to_string(),
            ParserError::NamelessVarDeclaration
                => "Expected a variable name.".to_string(),
            ParserError::InvalidAssignment
                => "Invalid assignment target.".to_string(),
            ParserError::RightBraceExpected
                => "Expected '}' after block".to_string(),
        }
    }

    pub fn name(&self) -> String {
        match self {
            ParserError::TokenPeekError
                => "TokenPeekError".to_string(),
            ParserError::UnclosedParen
                => "UnclosedParen".to_string(),
            ParserError::PrimaryExprExpected
                => "PrimaryExprExpected".to_string(),
            ParserError::SemicolonExpected
                => "SemicolonExpected".to_string(),
            ParserError::NamelessVarDeclaration
                => "NamelessVarDeclaration".to_string(),
            ParserError::InvalidAssignment
                => "InvalidAssignment".to_string(),
            ParserError::RightBraceExpected
                => "RightBraceExpected".to_string()
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
    DivisionByZeroError,
    UndefinedVariableError(String),
}

impl RuntimeError {
    pub fn message(&self) -> String {
        match self {
            RuntimeError::InvalidUnaryOperandError 
                => "Operand must be a number.".to_string(),
            RuntimeError::InvalidBinaryOperandsError 
                => "Both operands must be a number.".to_string(),
            RuntimeError::InvalidSumOperandsError 
                => "Both operands must be either a number or a string.".to_string(),
            RuntimeError::DivisionByZeroError
                => "Cannot divide by zero.".to_string(),
            RuntimeError::UndefinedVariableError(var_name)
                => format!("Undefined variable '{var_name}'.")
        }
    }

    pub fn name(&self) -> String {
        match self {
            RuntimeError::InvalidUnaryOperandError
                => "InvalidUnaryOperandError".to_string(),
            RuntimeError::InvalidBinaryOperandsError
                => "InvalidBinaryOperandsError".to_string(),
            RuntimeError::InvalidSumOperandsError
                => "InvalidSumOperandsError".to_string(),
            RuntimeError::DivisionByZeroError
                => "DivisionByZeroError".to_string(),
            RuntimeError::UndefinedVariableError(_)
                => "UndefinedVariableError".to_string(),
        }
    }
}

impl From<RuntimeError> for LoxError {
    fn from(err: RuntimeError) -> Self {
        LoxError::RuntimeErr(err)
    }
}