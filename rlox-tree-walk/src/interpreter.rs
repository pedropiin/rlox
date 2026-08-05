use fnv::FnvHashMap;
use std::rc::Rc;
use std::cell::RefCell;

use crate::token::{TokenType};
use crate::expr::{Expr, LiteralObject};
use crate::stmt::Stmt;
use crate::errors::{RuntimeErrTup, RuntimeError, lox_error};
use crate::utils::{IGNORE_USIZE, parse_escape_sequences};
use crate::lox_callable::*;
use crate::native_functions::ClockFunction;

const EPSILON: f32 = 1e-6;

#[derive(Debug, Clone)]
pub enum LiteralValue {
    StringValue(String),
    NumberValue(f32),
    BooleanValue(bool),
    NilValue,
    UninitializedValue,
    Function(Function),
}

pub struct Interpreter {
    pub globals: Rc<RefCell<Environment>>,
    variables: Rc<RefCell<Environment>>,
    repl_mode: bool,
}

impl Interpreter {
    pub fn new(repl_mode: bool) -> Interpreter {
        let env: Rc<RefCell<Environment>> = Rc::new(RefCell::new(Environment::new()));

        env.borrow_mut().define(
            "clock".to_string(),
            LiteralValue::Function(Function::Native(Rc::new(ClockFunction)))
        );

        Interpreter { globals: env.clone(), variables: env.clone(), repl_mode: repl_mode }
    }

