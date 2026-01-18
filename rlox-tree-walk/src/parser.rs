use core::panic;

use crate::token::{self, Token};
use crate::token::TokenType::{self, *};
use crate::expr::Expr;
use crate::stmt::Stmt;
use crate::expr::LiteralObject::{self, *};
use crate::errors::{ParserError, lox_error};

struct ParserErrTup(usize, ParserError);

pub struct Parser<'a> {
    source: &'a str,
    tokens: &'a mut Vec<Token>,
    current: usize,
}

impl<'a> Parser<'a> {
    pub fn new(source: &'a str, tokens: &'a mut Vec<Token>) -> Parser<'a> {
        Parser { source, tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Vec<Box<Stmt>> {
        let mut stmts: Vec<Box<Stmt>> = vec![];
        while !self.is_at_end() {
            if let Some(stmt) = self.declaration(){
                stmts.push(stmt);
            }
        }
        stmts
    }

    fn declaration(&mut self) -> Option<Box<Stmt>> {
        todo!("Deal with syntax and parser errors, that may be returned/found");
        let result = 
        if self.match_token(&[Var]) {
            self.var_declaration()
        } else { self.statement() };

        match result {
            Ok(stmt) => Some(stmt),
            Err(_) => {
                self.synchronize();
                None
            }
        }
    }

    fn var_declaration(&mut self) -> Result<Box<Stmt>, ParserErrTup> {
        let name: Token = self.consume(Identifier, ParserError::NamelessVarDeclaration)?;

        let mut initializer: Option<Box<Expr>> = None;
        if self.match_token(&[Equal]) { initializer = Some(self.expression()) }

        self.consume(Semicolon, ParserError::SemicolonExpected)?;
        Ok(Box::new(Stmt::Var { name: name, initializer: initializer }))
    }

    fn statement(&mut self) -> Result<Box<Stmt>, ParserErrTup> {
        if self.match_token(&[Print]) {
            return self.print_statement()
        } 
        self.expr_statement()
    }

    fn expr_statement(&mut self) -> Result<Box<Stmt>, ParserErrTup> {
        let expr: Box<Expr> = self.expression();
        self.consume(Semicolon, ParserError::SemicolonExpected)?;

        Ok(Box::new(Stmt::Expression { expr: expr }))
    }

    fn print_statement(&mut self) -> Result<Box<Stmt>, ParserErrTup> {
        let expr: Box<Expr> = self.expression();
        self.consume(Semicolon, ParserError::SemicolonExpected)?;
        
        Ok(Box::new(Stmt::Print { expr: expr }))
    }

    fn expression(&mut self) -> Box<Expr> {
        self.equality()
    }

    fn equality(&mut self) -> Box<Expr> {
        let mut expr: Box<Expr> = self.comparison();
        
        while self.match_token(&[BangEqual, EqualEqual]) {
            let op: Token = self.previous();
            let rhs: Box<Expr> = self.comparison();
            expr = Box::new(Expr::Binary { left: expr, operator: op, right: rhs });
        }

        expr
    }

    fn comparison(&mut self) -> Box<Expr> {
        let mut expr: Box<Expr> = self.term();

        while self.match_token(&[Greater, GreaterEqual, Less, LessEqual]) {
            let op: Token = self.previous();
            let rhs: Box<Expr> = self.term();
            expr = Box::new(Expr::Binary { left: expr, operator: op, right: rhs });
        }

        expr
    }

    fn term(&mut self) -> Box<Expr> {
        let mut expr: Box<Expr> = self.factor();

        while self.match_token(&[Minus, Plus]) {
            let op: Token = self.previous();
            let rhs: Box<Expr> = self.factor();
            expr = Box::new(Expr::Binary { left: expr, operator: op, right: rhs });
        }

        expr
    }

    fn factor(&mut self) -> Box<Expr> {
        let mut expr: Box<Expr> = self.unary();

        while self.match_token(&[Slash, Star]) {
            let op: Token = self.previous();
            let rhs: Box<Expr> = self.unary();
            expr = Box::new(Expr::Binary { left: expr, operator: op, right: rhs });
        }

        expr
    }

    fn unary(&mut self) -> Box<Expr> {
        if self.match_token(&[Bang, Minus]) {
            let op: Token = self.previous();
            let rhs: Box<Expr> =  self.unary();
            return Box::new(Expr::Unary { operator: op, right: rhs });
        }

        self.primary()
    }

    fn primary(&mut self) -> Box<Expr> { 
        if self.match_token(&[False]) {
            return Box::new(Expr::Literal { value: LiteralObject::BooleanLiteral { value: false }})
        } else if self.match_token(&[True]) {
            return Box::new(Expr::Literal { value: LiteralObject::BooleanLiteral { value: true }});
        } else if self.match_token(&[Nil]) {
            return Box::new(Expr::Literal { value: LiteralObject::NilLiteral })
        }

        if self.match_token(&[Number]) {
            let tok = self.previous();
            return Box::new(Expr::Literal { value: LiteralObject::NumberLiteral { start: tok.start , end: tok.end }})
        } else if self.match_token(&[Str]) {
            let tok: Token = self.previous();
            return Box::new(Expr::Literal { value: LiteralObject::StringLiteral { start: tok.start, end: tok.end } })
        }

        if self.match_token(&[Identifier]) {
            return Box::new(Expr::Variable { name: self.previous() })
        }

        if self.match_token(&[LeftParen]) {
            let expr: Box<Expr> = self.expression();
            self.consume(RightParen, ParserError::UnclosedParen);
            return Box::new(Expr::Grouping { expression: expr })
        }

        lox_error((self.previous()).line, ParserError::PrimaryExprExpected.into());
        panic!();
    }

    fn match_token(&mut self, tokens: &[TokenType]) -> bool {
        for token_type in tokens {
            if self.check(token_type) {
                self.advance();
                return true
            }
        }

        false
    }

    fn check(&self, token_type: &TokenType) -> bool {
        if self.is_at_end() { return false }
        self.peek().token_type == *token_type
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() { self.current += 1 }
        return self.previous()

    }

    fn is_at_end(&self) -> bool {
        self.peek().token_type == Eof
    }

    fn peek(&self) -> Token {
        *self.tokens.get(self.current).unwrap_or_else(|| panic!("Could not get the {}th token.", self.current))
    }
    
    fn previous(&self) -> Token {
        *self.tokens.get(self.current - 1).unwrap_or_else(|| panic!("Could not get the {}th token.", self.current-1))
    }

    fn consume(&mut self, token_type: TokenType, err: ParserError) -> Result<Token, ParserErrTup> {
        if self.check(&token_type) {
            return Ok(self.advance())
        } 
        Err(ParserErrTup(self.previous().line, err))
    }

    fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if self.previous().token_type == Semicolon { break; }

            match self.peek().token_type {
                Class | Fun | Var | For | If | While | Print | Return => break,
                _ => { self.advance(); }
            }
        }
    }
}