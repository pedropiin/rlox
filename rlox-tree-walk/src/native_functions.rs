use std::time::{SystemTime, UNIX_EPOCH};

use crate::interpreter::{Interpreter, LiteralValue};
use crate::lox_callable::LoxCallable;

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