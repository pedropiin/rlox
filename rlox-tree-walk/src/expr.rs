use crate::token::Token;

#[derive(Debug, Clone)]
pub enum Expr {
    Assign {
        token: Token,
        value: Box<Expr>,
    },
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
    Logical {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
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
    BooleanLiteral {
        value: bool,
    },
    NilLiteral,
}