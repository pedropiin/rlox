use crate::expr::Expr;
use crate::token::Token;

#[derive(Clone, Debug)]
pub enum Stmt {
    Block {
        statements: Vec<Box<Stmt>>,
    },
    Break,
    Continue,
    Expression {
        expr: Box<Expr>,
    },
    Function {
        name: Token,
        params: Vec<Token>,
        body: Vec<Box<Stmt>>,
    },
    If {
        condition: Box<Expr>,
        then_branch: Box<Stmt>,
        else_branch: Option<Box<Stmt>>,
    },
    Print {
        expr: Box<Expr>,
    },
    Return {
        keyword: Token,
        value: Option<Box<Expr>>,
    },
    Var {
        token: Token,
        initializer: Option<Box<Expr>>,
    },
    While { 
        condition: Box<Expr>,
        body: Box<Stmt>,
    },
}