    pub fn interpret(&mut self, stmts: &Vec<Box<Stmt>>) -> bool {
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

    fn execute(&mut self, stmt: &Stmt) -> Result<(), RuntimeErrTup> {
        match stmt {
            Stmt::Block { statements } => {
                let local_env: Rc<RefCell<Environment>> = Rc::new(RefCell::new(Environment::new_local(self.variables.clone())));
                self.execute_block(statements, local_env)
            },
            Stmt::Break => {
                Err(RuntimeErrTup(IGNORE_USIZE, RuntimeError::BreakStmtException))
            },
            Stmt::Continue => {
                Err(RuntimeErrTup(IGNORE_USIZE, RuntimeError::ContinueStmtException))
            },
            Stmt::Expression { expr } => {
                match self.evaluate(expr) {
                    Ok(res) => {
                        if self.repl_mode { self.stringify(&res); }
                        Ok(())
                    },
                    Err(err) => Err(err),
                }
            },
            Stmt::Function { name, params: _, body: _ } => {
                let user_def_func = UserDefinedFunction::new(Box::new(stmt.clone()));
                let function = LiteralValue::Function(Function::UserDefined(user_def_func));
                self.variables.borrow_mut().define((*name.lexeme).clone(), function);
                Ok(())
            },
            Stmt::If { condition, then_branch, else_branch } => {
                let condition_result: LiteralValue = self.evaluate(condition)?;
                if self.is_truthy(&condition_result) {
                    self.execute(then_branch)?;
                } else if let Some(else_br) = else_branch {
                    self.execute(else_br)?;
                }
                Ok(())
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
            Stmt::Return { keyword: _, value } => {
                let mut ret_value = LiteralValue::NilValue;
                if let Some(val) = value {
                    ret_value = self.evaluate(val.as_ref())?;
                }

                Err(RuntimeErrTup(IGNORE_USIZE, RuntimeError::ReturnStmtException(ret_value)))
            },
            Stmt::Var { token, initializer } => {
                let init_value = match initializer {
                    Some(expr) => self.evaluate(expr)?,
                    None => LiteralValue::UninitializedValue,
                };
                let var_name = (*token.lexeme).clone();
                self.variables.borrow_mut().define(var_name, init_value);
                Ok(())
            },
            Stmt::While { condition, body } => {
                let mut condition_result: LiteralValue = self.evaluate(condition)?;
                while self.is_truthy(&condition_result) {
                    if let Err(err) = self.execute(body) {
                        match err.1 {
                            RuntimeError::BreakStmtException    => break,
                            RuntimeError::ContinueStmtException => continue,
                            _                                   => return Err(err),
                        }
                    }
                    condition_result = self.evaluate(condition)?;
                }
                Ok(())
            },
        }
    }

    pub fn execute_block(&mut self, statements: &Vec<Box<Stmt>>, environment: Rc<RefCell<Environment>>) -> Result<(), RuntimeErrTup> {
        let previous_env = self.variables.clone();

        self.variables = environment;
        let mut error: Option<RuntimeErrTup> = None;
        for stmt in statements {
            match self.execute(stmt) {
                Ok(_) => (),
                Err(err) => {
                    error = Some(err);
                    break;
                },
            }
        }

        self.variables = previous_env;
        if let Some(err) = error { return Err(err) }
        Ok(())
    }

    fn evaluate(&mut self, expr: &Expr) -> Result<LiteralValue, RuntimeErrTup> {
        match expr {
            Expr::Assign { token, value } => {
                let rvalue = self.evaluate(value)?;
                let lvalue_name: &String = token.lexeme.as_ref();
                match self.variables.borrow_mut().assign(lvalue_name, rvalue.clone()) {
                    Ok(_) => Ok(rvalue),
                    Err(err) => Err(RuntimeErrTup(token.line, err))
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
                            Ok(LiteralValue::NumberValue(lhs_num + rhs_num))
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
                        match self.is_equal(&lhs_value, &rhs_value) {
                            Ok(b) => Ok(LiteralValue::BooleanValue(b)),
                            Err(err) => Err(RuntimeErrTup(operator.line, err))
                        }
                    },TokenType::BangEqual => {
                        match self.is_equal(&lhs_value, &rhs_value) {
                            Ok(b) => Ok(LiteralValue::BooleanValue(!b)),
                            Err(err) => Err(RuntimeErrTup(operator.line, err))
                        }
                    },
                    _ => unreachable!("A binary expression cannot contain any other TokenType."),
                }
            },
            Expr::Call { callee, paren, args } => {
                let callee: LiteralValue = self.evaluate(callee)?;

                let mut arguments: Vec<LiteralValue> = Vec::new();
                for arg in args {
                    arguments.push(self.evaluate(arg.as_ref())?);
                }

                match callee {
                    LiteralValue::Function(function) => {
                        if arguments.len() != function.arity() {
                            return Err(RuntimeErrTup(paren.line, RuntimeError::CallParityError(function.arity(), arguments.len())))
                        }
                        Ok(function.call(self, arguments)?)
                    },
                    _ => Err(RuntimeErrTup(paren.line, RuntimeError::NonCallableValueError)),
                }
            },
            Expr::Grouping { expression } => {
                self.evaluate(expression.as_ref())
            },
            Expr::Literal { value} => {
                match value {
                    LiteralObject::StringLiteral { lexeme } =>  {
                        Ok(LiteralValue::StringValue(parse_escape_sequences(lexeme.as_ref())))
                    },
                    LiteralObject::NumberLiteral { lexeme } => {
                        // if .unwrap() fails, it means that either 
                        // (1) tokenization or (2) parsing expressions failed,
                        // because if something else is stored inside a 
                        // "LiteralObject::NumberLiteral", it's not the users fault.
                        Ok(LiteralValue::NumberValue(lexeme.as_ref().parse::<f32>().unwrap()))
                    },
                    LiteralObject::BooleanLiteral { value } => {
                        Ok(LiteralValue::BooleanValue(*value))
                    },
                    LiteralObject::NilLiteral => {
                        Ok(LiteralValue::NilValue)
                    },
                }
            },
            Expr::Logical { left, operator, right } => {
                let lhs_value: LiteralValue = self.evaluate(left)?;
                match operator.token_type {
                    TokenType::Or => {
                        if self.is_truthy(&lhs_value) {
                            return Ok(lhs_value)
                        }
                    },
                    TokenType::And => {
                        if !self.is_truthy(&lhs_value) {
                            return Ok(lhs_value)
                        }
                    },
                    _ => unreachable!("A logical expression cannot contain any other operator/token type."),
                }
                Ok(self.evaluate(right)?)
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
            Expr::Variable { token } => {
                let var_name: &str = token.lexeme.as_ref();
                match self.variables.borrow().get(var_name) {
                    Some(val) => {
                        match val {
                            LiteralValue::UninitializedValue => Err(RuntimeErrTup(token.line, RuntimeError::UninitializedVariableError)),
                            _ => Ok(val),
                        }
                    },
                    None => Err(RuntimeErrTup(token.line, RuntimeError::UndefinedVariableError(var_name.to_string()))),
                }
            },
        }
    }

    fn is_truthy(&self, value: &LiteralValue) -> bool {
        match value {
            LiteralValue::NilValue                => false,
            LiteralValue::BooleanValue(b ) => *b,
            _                                     => true,
        }
    }

    fn is_equal(&self, lhs_val: &LiteralValue, rhs_val: &LiteralValue) -> Result<bool, RuntimeError> {
        match lhs_val {
            LiteralValue::NumberValue(lhs) => {
                if let LiteralValue::NumberValue(rhs) = rhs_val {
                    Ok(lhs == rhs)
                } else { Ok(false) }
            },
            LiteralValue::StringValue(lhs) => {
                if let LiteralValue::StringValue(rhs) = rhs_val {
                    Ok(lhs == rhs)
                } else { Ok(false) }
            },
            LiteralValue::BooleanValue(lhs) => {
                if let LiteralValue::BooleanValue(rhs) = rhs_val {
                    Ok(lhs == rhs)
                } else { Ok(false) }
            },
            LiteralValue::NilValue => {
                if let LiteralValue::NilValue = rhs_val { Ok(true) }
                else { Ok(false) }
            },
            LiteralValue::UninitializedValue => Err(RuntimeError::UninitializedVariableError),
            LiteralValue::Function(_) => Err(RuntimeError::FunctionNotComparableError)
        }
    }

    fn stringify(&self, value: &LiteralValue) -> () {
        let str_value = match value {
            LiteralValue::StringValue(s) => s.clone(),
            LiteralValue::NumberValue(num) => {
                let str_num: String = num.to_string();
                match str_num.strip_suffix(".0") {
                    Some(s) => s.to_string(),
                    None => str_num,
                }
                
            },
            LiteralValue::BooleanValue(b) => b.to_string(),
            LiteralValue::NilValue => "nil".to_string(),
            LiteralValue::UninitializedValue 
                => unreachable!("Even though a print statement can be called with an uninitialized variable, 
                                its evaluation will raise the error prior to the 'print' statement evaluation itself."),
            LiteralValue::Function(function) => function.to_string().to_string(),
        };
        println!("{}", str_value);
    }
}

#[derive(Clone)]
pub struct Environment {
    variables: FnvHashMap<String, LiteralValue>,
    enclosing: Option<Rc<RefCell<Environment>>>,
}

impl Environment {
    pub fn new() -> Environment {
        Environment { variables: FnvHashMap::default(), enclosing: None }
    }

    pub fn new_local(enclosing: Rc<RefCell<Environment>>) -> Environment {
        Environment { variables: FnvHashMap::default(), enclosing: Some(enclosing) }
    }

    pub fn define(&mut self, name: String, value: LiteralValue) -> () {
        self.variables.insert(name, value);
    }

    pub fn get(&self, name: &str) -> Option<LiteralValue> {
        if self.variables.contains_key(name) {
            return self.variables.get(name).cloned()
        } else {
            if let Some(ref env) = self.enclosing {
                return env.borrow().get(name)
            }
            None
        }
    }

    pub fn assign(&mut self, name: &str, value: LiteralValue) -> Result<(), RuntimeError> {
        if let Some(val) = self.variables.get_mut(name) {
            *val = value;
            return Ok(())
        } else {
            if let Some(ref env) = self.enclosing {
                return env.borrow_mut().assign(name, value)
            }            
        }
        Err(RuntimeError::UndefinedVariableError(name.to_string()))
    }
}