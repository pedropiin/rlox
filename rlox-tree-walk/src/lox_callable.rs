use std::rc::Rc;
use std::cell::RefCell;
use std::{fmt, unreachable};

use crate::errors::{RuntimeErrTup, RuntimeError};
use crate::interpreter::{Environment, Interpreter, LiteralValue};
use crate::Stmt;
use crate::token::Token;

pub trait LoxCallable {
    fn call(&self, interpreter: &mut Interpreter, arguments: Vec<LiteralValue>) -> Result<LiteralValue, RuntimeErrTup>;
    fn arity(&self) -> usize;
    fn to_string(&self) -> String;
}
impl fmt::Debug for dyn LoxCallable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("LoxCallable")
            .field("name", &self.to_string())
            .finish()
    }
}

#[derive(Debug,Clone)]
pub enum Function {
    // I guess here we can use Rc's of trait objects to allow cloning due to the fact that 
    // native functions won't change ever.  
    Native(Rc<dyn LoxCallable>), 
    UserDefined(UserDefinedFunction),
    Lambda(LambdaFunction),
}
impl LoxCallable for Function {
    fn call(&self, interpreter: &mut Interpreter, arguments: Vec<LiteralValue>) -> Result<LiteralValue, RuntimeErrTup> {
        match self {
            Function::Native(native_func) => {
                native_func.call(interpreter, arguments)
            },
            Function::UserDefined(user_defined_func) => {
                user_defined_func.call(interpreter, arguments)
            },
            Function::Lambda(lambda_func) => {
                lambda_func.call(interpreter, arguments)
            },
        }
    }

    fn arity(&self) -> usize {
        match self {
            Function::Native(native_func) => {
                native_func.arity()
            },
            Function::UserDefined(user_defined_func) => {
                user_defined_func.arity()
            },
            Function::Lambda(lambda_func) => {
                lambda_func.arity()
            },
        }
    }

    fn to_string(&self) -> String {
        match self {
            Function::Native(native_func) => {
                native_func.to_string()
            },
            Function::UserDefined(user_defined_func) => {
                user_defined_func.to_string()
            },
            Function::Lambda(lambda_func) => {
                lambda_func.to_string()
            },
        }
    }

}

#[derive(Debug,Clone)]
pub struct UserDefinedFunction {
    pub declaration: Box<Stmt>,
    pub closure: Rc<RefCell<Environment>>,
}
impl UserDefinedFunction {
    pub fn new(decl: Box<Stmt>, clos: Rc<RefCell<Environment>>) -> UserDefinedFunction {
        let Stmt::Function {..} = decl.as_ref() else {
            unreachable!("Function declaration must be extracted out of a Stmt::Function node.
                        Unreachable because 'UserDefinedFunction' instantiation is guarded by 
                        the runtime's exhaustive statement pattern matching @interpreter.rs, line 81.")
        };
        UserDefinedFunction { declaration: decl, closure: clos }
    }

    fn call(&self, interpreter: &mut Interpreter, arguments: Vec<LiteralValue>) -> Result<LiteralValue, RuntimeErrTup> {
        let Stmt::Function {name: _, params, body} = self.declaration.as_ref() else {
            unreachable!("Function declaration must be extracted out of a Stmt::Function node.
                        Unreachable because 'UserDefinedFunction' instantiation is guarded by 
                        the runtime's exhaustive statement pattern matching @interpreter.rs, line 81.")
        };

        let env: Rc<RefCell<Environment>> = Rc::new(RefCell::new(Environment::new_local(self.closure.clone())));
        for i in 0..params.len() {
            let param_name = (*params.get(i).unwrap().lexeme).clone();
            let arg_value = arguments.get(i).unwrap().clone();
            env.borrow_mut().define(param_name, arg_value);
        }   

        let mut return_value: LiteralValue = LiteralValue::NilValue;
        if let Err(err) = interpreter.execute_block(body, env) {
            match err.1 {
                RuntimeError::ReturnStmtException(ret) => return_value = ret,
                _ => return Err(err),
            }
        }
        Ok(return_value)
    }

    fn arity(&self) -> usize {
        let Stmt::Function {name: _, params, body: _} = self.declaration.as_ref() else {
            unreachable!("Function declaration must be extracted out of a Stmt::Function node.
                        Unreachable because 'UserDefinedFunction' instantiation is guarded by 
                        the runtime's exhaustive statement pattern matching @interpreter.rs, line 81.")
        };
        params.len()
    }

    fn to_string(&self) -> String {
        let Stmt::Function {name, params: _, body: _} = self.declaration.as_ref() else {
            unreachable!("Function declaration must be extracted out of a Stmt::Function node.
                        Unreachable because 'UserDefinedFunction' instantiation is guarded by 
                        the runtime's exhaustive statement pattern matching @interpreter.rs, line 81.")
        };
        let mut ret: String = String::from("<fn "); 
        ret.push_str(name.lexeme.as_ref());
        ret.push_str(">");
        ret
    }
}

#[derive(Debug, Clone)]
pub struct LambdaFunction {
    pub params: Vec<Token>,
    pub body: Vec<Box<Stmt>>,
}
impl LambdaFunction {
    pub fn new(params: Vec<Token>, body: Vec<Box<Stmt>>) -> LambdaFunction {
        LambdaFunction { params: params, body: body }
    }

    fn call(&self, interpreter: &mut Interpreter, arguments: Vec<LiteralValue>) -> Result<LiteralValue, RuntimeErrTup> {
        let env: Rc<RefCell<Environment>> = Rc::new(RefCell::new(Environment::new_local(interpreter.globals.clone())));
        for i in 0..self.params.len() {
            let param_name: String = (*self.params.get(i).unwrap().lexeme).clone();
            let arg_value: LiteralValue = arguments.get(i).unwrap().clone();
            env.borrow_mut().define(param_name, arg_value);
        }

        let mut return_value: LiteralValue = LiteralValue::NilValue;
        if let Err(err) = interpreter.execute_block(&self.body, env) {
            match err.1 {
                RuntimeError::ReturnStmtException(ret) => return_value = ret,
                _ => return Err(err)
            }
        }
        Ok(return_value)
    }

    fn arity(&self) -> usize {
        self.params.len()
    }

    fn to_string(&self) -> String {
        String::from("<fn lambda>")
    }
}