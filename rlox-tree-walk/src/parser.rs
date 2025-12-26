use crate::token::Token;
use crate::token::TokenType::{self, *};
use crate::expr::Expr;

pub struct Parser<'a> {
    source: &'a str,
    tokens: &'a mut Vec<Token>,
    current: usize,
}

impl<'a> Parser<'a> {
    pub fn new(&mut self, source: &'a str, tokens: &'a mut Vec<Token>) -> Parser<'_> {
        Parser { source, tokens, current: 0 }
    }

    fn expression(&mut self) -> Expr {
        self.equality()
    }

    fn equality(&mut self) -> Expr {
        let mut expr: Box<Expr> = Box::new(self.comparison());
        
        let tok_types: Vec<TokenType> = vec![BangEqual, EqualEqual];
        while self.match_consume(&tok_types) {
            let op: Token = *self.previous();
            let rhs: Box<Expr> = Box::new(self.comparison());
            expr = Box::new(Expr::Binary { left: expr, operator: op, right: rhs });
        }

        return *expr
    }

    fn comparison(&self) -> Expr {
        todo!();
    }

    fn match_consume(&mut self, tokens: &Vec<TokenType>) -> bool {
        for token_type in tokens {
            if self.check(token_type) {
                self.advance();
                return true
            }
        }

        false
    }

    fn check(&self, token_type: &TokenType) -> bool {
        if !self.is_at_end() { return false }
        self.peek().token_type == *token_type
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() { self.current += 1 }
        return self.previous()

    }

    fn is_at_end(&self) -> bool {
        self.peek().token_type == Eof
    }

    fn peek(&self) -> &Token {
        self.tokens.get(self.current).unwrap_or_else(|| panic!("Could not get the {}th token.", self.current))
    }
    
    fn previous(&self) -> &Token {
        self.tokens.get(self.current - 1).unwrap_or_else(|| panic!("Could not get the {}th token.", self.current-1))
    }
}