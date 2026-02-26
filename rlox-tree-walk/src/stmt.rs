use crate::expr::Expr;
use crate::token::Token;

#[derive(Clone, Debug)]
pub enum Stmt {
    Expression {
        expr: Box<Expr>,
    },
    If {
        condition: Box<Expr>,
        then_branch: Box<Stmt>,
        else_branch: Option<Box<Stmt>>,
    },
    Print {
        expr: Box<Expr>,
    },
    Var {
        token: Token,
        initializer: Option<Box<Expr>>,
    },
    While { 
        condition: Box<Expr>,
        body: Box<Stmt>,
    },
    Block {
        statements: Vec<Box<Stmt>>,
    },
    Break,
}