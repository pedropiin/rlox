use crate::token::Token;

#[derive(Debug, Clone)]
pub enum Expr {
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Grouping {
        expression: Box<Expr>,
    },
    Literal {
        value: LiteralObject,
    },
    Unary {
        operator: Token,
        right: Box<Expr>,
    },
    Variable {
        token: Token,
    },
}

#[derive(Debug, Clone, Copy)]
pub enum LiteralObject {
    StringLiteral {
        start: usize,
        end: usize,
    },
    NumberLiteral {
        start: usize, 
        end: usize,
    },
    IdentifierLiteral {
        start: usize,
        end: usize,
    },
    BooleanLiteral {
        value: bool,
    },
    NilLiteral,
}