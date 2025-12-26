pub fn lox_error(line: usize, error_type: LoxError) {
    fn report(line: usize, locale: &str, error_type: LoxError) {
        eprintln!("[line {}] Error {}: {}", line, locale, error_type.message());
    }

    report(line, "", error_type);
}

pub enum LoxError {
    LexerErr(LexerError),
    ParserErr(ParserError),
}

impl LoxError {
    pub fn message(&self) -> String {
        match &self {
            LoxError::LexerErr(l) => l.message(),
            LoxError::ParserErr(p) => p.message(),
        }
    }
}

impl From<LexerError> for LoxError {
    fn from(err: LexerError) -> Self {
        LoxError::LexerErr(err)
    }
}

impl From<ParserError> for LoxError {
    fn from(err: ParserError) -> Self {
        LoxError::ParserErr(err)
    }
}

pub enum LexerError {
    InvalidChar(char),
    UnterminatedString,
}

impl LexerError {
    pub fn message(&self) -> String {
        match self {
            LexerError::InvalidChar(c)    => format!("Unexpected character '{}'.", c),
            LexerError::UnterminatedString => "Unterminated string.".to_string(),
        }
    }
}

pub enum ParserError {
    TokenPeekError,
    UnclosedParen,
    PrimaryExprExpected,
}

impl ParserError {
    pub fn message(&self) -> String {
        match self {
            ParserError::TokenPeekError => "Error when trying to get token from internal Vec<Token>.".to_string(),
            ParserError::UnclosedParen  => "Expected ')' after expression.".to_string(),
            ParserError::PrimaryExprExpected => "Expected primary expression.".to_string(),
        }
    }
}