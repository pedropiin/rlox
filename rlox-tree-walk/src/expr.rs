use std::rc::Rc;

use crate::stmt::Stmt;
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
    Call {
        callee: Box<Expr>,
        paren: Token,
        args: Vec<Box<Expr>>,
    },
    Grouping {
        expression: Box<Expr>,
    },
    Lambda {
        params: Vec<Token>,
        body: Vec<Box<Stmt>>,
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

#[derive(Debug, Clone)]
pub enum LiteralObject {
    BooleanLiteral {
        value: bool,
    },
    NilLiteral,
    NumberLiteral {
        lexeme: Rc<String>,
    },
    StringLiteral {
        lexeme: Rc<String>,
    },
}