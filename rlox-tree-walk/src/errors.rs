use crate::resolver::Resolver;
use crate::utils::{get_callable_kind};
use crate::interpreter::LiteralValue;

pub const MAX_ARGS: usize = 255;

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
    ResolverErr(ResolverError),
    RuntimeErr(RuntimeError),
}

impl LoxError {
    pub fn message(&self) -> String {
        match &self {
            LoxError::LexerErr(l) => l.message(),
            LoxError::ParserErr(p) => p.message(),
            LoxError::ResolverErr(res) => res.message(),
            LoxError::RuntimeErr(run) => run.message(),
        }
    }

    pub fn name(&self) -> String {
        match &self {
            LoxError::LexerErr(l) => l.name(),
            LoxError::ParserErr(p) => p.name(),
            LoxError::ResolverErr(res) => res.name(),
            LoxError::RuntimeErr(run) => run.name(),
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
    UnclosedParen,
    PrimaryExprExpected,
    SemicolonExpected,
    NamelessVarDeclaration,
    InvalidAssignment,
    RightBraceExpected,
    LeftParenControlFlowConditionExpected,
    RightParenControlFlowConditionExpected,
    ForConditionSemicolonExpected,
    BreakOutsideLoop,
    ContinueOutsideLoop,
    MissingParenArgumentList,
    TooManyArguments,
    CallableDefIdentifierExpected(u8),
    CallableDefMissingLeftParen(u8),
    CallableDefMissingRightParen,
    MissingParameterError,
    CallableDefMissingLeftBrace(u8),
    SemicolonExpetedAfterReturn,
}

impl ParserError {
    pub fn message(&self) -> String {
        match self {
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
            ParserError::LeftParenControlFlowConditionExpected
                => "Expected '(' after 'if' token.".to_string(),
            ParserError::RightParenControlFlowConditionExpected
                => "Expected ')' after if condition.".to_string(),
            ParserError::ForConditionSemicolonExpected
                => "Expected ';' after 'for' loop condition.".to_string(),
            ParserError::BreakOutsideLoop
                => "Break statement outside of any enclosing loop.".to_string(),
            ParserError::ContinueOutsideLoop
                => "Continue statement outside of any enclosing loop.".to_string(),
            ParserError::MissingParenArgumentList
                => "Expected ')' after argument list.".to_string(),
            ParserError::TooManyArguments 
                => format!("Can't have more than {} arguments.", MAX_ARGS),
            ParserError::CallableDefIdentifierExpected(mode) 
                => format!("Expected {} name.", get_callable_kind(*mode)),
            ParserError::CallableDefMissingLeftParen(mode)
                => format!("Expected '(' after {} name.", get_callable_kind(*mode)),
            ParserError::CallableDefMissingRightParen
                => "Expected ')' after parameter names.".to_string(),
            ParserError::MissingParameterError 
                => "Expected parameter name.".to_string(),
            ParserError::CallableDefMissingLeftBrace(mode)
                => format!("Expected '{{' before {} body.", get_callable_kind(*mode)),
            ParserError::SemicolonExpetedAfterReturn
                => "Expected ';' after return value.".to_string(),
        }
    }

    pub fn name(&self) -> String {
        match self {
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
                => "RightBraceExpected".to_string(),
            ParserError::LeftParenControlFlowConditionExpected
                => "LeftParenControlFlowConditionExpected".to_string(),
            ParserError::RightParenControlFlowConditionExpected
                => "RightParenControlFlowConditionExpected".to_string(),
            ParserError::ForConditionSemicolonExpected
                => "ForConditionSemicolonExpected".to_string(),
            ParserError::BreakOutsideLoop
                => "BreakOutsideLoop".to_string(),
            ParserError::ContinueOutsideLoop
                => "ContinueOutsideLoop".to_string(),
            ParserError::MissingParenArgumentList
                => "MissingParenArgumentList".to_string(),
            ParserError::TooManyArguments 
                => "TooManyArguments".to_string(),
            ParserError::CallableDefIdentifierExpected(_) 
                => "CallableDefIdentifierExpected".to_string(),
            ParserError::CallableDefMissingLeftParen(_)
                => "CallableDefMissingLeftParen".to_string(),
            ParserError::CallableDefMissingRightParen 
                => "CallableDefMissingRightParen".to_string(),
            ParserError::MissingParameterError 
                => "MissingParameterError".to_string(),
            ParserError::CallableDefMissingLeftBrace(_)
                => "CallableDefMissingLeftBrace".to_string(),
            ParserError::SemicolonExpetedAfterReturn
                => "SemicolonExpetedAfterReturn".to_string(),
        }
    }
}

impl From<ParserError> for LoxError {
    fn from(err: ParserError) -> Self {
        LoxError::ParserErr(err)
    }
}

pub struct ParserErrTup(pub usize, pub ParserError);


// All resolver errors that may be evaluated during variable resolving and binding
pub enum ResolverError {
    VarReadOwnInitializer,
    VarRedeclarationInScope(String),
    ReturnStmtOutisdeFunc,
}
impl ResolverError {
    pub fn message(&self) -> String {
        match self {
            ResolverError::VarReadOwnInitializer
                => "Can't read local variable in its own initializer,".to_string(),
            ResolverError::VarRedeclarationInScope(s)
                => "Already a variable with name '{}' in this scope.".to_string(),
            ResolverError::ReturnStmtOutisdeFunc
                => "Can't return from top-level code.".to_string(),
        }
    }

    pub fn name(&self) -> String { 
        match self {
            ResolverError::VarReadOwnInitializer
                => "VarReadOwnInitializer".to_string(),
            ResolverError::VarRedeclarationInScope(_)
                => "VarRedeclarationInScope".to_string(),
            ResolverError::ReturnStmtOutisdeFunc
                => "ReturnStmtOutisdeFunc".to_string(),
        }
    }
}

impl From<ResolverError> for LoxError {
    fn from(err: ResolverError) -> Self {
        LoxError::ResolverErr(err)
    }
}

pub struct ResolverErrTup(pub usize, pub ResolverError);


// All runtime errors that may be evaluated during AST interpretation
pub enum RuntimeError {
    InvalidUnaryOperandError,
    InvalidBinaryOperandsError,
    InvalidSumOperandsError,
    DivisionByZeroError,
    UndefinedVariableError(String),
    UninitializedVariableError,
    BreakStmtException,
    ContinueStmtException,
    NonCallableValueError,
    FunctionNotComparableError,
    CallParityError(usize, usize),
    ClockCallError,
    ReturnStmtException(LiteralValue),
    InvalidResolverDistance,
    InvalidResolverExpr,
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
                => format!("Undefined variable '{var_name}'."),
            RuntimeError::UninitializedVariableError
                => "Use of uninitialized variable.".to_string(),
            RuntimeError::BreakStmtException
                => "Error used as exception to properly deal with a 'break' statement. Not to be thrown at user. ".to_string(),
            RuntimeError::ContinueStmtException
                => "Error used as exception to properly deal with a 'continue' statement. Not to be thrown at user. ".to_string(),
            RuntimeError::NonCallableValueError
                => "Can only call functions and classes.".to_string(),
            RuntimeError::FunctionNotComparableError
                => "Cannot compare two callable values.".to_string(),
            RuntimeError::CallParityError(og_parity, found_parity)
                => format!("Expected {} arguments, but got {} in call.", og_parity, found_parity),
            RuntimeError::ClockCallError
                => "Non-monotonic clock drift caused interal 'clock' duration to be negative.".to_string(),
            RuntimeError::ReturnStmtException(_)
                => "Error used as exception to properly deal with a 'return' statement. Not to be thrown at user.".to_string(),
            RuntimeError::InvalidResolverDistance
                => "Resolver assigned an out of bounds index to expression. Not to be thrown at user.".to_string(),
            RuntimeError::InvalidResolverExpr
                => "Resolver scope map doesn't contain the given expression. Not to be thrown at user.".to_string(),
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
            RuntimeError::UninitializedVariableError
                => "UninitializedVariableError".to_string(),
            RuntimeError::BreakStmtException
                => "BreakStmtException".to_string(),
            RuntimeError::ContinueStmtException
                => "ContinueStmtException".to_string(),
            RuntimeError::NonCallableValueError
                => "NonCallableValue".to_string(),
            RuntimeError::FunctionNotComparableError
                => "FunctionNotComparableError".to_string(),
            RuntimeError::CallParityError(_, __)
                => "CallParityError".to_string(),
            RuntimeError::ClockCallError
                => "ClockCallError".to_string(),
            RuntimeError::ReturnStmtException(_)
                => "ReturnStmtException".to_string(),
            RuntimeError::InvalidResolverDistance
                => "InvalidResolverDistance".to_string(),
            RuntimeError::InvalidResolverExpr
                => "InvalidResolverExpr".to_string(),
        }
    }
}

impl From<RuntimeError> for LoxError {
    fn from(err: RuntimeError) -> Self {
        LoxError::RuntimeErr(err)
    }
}

pub struct RuntimeErrTup(pub usize, pub RuntimeError);