use crate::token::Token;
use crate::expr::{Expr, LiteralObject};
use crate::stmt::Stmt;

pub struct AstPrinter<'a> {
    source: &'a str,
    tokens: &'a Vec<Token>,
}

impl<'a> AstPrinter<'a> {
    pub fn new(source: &'a str, tokens: &'a Vec<Token>) -> AstPrinter<'a> {
        AstPrinter { source, tokens }
    }

    pub fn print(&self, stmts: &Vec<Box<Stmt>>) -> () {
        for stmt in stmts {
            match stmt.as_ref() {
                Stmt::Expression { expr } | Stmt::Print { expr }
                    => println!("{}", self.visit_expr(expr)),
                Stmt::Var { token, initializer } => {
                    let init: String = match initializer {
                        Some(expr) => self.visit_expr(expr),
                        None => "nil".to_string(),
                    };
                    println!("{} -> {}", token.lexeme, init);
                },
                _ => todo!(),
            }
        }
    }

    fn visit_expr(&self, expr: &Expr) -> String {
        match expr { 
            Expr::Binary { left, operator, right } => {
                let lexeme: String = (*operator.lexeme).clone();
                self.parenthesize(lexeme, &[left.as_ref(), right.as_ref()])
            },
            Expr::Grouping { expression } => {
                self.parenthesize(String::from("group"), &[expression.as_ref()])
            },
            Expr::Literal { value } => {
                match value {
                    LiteralObject::StringLiteral { lexeme } | 
                    LiteralObject::NumberLiteral { lexeme } => (**lexeme).clone(),

                    LiteralObject::BooleanLiteral { value } => value.to_string(),
                    LiteralObject::NilLiteral   => String::from("nil"),
                }
            },
            Expr::Unary { operator, right } => {
                let lexeme: String = (*operator.lexeme).clone();
                self.parenthesize(lexeme, &[right.as_ref()])
            },
            _ => todo!(),
        }
    }

    fn parenthesize(&self, name: String, exprs: &[&Expr]) -> String {
        let mut builder: String = String::from("(");
        builder.push_str(&name);
        for expr in exprs {
            builder.push_str(" ");
            builder.push_str(&self.visit_expr(expr));
        }
        builder.push_str(")");
        builder
    }
}