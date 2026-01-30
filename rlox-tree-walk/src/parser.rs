use core::panic;

use crate::token::Token;
use crate::token::TokenType::{self, *};
use crate::expr::Expr;
use crate::stmt::Stmt;
use crate::expr::LiteralObject::{self};
use crate::errors::{ParserError, lox_error};

#[derive(Clone)]
struct ParserErrTup(usize, ParserError);

pub struct Parser<'a> {
    tokens: &'a mut Vec<Token>,
    stmts: &'a mut Vec<Box<Stmt>>,
    current: usize,
    had_error: bool,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a mut Vec<Token>, stmts: &'a mut Vec<Box<Stmt>>) -> Parser<'a> {
        Parser { tokens, stmts, current: 0, had_error: false }
    }

    pub fn parse(&mut self) -> bool {
        while !self.is_at_end() {
            if let Some(stmt) = self.declaration() {
                self.stmts.push(stmt);
            }
        }
        self.had_error
    }

    fn declaration(&mut self) -> Option<Box<Stmt>> {
        let result = 
            if self.match_token(&[Var]) {
                self.var_declaration()
            } else { self.statement() };

        match result {
            Ok(stmt) => Some(stmt),
            Err(parser_err) => {
                lox_error(parser_err.0, parser_err.1.into());
                self.had_error = true;
                self.synchronize();
                None
            }
        }
    }

    fn var_declaration(&mut self) -> Result<Box<Stmt>, ParserErrTup> {
        let name: Token = self.consume(Identifier, ParserError::NamelessVarDeclaration)?;

        let mut initializer: Option<Box<Expr>> = None;
        if self.match_token(&[Equal]) { initializer = Some(self.expression()?) }

        self.consume(Semicolon, ParserError::SemicolonExpected)?;
        Ok(Box::new(Stmt::Var { token: name, initializer: initializer }))
    }

    fn statement(&mut self) -> Result<Box<Stmt>, ParserErrTup> {
        if self.match_token(&[Print]) {
            return self.print_statement()
        } 
        if self.match_token(&[LeftBrace]) {
            return Ok(Box::new(Stmt::Block { statements: self.block()? }))
        }
        self.expr_statement()
    }
    
    fn expr_statement(&mut self) -> Result<Box<Stmt>, ParserErrTup> {
        let expr: Box<Expr> = self.expression()?;
        self.consume(Semicolon, ParserError::SemicolonExpected)?;
        
        Ok(Box::new(Stmt::Expression { expr: expr }))
    }

    fn block(&mut self) -> Result<Vec<Box<Stmt>>, ParserErrTup> {
        let mut statements: Vec<Box<Stmt>> = vec![];

        while !self.check(RightBrace) && !self.is_at_end() {
            if let Some(stmt) = self.declaration() {
                statements.push(stmt);
            }
        }

        self.consume(RightBrace, ParserError::RightBraceExpected)?;
        Ok(statements)
    }
    
    fn print_statement(&mut self) -> Result<Box<Stmt>, ParserErrTup> {
        let expr: Box<Expr> = self.expression()?;
        self.consume(Semicolon, ParserError::SemicolonExpected)?;
        
        Ok(Box::new(Stmt::Print { expr: expr }))
    }
    
    fn expression(&mut self) -> Result<Box<Expr>, ParserErrTup> {
        self.assignment()
    }

    fn assignment(&mut self) -> Result<Box<Expr>, ParserErrTup> {
        let lvalue_expr = self.equality()?;

        if self.match_token(&[Equal]) {
            let equals: Token = self.previous();
            let value: Box<Expr> = self.assignment()?;

            if let Expr::Variable { token } = lvalue_expr.as_ref() {
                return Ok(Box::new(Expr::Assign { token: *token, value: value }))
            }

            // No need to return early and synchronize, as there's no need to panic.
            // The remaining of the script can (possibly) run.
            lox_error(equals.line, ParserError::InvalidAssignment.into());
            self.had_error = true;
        }

        Ok(lvalue_expr)
    }

    fn equality(&mut self) -> Result<Box<Expr>, ParserErrTup> {
        let mut expr: Box<Expr> = self.comparison()?;
        
        while self.match_token(&[BangEqual, EqualEqual]) {
            let op: Token = self.previous();
            let rhs: Box<Expr> = self.comparison()?;
            expr = Box::new(Expr::Binary { left: expr, operator: op, right: rhs });
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Box<Expr>, ParserErrTup> {
        let mut expr: Box<Expr> = self.term()?;

        while self.match_token(&[Greater, GreaterEqual, Less, LessEqual]) {
            let op: Token = self.previous();
            let rhs: Box<Expr> = self.term()?;
            expr = Box::new(Expr::Binary { left: expr, operator: op, right: rhs });
        }

        Ok(expr)
    }

    fn term(&mut self) -> Result<Box<Expr>, ParserErrTup> {
        let mut expr: Box<Expr> = self.factor()?;

        while self.match_token(&[Minus, Plus]) {
            let op: Token = self.previous();
            let rhs: Box<Expr> = self.factor()?;
            expr = Box::new(Expr::Binary { left: expr, operator: op, right: rhs });
        }

        Ok(expr)
    }

    fn factor(&mut self) -> Result<Box<Expr>, ParserErrTup> {
        let mut expr: Box<Expr> = self.unary()?;

        while self.match_token(&[Slash, Star]) {
            let op: Token = self.previous();
            let rhs: Box<Expr> = self.unary()?;
            expr = Box::new(Expr::Binary { left: expr, operator: op, right: rhs });
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Box<Expr>, ParserErrTup> {
        if self.match_token(&[Bang, Minus]) {
            let op: Token = self.previous();
            let rhs: Box<Expr> =  self.unary()?;
            return Ok(Box::new(Expr::Unary { operator: op, right: rhs }))
        }

        self.primary()
    }

    fn primary(&mut self) -> Result<Box<Expr>, ParserErrTup> { 
        if self.match_token(&[False]) {
            return Ok(Box::new(Expr::Literal { value: LiteralObject::BooleanLiteral { value: false }}))
        } else if self.match_token(&[True]) {
            return Ok(Box::new(Expr::Literal { value: LiteralObject::BooleanLiteral { value: true }}))
        } else if self.match_token(&[Nil]) {
            return Ok(Box::new(Expr::Literal { value: LiteralObject::NilLiteral }))
        }

        if self.match_token(&[Number]) {
            let tok = self.previous();
            return Ok(Box::new(Expr::Literal { value: LiteralObject::NumberLiteral { start: tok.start , end: tok.end }}))
        } else if self.match_token(&[Str]) {
            let tok: Token = self.previous();
            return Ok(Box::new(Expr::Literal { value: LiteralObject::StringLiteral { start: tok.start, end: tok.end } }))
        }

        if self.match_token(&[Identifier]) {
            return Ok(Box::new(Expr::Variable { token: self.previous() }))
        }

        if self.match_token(&[LeftParen]) {
            let expr: Box<Expr> = self.expression()?;
            self.consume(RightParen, ParserError::UnclosedParen)?;
            return Ok(Box::new(Expr::Grouping { expression: expr }))
        }

        Err(ParserErrTup(self.previous().line, ParserError::PrimaryExprExpected))
    }

    fn match_token(&mut self, tokens: &[TokenType]) -> bool {
        for token_type in tokens {
            if self.check(*token_type) {
                self.advance();
                return true
            }
        }

        false
    }

    fn check(&self, token_type: TokenType) -> bool {
        if self.is_at_end() { return false }
        self.peek().token_type == token_type
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
        if self.check(token_type) {
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