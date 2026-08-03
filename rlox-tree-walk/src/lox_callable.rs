use std::rc::Rc;
use std::fmt;

use crate::interpreter::{Interpreter, LiteralValue};

pub trait LoxCallable {
    fn call(&self, interpreter: &Interpreter, arguments: Vec<LiteralValue>) -> LiteralValue;
    fn arity(&self) -> usize;
    fn to_string(&self) -> &str;
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
}
impl LoxCallable for Function {
    fn call(&self, interpreter: &Interpreter, arguments: Vec<LiteralValue>) -> LiteralValue {
        match self {
            Function::Native(native_func) => {
                native_func.call(interpreter, arguments)
            },
            Function::UserDefined(user_defined_func) => {
                todo!();
            },
        }
    }

    fn arity(&self) -> usize {
        match self {
            Function::Native(native_func) => {
                native_func.arity()
            },
            Function::UserDefined(user_defined_func) => {
                todo!();
            },
        }
    }

    fn to_string(&self) -> &str {
        match self {
            Function::Native(native_func) => {
                native_func.to_string()
            },
            Function::UserDefined(user_defined_func) => {
                todo!();
            },
        }
    }

}

#[derive(Debug,Clone)]
struct UserDefinedFunction;