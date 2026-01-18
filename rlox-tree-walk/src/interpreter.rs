use crate::token::{Token, TokenType};
use crate::expr::{Expr, LiteralObject};
use crate::stmt::Stmt;
use crate::errors::{LoxError, RuntimeError, lox_error};

const EPSILON: f32 = 1e-6;

pub struct RuntimeErrTup(usize, RuntimeError);

pub enum LiteralValue {
    StringValue(String),
    NumberValue(f32),
    BooleanValue(bool),
    NilValue,
}

pub struct Interpreter<'a> {
    source: &'a str, 
    tokens: &'a Vec<Token>,
}

impl<'a> Interpreter<'a> {
    pub fn new(source: &'a str, tokens: &'a Vec<Token>) -> Interpreter<'a> {
        Interpreter { source: source, tokens: tokens }
    }

    pub fn interpret(&self, stmts: &Vec<Box<Stmt>>) -> bool {
        let mut had_runtime_error: bool = false;
        for stmt in stmts {
            match self.execute(stmt) {
                Ok(()) => (),
                Err(runtime_err) => {
                    lox_error(runtime_err.0, runtime_err.1.into());
                    had_runtime_error = true;
                },
            }
        }
        had_runtime_error
    }

    fn execute(&self, stmt: &Stmt) -> Result<(), RuntimeErrTup> {
        match stmt {
            Stmt::Expression { expr } => {
                match self.evaluate(expr) {
                    Ok(_) => Ok(()),
                    Err(err) => Err(err),
                }
            },
            Stmt::Print { expr } => {
                match self.evaluate(expr) {
                    Ok(val) => {
                        self.stringify(&val);
                        Ok(())
                    },
                    Err(err) => Err(err),
                }
            },
            Stmt::Var { name, initializer } => todo!(),
        }
    }

    fn evaluate(&self, expr: &Expr) -> Result<LiteralValue, RuntimeErrTup> {
        match expr {
            Expr::Literal { value} => {
                match value {
                    LiteralObject::StringLiteral { start, end } =>  {
                        Ok(LiteralValue::StringValue(self.get_lexeme(*start, *end).to_string()))
                    },
                    LiteralObject::NumberLiteral { start, end } => {
                        // if .unwrap() fails, it means that either 
                        // (1) tokenization or (2) parsing expressions failed,
                        // because if something else is stored inside a 
                        // "LiteralObject::NumberLiteral", it's not the users fault.
                        Ok(LiteralValue::NumberValue(self.get_lexeme(*start, *end).parse::<f32>().unwrap()))
                    },
                    LiteralObject::BooleanLiteral { value } => {
                        Ok(LiteralValue::BooleanValue(*value))
                    },
                    LiteralObject::NilLiteral => {
                        Ok(LiteralValue::NilValue)
                    },
                    LiteralObject::IdentifierLiteral { start, end } => {
                        todo!();
                    },
                }
            },
            Expr::Grouping { expression } => {
                self.evaluate(expression.as_ref())
            },
            Expr::Unary { operator, right } => {
                let rhs_value: LiteralValue = self.evaluate(right.as_ref())?;

                match operator.token_type {
                    TokenType::Minus => {
                        if let LiteralValue::NumberValue(num) = rhs_value {
                            Ok(LiteralValue::NumberValue(-num))
                        } else {
                            Err(RuntimeErrTup(operator.line, RuntimeError::InvalidUnaryOperandError))
                        }
                    },
                    TokenType::Bang => {
                        Ok(LiteralValue::BooleanValue(!self.is_truthy(&rhs_value)))
                    },
                    _ => unreachable!(),
                }
            },
            Expr::Binary { left, operator, right } => {
                let lhs_value: LiteralValue = self.evaluate(left.as_ref())?;
                let rhs_value: LiteralValue = self.evaluate(right.as_ref())?;

                match operator.token_type {
                    // Arithmetic Operators
                    TokenType::Minus => {
                        if let LiteralValue::NumberValue(lhs_num) = lhs_value && let LiteralValue::NumberValue(rhs_num) = rhs_value {
                            Ok(LiteralValue::NumberValue(lhs_num - rhs_num))
                        } else {
                            Err(RuntimeErrTup(operator.line, RuntimeError::InvalidBinaryOperandsError))
                        }
                    },
                    TokenType::Slash => {
                        if let LiteralValue::NumberValue(lhs_num) = lhs_value && let LiteralValue::NumberValue(rhs_num) = rhs_value {
                            if (rhs_num - 0.0).abs() < EPSILON {
                                Err(RuntimeErrTup(operator.line, RuntimeError::DivisionByZeroError))
                            } else {
                                Ok(LiteralValue::NumberValue(lhs_num / rhs_num))
                            }
                        } else {
                            Err(RuntimeErrTup(operator.line, RuntimeError::InvalidBinaryOperandsError))
                        }
                    },
                    TokenType::Star => {
                        if let LiteralValue::NumberValue(lhs_num) = lhs_value && let LiteralValue::NumberValue(rhs_num) = rhs_value {
                            Ok(LiteralValue::NumberValue(lhs_num * rhs_num))
                        } else {
                            Err(RuntimeErrTup(operator.line, RuntimeError::InvalidBinaryOperandsError))
                        }
                    },
                    TokenType::Plus => {
                        if let LiteralValue::NumberValue(lhs_num) = lhs_value && let LiteralValue::NumberValue(rhs_num) = rhs_value {
                            Ok(LiteralValue::NumberValue(lhs_num * rhs_num))
                        }  else if let LiteralValue::StringValue(lhs_str) = lhs_value && let LiteralValue::StringValue(rhs_str) = rhs_value {
                            Ok(LiteralValue::StringValue(format!("{lhs_str}{rhs_str}")))
                        } else {
                            Err(RuntimeErrTup(operator.line, RuntimeError::InvalidSumOperandsError))
                        }
                    },
                    
                    // Comparison Operators
                    TokenType::Greater => {
                        if let LiteralValue::NumberValue(lhs_num) = lhs_value && let LiteralValue::NumberValue(rhs_num) = rhs_value {
                            Ok(LiteralValue::BooleanValue(lhs_num > rhs_num))
                        } else {
                            Err(RuntimeErrTup(operator.line, RuntimeError::InvalidBinaryOperandsError))
                        }
                    },
                    TokenType::GreaterEqual => {
                        if let LiteralValue::NumberValue(lhs_num) = lhs_value && let LiteralValue::NumberValue(rhs_num) = rhs_value {
                            Ok(LiteralValue::BooleanValue(lhs_num >= rhs_num))
                        } else {
                            Err(RuntimeErrTup(operator.line, RuntimeError::InvalidBinaryOperandsError))
                        }
                    },
                    TokenType::Less => {
                        if let LiteralValue::NumberValue(lhs_num) = lhs_value && let LiteralValue::NumberValue(rhs_num) = rhs_value {
                            Ok(LiteralValue::BooleanValue(lhs_num < rhs_num))
                        } else {
                            Err(RuntimeErrTup(operator.line, RuntimeError::InvalidBinaryOperandsError))
                        }
                    },
                    TokenType::LessEqual => {
                        if let LiteralValue::NumberValue(lhs_num) = lhs_value && let LiteralValue::NumberValue(rhs_num) = rhs_value {
                            Ok(LiteralValue::BooleanValue(lhs_num <= rhs_num))
                        } else {
                            Err(RuntimeErrTup(operator.line, RuntimeError::InvalidBinaryOperandsError))
                        }
                    },

                    // Equality operators
                    TokenType::EqualEqual => {
                        Ok(LiteralValue::BooleanValue(self.is_equal(&lhs_value, &rhs_value)))
                    },TokenType::BangEqual => {
                        Ok(LiteralValue::BooleanValue(!self.is_equal(&lhs_value, &rhs_value)))
                    },
                    _ => unreachable!(),
                }
            },
            Expr::Variable { name } => todo!(),
        }
    }

    fn is_truthy(&self, value: &LiteralValue) -> bool {
        match value {
            LiteralValue::NilValue                => false,
            LiteralValue::BooleanValue(b ) => *b,
            _                                     => true,
        }
    }

    fn is_equal(&self, lhs_val: &LiteralValue, rhs_val: &LiteralValue) -> bool {
        match lhs_val {
            LiteralValue::NumberValue(lhs) => {
                if let LiteralValue::NumberValue(rhs) = rhs_val {
                    lhs == rhs
                } else { false }
            },
            LiteralValue::StringValue(lhs) => {
                if let LiteralValue::StringValue(rhs) = rhs_val {
                    lhs == rhs
                } else { false }
            },
            LiteralValue::BooleanValue(lhs) => {
                if let LiteralValue::BooleanValue(rhs) = rhs_val {
                    lhs == rhs
                } else { false }
            },
            LiteralValue::NilValue => {
                if let LiteralValue::NilValue = rhs_val { true }
                else { false }
            }
        }
    }

    fn stringify(&self, value: &LiteralValue) -> () {
        let str_value: String = match value {
            LiteralValue::StringValue(s) => s.clone(),
            LiteralValue::NumberValue(num) => {
                let str_num: String = num.to_string();
                match str_num.strip_suffix(".0") {
                    Some(s) => s.to_owned(),
                    None => str_num,
                }
                
            },
            LiteralValue::BooleanValue(b) => b.to_string(),
            LiteralValue::NilValue => "nil".to_owned(),
        };
        println!("{}", str_value);
    }

    fn get_lexeme(&self, start: usize, end: usize) -> &str {
        &self.source[start..end]
    }
}