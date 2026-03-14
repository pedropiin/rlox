use std::time::{SystemTime, UNIX_EPOCH};

use crate::interpreter::{Interpreter, LiteralValue};

pub trait LoxCallable {
    fn call(&self, interpreter: &Interpreter, arguments: Vec<LiteralValue>) -> LiteralValue;
    fn arity(&self) -> usize;
    fn to_string(&self) -> &str;
}

#[derive(Debug, Clone)]
pub enum Function {
    Native(NativeFunction),
    UserDefined(UserDefinedFunction),
}
impl LoxCallable for Function {
    fn call(&self, interpreter: &Interpreter, arguments: Vec<LiteralValue>) -> LiteralValue {
        match self {
            Function::Native(native_func) => {
                match native_func {
                    NativeFunction::Clock(clock_func) => clock_func.call(interpreter, arguments),
                }
            },
            Function::UserDefined(user_defined_func) => {
                todo!();
            }
        }
    }

    fn arity(&self) -> usize {
        match self {
            Function::Native(native_func) => {
                match native_func {
                    NativeFunction::Clock(clock_func) => clock_func.arity(),
                }
            },
            Function::UserDefined(user_defined_func) => {
                todo!();
            }
        }
    }

    fn to_string(&self) -> &str {
        match self {
            Function::Native(native_func) => {
                match native_func {
                    NativeFunction::Clock(clock_func) => clock_func.to_string(),
                }
            },
            Function::UserDefined(user_defined_func) => {
                todo!();
            },
        }
    }
}

#[derive(Debug, Clone)]
pub enum NativeFunction {
    Clock(ClockFunction),
}

#[derive(Debug, Clone)]
pub struct ClockFunction;
impl LoxCallable for ClockFunction {
    fn call(&self, _1: &Interpreter, _2: Vec<LiteralValue>) -> LiteralValue {
        let now = 
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Non-monotonic clock drift caused interal 'clock' duration to be negative.")
                .as_secs_f32();
        LiteralValue::NumberValue(now)
    }

    fn arity(&self) -> usize {
        0
    }

    fn to_string(&self) -> &str {
        "<native clock fn>"
    }
}

#[derive(Debug, Clone)]
struct UserDefinedFunction;