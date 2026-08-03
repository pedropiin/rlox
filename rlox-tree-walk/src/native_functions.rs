use std::time::{SystemTime, UNIX_EPOCH};

use crate::interpreter::{Interpreter, LiteralValue};
use crate::lox_callable::LoxCallable;
use crate::utils::IGNORE_USIZE;
use crate::errors::{RuntimeErrTup, RuntimeError};

#[derive(Debug, Clone)]
pub struct ClockFunction;
impl LoxCallable for ClockFunction {
    fn call(&self, _1: &mut Interpreter, _2: Vec<LiteralValue>) -> Result<LiteralValue, RuntimeErrTup> {
        match SystemTime::now().duration_since(UNIX_EPOCH) {
            Ok(duration) => Ok(LiteralValue::NumberValue(duration.as_secs_f32())),
            Err(e) => Err(RuntimeErrTup(IGNORE_USIZE, RuntimeError::ClockCallError)),
        }
    }

    fn arity(&self) -> usize {
        0
    }

    fn to_string(&self) -> String {
        "<native clock fn>".to_string()
    }
}