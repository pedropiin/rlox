use fnv::FnvHashMap;
use std::rc::Rc;
use std::cell::RefCell;

use crate::errors::{ResolverError, ResolverErrTup, lox_error};
use crate::expr::Expr;
use crate::interpreter::Interpreter;
use crate::resolver::FunctionType::Function;
use crate::stmt::Stmt;
use crate::token::Token;
use crate::utils::peek_vec;

#[derive(Debug, Clone, Copy)]
enum FunctionType {
    None, 
    Function,
    Lambda,
}

pub struct Resolver {
    interpreter: Rc<RefCell<Interpreter>>,
    scopes: Vec<Box<FnvHashMap<String, bool>>>,
    current_function: FunctionType,
}
impl Resolver {
    pub fn new(interpreter: Rc<RefCell<Interpreter>>) -> Resolver {
        Resolver { interpreter: interpreter, scopes: Vec::new(), current_function: FunctionType::None }
    }

    pub fn resolve(&mut self, stmts: &Vec<Box<Stmt>>) -> bool {
        let mut resolver_err: bool = false;
        for stmt in stmts { 
            match self.resolve_stmt(stmt) {
                Ok(()) => (),
                Err(err) => {
                    lox_error(err.0, err.1.into());
                    resolver_err = true;
                },
            }
        }

        resolver_err
    }

    fn resolve_stmt(&mut self, stmt: &Stmt) -> Result<(), ResolverErrTup> {
        match stmt {
            Stmt::Block { statements } => {
                self.begin_scope();
                for block_stmt in statements {
                    self.resolve_stmt(block_stmt)?;
                }
                self.end_scope();
            },
            Stmt::Break => {
                ();
            },
            Stmt::Continue => {
                ();
            },
            Stmt::Expression { expr } => {
                self.resolve_expr(expr)?;
            },
            Stmt::Function { name, params , body } => {
                self.declare(name)?;
                self.define(name);
                self.resolve_function(params, body, FunctionType::Function)?;
            },
            Stmt::If { condition, then_branch, else_branch } => {
                self.resolve_expr(condition)?;
                self.resolve_stmt(then_branch)?;
                if let Some(else_stmt) = else_branch {
                    self.resolve_stmt(else_stmt)?;
                } 
            },
            Stmt::Print { expr } => {
                self.resolve_expr(expr)?;
            },
            Stmt::Return { keyword: tok, value } => {
                if let FunctionType::None = self.current_function {
                    return Err(ResolverErrTup(tok.line, ResolverError::ReturnStmtOutisdeFunc))
                }

                if let Some(ret_value) = value {
                    self.resolve_expr(ret_value)?;
                }
            },
            Stmt::Var { token, initializer } => {
                self.declare(token)?;
                if let Some(init) = initializer {
                    self.resolve_expr(init)?;
                }
                self.define(token);
            },
            Stmt::While { condition, body } => {
                self.resolve_expr(condition)?;
                self.resolve_stmt(body)?;
            },

        }

        Ok(())
    }

    fn resolve_expr(&mut self, expr: &Expr) -> Result<(), ResolverErrTup> {
        match expr {
            Expr::Assign { token, value } => { 
                self.resolve_expr(value)?;
                self.resolve_local(expr, token);
            },
            Expr::Binary { left, operator: _, right } => { 
                self.resolve_expr(left)?;
                self.resolve_expr(right)?;
            },
            Expr::Call { callee, paren: _, args } => { 
                self.resolve_expr(callee)?;
                for arg in args {
                    self.resolve_expr(arg)?;
                }
            },
            Expr::Grouping { expression } => { 
                self.resolve_expr(expression)?;
            },
            Expr::Lambda { params, body } => { 
                self.resolve_function(params, body, FunctionType::Lambda)?;
            },
            Expr::Literal { value: _ } => { 
                ();
            },
            Expr::Logical { left, operator: _, right } => { 
                self.resolve_expr(left)?;
                self.resolve_expr(right)?;
            },
            Expr::Unary { operator: _, right } => { 
                self.resolve_expr(right)?;
            },
            Expr::Variable { token } => { 
                if let Some(scope) = peek_vec::<Box<FnvHashMap<String, bool>>>(self.scopes.as_mut()) {
                    if let Some(v) = scope.get(token.lexeme.as_ref()) && *v == false {
                        lox_error(token.line, ResolverError::VarReadOwnInitializer.into());
                    }
                } 

                // No need of checking 'None' cases, because if 'None', we are dealing with a 
                // varExpr in the global scope and, therefore, there's no shadowing scenario.
                // The program is either (1) defining a variable with itself, which is an error
                // by itself caught in runtime or (2) reassigning to a variable with its own value,
                // which is not exactly useful, but nor it is an error.
                self.resolve_local(expr, token);
            },
        }

        Ok(())
    }

    fn resolve_function(&mut self, params: &Vec<Token>, body: &Vec<Box<Stmt>>, new_function_type: FunctionType) -> Result<(), ResolverErrTup> {
        let enclosing_function_type: FunctionType = self.current_function;
        self.current_function = new_function_type;

        self.begin_scope();
        for param in params {
            self.declare(param)?;
            self.define(param);
        }
        for body_stmt in body {
            self.resolve_stmt(body_stmt)?;
        }
        self.end_scope();

        self.current_function = enclosing_function_type;

        Ok(())
    }

    fn begin_scope(&mut self) {
        self.scopes.push(Box::new(FnvHashMap::default()));
    }
    
    fn end_scope(&mut self) {
        self.scopes.pop();
    }

    fn declare(&mut self, tok: &Token) -> Result<(), ResolverErrTup> {
        if self.scopes.len() == 0 { return Ok(()) }

        if let Some(scope) = peek_vec::<Box<FnvHashMap<String, bool>>>(self.scopes.as_mut()) {
            if let Some(_) = scope.get(&*(tok.lexeme)) {
                return Err(ResolverErrTup(tok.line, ResolverError::VarRedeclarationInScope((*tok.lexeme).clone())))
            }
            scope.insert((*tok.lexeme).clone(), false);
        }

        Ok(())
    }

    fn define(&mut self, tok: &Token) {
        if self.scopes.len() == 0 { return }

        if let Some(scope) = peek_vec::<Box<FnvHashMap<String, bool>>>(self.scopes.as_mut()) {
            scope.insert((*tok.lexeme).clone(), true);
        }
    }

    fn resolve_local(&mut self, expr: &Expr, tok: &Token) {
        if self.scopes.len() == 0 { return }
        
        let sz: usize = self.scopes.len() - 1;
        for i in (0..=sz).rev() {
            if let Some(scope) = self.scopes.get(i) && let Some(_) = scope.get(tok.lexeme.as_ref()) {
                self.interpreter.borrow_mut().resolve(expr.clone(), sz - i);
                return;
            }
        }
    }
}