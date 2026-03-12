use core::panic;

use crate::token::Token;
use crate::token::TokenType::{self, *};
use crate::expr::Expr;
use crate::stmt::Stmt;
use crate::expr::LiteralObject::{self};
use crate::errors::{ParserError, lox_error, MAX_ARGS};

#[derive(Clone)]
struct ParserErrTup(usize, ParserError);

pub struct Parser<'a> {
    tokens: &'a mut Vec<Token>,
    stmts: &'a mut Vec<Box<Stmt>>,
    current: usize,
    had_error: bool,
    in_loop: bool,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a mut Vec<Token>, stmts: &'a mut Vec<Box<Stmt>>) -> Parser<'a> {
        Parser { tokens, stmts, current: 0, had_error: false, in_loop: false }
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
        if self.match_token(&[For]) {
            return self.for_statement();
        }
        if self.match_token(&[If]) {
            return self.if_statement();
        }
        if self.match_token(&[Print]) {
            return self.print_statement()
        } 
        if self.match_token(&[While]) {
            return self.while_statement();
        }
        if self.match_token(&[LeftBrace]) {
            return Ok(Box::new(Stmt::Block { statements: self.block()? }))
        }
        if self.match_token(&[Break]) {
            return self.break_statement();
        }
        if self.match_token(&[Continue]) {
            return self.continue_statement();
        }
        self.expr_statement()
    }

    fn for_statement(&mut self) -> Result<Box<Stmt>, ParserErrTup> {
        let prev_in_loop: bool = self.in_loop;
        self.in_loop = true;

        self.consume(LeftParen, ParserError::LeftParenControlFlowConditionExpected)?;

        let initializer: Option<Box<Stmt>> = 
            if self.match_token(&[Semicolon]) { 
                None
            } else if self.match_token(&[Var]) { 
                Some(self.var_declaration()?)
            } else { 
                Some(self.expr_statement()?)
            };

        let condition: Box<Expr> = 
            if !self.check(Semicolon) { 
                self.expression()?
            } else { 
                Box::new(Expr::Literal { value: LiteralObject::BooleanLiteral { value: true } })
            };
        self.consume(Semicolon, ParserError::ForConditionSemicolonExpected)?;

        let increment: Option<Box<Expr>> = 
            if !self.check(RightParen) {
                Some(self.expression()?)
            } else {
                None
            };
        self.consume(RightParen, ParserError::RightParenControlFlowConditionExpected)?;

        let mut body: Box<Stmt> = self.statement()?;
        if let Some(inc_expr) = increment {
            body = Box::new(Stmt::Block { statements: vec![body, Box::new(Stmt::Expression { expr: inc_expr })] });
        }
        body = Box::new(Stmt::While { condition: condition, body });

        self.in_loop = prev_in_loop;

        if let Some(init_expr) = initializer {
            Ok(Box::new(Stmt::Block { statements: vec![init_expr, body] }))
        } else {
            Ok(Box::new(Stmt::Block { statements: vec![body] }))
        }
    }

    fn if_statement(&mut self) -> Result<Box<Stmt>, ParserErrTup> {
        self.consume(LeftParen, ParserError::LeftParenControlFlowConditionExpected)?;
        let condition: Box<Expr> = self.expression()?;
        self.consume(RightParen, ParserError::RightParenControlFlowConditionExpected)?;

        let then_branch: Box<Stmt> = self.statement()?;
        let else_branch: Option<Box<Stmt>> = 
            if self.match_token(&[Else]) { 
                Some(self.statement()?) 
            } else { None };

        Ok(Box::new(Stmt::If { condition, then_branch, else_branch }))
    }

    fn print_statement(&mut self) -> Result<Box<Stmt>, ParserErrTup> {
        let expr: Box<Expr> = self.expression()?;
        self.consume(Semicolon, ParserError::SemicolonExpected)?;
        
        Ok(Box::new(Stmt::Print { expr: expr }))
    }

    fn while_statement(&mut self) -> Result<Box<Stmt>, ParserErrTup> {
        let prev_in_loop: bool = self.in_loop;
        self.in_loop = true;

        self.consume(LeftParen, ParserError::LeftParenControlFlowConditionExpected)?;
        let condition: Box<Expr> = self.expression()?;
        self.consume(RightParen, ParserError::RightParenControlFlowConditionExpected)?;

        let body: Box<Stmt> = self.statement()?;

        self.in_loop = prev_in_loop;

        Ok(Box::new(Stmt::While { condition, body }))
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

    fn break_statement(&mut self) -> Result<Box<Stmt>, ParserErrTup> {
        if self.in_loop {
            self.consume(Semicolon, ParserError::SemicolonExpected)?;
            return Ok(Box::new(Stmt::Break))
        } else {
            return Err(ParserErrTup(self.previous().line, ParserError::BreakOutsideLoop))
        }
    }

    fn continue_statement(&mut self) -> Result<Box<Stmt>, ParserErrTup> {
        if self.in_loop {
            self.consume(Semicolon, ParserError::SemicolonExpected)?;
            return Ok(Box::new(Stmt::Continue))
        } else {
            return Err(ParserErrTup(self.previous().line, ParserError::ContinueOutsideLoop))
        }
    }

    fn expr_statement(&mut self) -> Result<Box<Stmt>, ParserErrTup> {
        let expr: Box<Expr> = self.expression()?;
        self.consume(Semicolon, ParserError::SemicolonExpected)?;
        
        Ok(Box::new(Stmt::Expression { expr: expr }))
    }
    
    fn expression(&mut self) -> Result<Box<Expr>, ParserErrTup> {
        self.assignment()
    }

    fn assignment(&mut self) -> Result<Box<Expr>, ParserErrTup> {
        let lvalue_expr = self.or()?;

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

    fn or(&mut self) -> Result<Box<Expr>, ParserErrTup> {
        let mut lhs_expr: Box<Expr> = self.and()?;

        while self.match_token(&[Or]) {
            let operator: Token = self.previous();
            let rhs_expr: Box<Expr> = self.and()?;
            lhs_expr = Box::new(Expr::Logical { left: lhs_expr, operator, right: rhs_expr })
        }
        Ok(lhs_expr)
    }

    fn and(&mut self) -> Result<Box<Expr>, ParserErrTup> {
        let mut lhs_expr: Box<Expr> = self.equality()?;

        while self.match_token(&[And]) {
            let operator: Token = self.previous();
            let rhs_expr: Box<Expr> = self.equality()?;
            lhs_expr = Box::new(Expr::Logical { left: lhs_expr, operator, right: rhs_expr });
        }
        Ok(lhs_expr)
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

    fn call(&mut self) -> Result<Box<Expr>, ParserErrTup> {
        let mut expr: Box<Expr> = self.primary()?;

        loop { 
            if self.match_token(&[LeftParen]) { 
                expr = self.finish_call(expr)?; 
            } else { 
                break;
            }
        }

        Ok(expr)
    }

    fn finish_call(&mut self, expr: Box<Expr>) -> Result<Box<Expr>, ParserErrTup> {
        let mut args: Vec<Box<Expr>> = Vec::new();
        
        if !self.check(RightParen) {
            loop {
                if args.len() >= MAX_ARGS {
                    lox_error(self.peek().line, ParserError::TooManyArguments.into());
                }
                args.push(self.expression()?);
                if self.match_token(&[Comma]) {
                    break;
                }
            }
        }

        let paren: Token = self.consume(RightParen, ParserError::MissingParenArgumentList)?;

        Ok(Box::new(Expr::Call { callee: expr, paren: paren, args: args }))
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