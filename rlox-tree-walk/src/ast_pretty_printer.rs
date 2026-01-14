use crate::token::Token;
use crate::token::TokenType::{self, *};
use crate::expr::{Expr, LiteralObject};

pub struct AstPrinter<'a> {
    source: &'a str,
    tokens: &'a Vec<Token>,
}

impl<'a> AstPrinter<'a> {
    pub fn new(source: &'a str, tokens: &'a Vec<Token>) -> AstPrinter<'a> {
        AstPrinter { source, tokens }
    }

    pub fn print(&self, expr: &Expr) {
        println!("{}", self.visit_expr(expr));
    }

    fn visit_expr(&self, expr: &Expr) -> String {
        match expr { 
            Expr::Binary { left, operator, right } => {
                let lexeme: String = self.get_lexeme(operator.start, operator.end);
                self.parenthesize(lexeme, &[left.as_ref(), right.as_ref()])
            },
            Expr::Grouping { expression } => {
                self.parenthesize(String::from("group"), &[expression.as_ref()])
            },
            Expr::Literal { value } => {
                match value {
                    LiteralObject::StringLiteral { start, end } | 
                    LiteralObject::IdentifierLiteral { start, end } | 
                    LiteralObject::NumberLiteral { start, end } => self.get_lexeme(*start, *end),

                    LiteralObject::BooleanLiteral { value } => value.to_string(),
                    LiteralObject::NilLiteral   => String::from("nil"),
                }
            },
            Expr::Unary { operator, right } => {
                let lexeme: String = self.get_lexeme(operator.start, operator.end);
                self.parenthesize(lexeme, &[right.as_ref()])
            },
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

    fn get_lexeme(&self, start: usize, end: usize) -> String {
        self.source[start..end].to_string()
    }
}