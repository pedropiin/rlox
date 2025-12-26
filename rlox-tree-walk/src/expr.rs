use crate::token::Token;

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
    LiteralValue {
        start: usize,
        end: usize,
    }
}

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
    }
}