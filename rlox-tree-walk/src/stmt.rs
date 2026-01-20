use crate::expr::Expr;
use crate::token::Token;

#[derive(Clone)]
pub enum Stmt {
    Expression {
        expr: Box<Expr>,
    },
    Print {
        expr: Box<Expr>,
    },
    Var {
        token: Token,
        initializer: Option<Box<Expr>>,
    },
    Block {
        statements: Vec<Box<Stmt>>,
    },
}