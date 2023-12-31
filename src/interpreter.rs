use crate::{expr::Expr, rox_type::RoxType, token_type::TokenType};


#[derive(Debug)]
pub struct Interpreter {
    pub had_runtime_error: bool
}

#[derive(Debug)]
pub struct InterpreterError{
    pub error_string: String,
}

pub type InterpreterResult = Result<RoxType, InterpreterError>;

impl Interpreter {

    pub fn new() -> Interpreter {
        Interpreter { had_runtime_error: false }
    }

    pub fn interpret(&mut self, expr: Expr) {
        let val = self.eval(expr);
        match val {
            Ok(rtyp) => println!("{:?}", rtyp),
            Err(e) => println!("Error: {}", e.error_string)
        }
    } 


    pub fn eval(&mut self, expr: Expr) -> InterpreterResult {
        match expr {
            Expr::Binary(lexp, t, rexp) => {
                let lval = self.eval(*lexp)?;
                let rval = self.eval(*rexp)?;
                match t.token_type {
                    TokenType::Minus => {
                        match (lval, rval) {
                            (RoxType::Number(lnum), RoxType::Number(rnum)) => Ok(RoxType::Number(lnum - rnum)),
                            _ => Err(InterpreterError { error_string: String::from("Invalid Types in Binary Minus Expr")}),
                        }
                    },
                    TokenType::Plus => {
                        match (lval, rval) {
                            (RoxType::Number(lnum), RoxType::Number(rnum)) => Ok(RoxType::Number(lnum - rnum)),
                            (RoxType::String(lstr), RoxType::String(rstr)) => {
                                let mut combined_string = lstr;
                                combined_string.push_str(&rstr);
                                Ok(RoxType::String(combined_string))
                            },
                            _ => Err(InterpreterError { error_string: String::from("Invalid Types in Binary Addition Expr")}),
                        }
                    }
                    TokenType::Slash => {
                        match (lval, rval) {
                            (RoxType::Number(lnum), RoxType::Number(rnum)) => Ok(RoxType::Number(lnum / rnum)),
                            _ => Err(InterpreterError { error_string: String::from("Invalid Types in Binary Division Expr")}),
                        }
                    }
                    TokenType::Star => {
                        match (lval, rval) {
                            (RoxType::Number(lnum), RoxType::Number(rnum)) => Ok(RoxType::Number(lnum * rnum)),
                            _ => Err(InterpreterError { error_string: String::from("Invalid Types in Binary Multiply Expr")}),
                        }
                    }
                    TokenType::Greater => {
                        match (lval, rval) {
                            (RoxType::Number(lnum), RoxType::Number(rnum)) => Ok(RoxType::Boolean(lnum > rnum)),
                            _ => Err(InterpreterError { error_string: String::from("Invalid Types in Binary Greater Comparison Expr")}),
                        }
                    }
                    TokenType::GreaterEqual => {
                        match (lval, rval) {
                            (RoxType::Number(lnum), RoxType::Number(rnum)) => Ok(RoxType::Boolean(lnum >= rnum)),
                            _ => Err(InterpreterError { error_string: String::from("Invalid Types in Binary GreaterEqual Comparison Expr")}),
                        }
                    }
                    TokenType::Less => {
                        match (lval, rval) {
                            (RoxType::Number(lnum), RoxType::Number(rnum)) => Ok(RoxType::Boolean(lnum < rnum)),
                            _ => Err(InterpreterError { error_string: String::from("Invalid Types in Binary Less Comparison Expr")}),
                        }
                    }
                    TokenType::LessEqual => {
                        match (lval, rval) {
                            (RoxType::Number(lnum), RoxType::Number(rnum)) => Ok(RoxType::Boolean(lnum <= rnum)),
                            _ => Err(InterpreterError { error_string: String::from("Invalid Types in Binary LessEqual Comparison Expr")}),
                        }
                    }
                    TokenType::BangEqual => {
                        Ok(RoxType::Boolean(isEqual(lval, rval)))
                    }
                    _ => panic!("Invalid Expression, parser panic")
                }
            }
            Expr::Grouping(exp) => self.eval(*exp),
            Expr::Literal(rtype) => Ok(rtype),
            Expr::Unary(t, exp) => {
                let eval = self.eval(*exp)?;
                match t.token_type {
                    TokenType::Bang => Ok(RoxType::Boolean(!truthy(eval))),
                    TokenType::Minus => {
                        match eval {
                            RoxType::Number(num) => Ok(RoxType::Number(-num)),
                            _ => Err(InterpreterError { error_string: String::from("Unary Expression must be number")})
                        }
                    }
                    _ => Err(InterpreterError { error_string: String::from("Invalid unary expression")})
                }
            }
        }

    }
}

fn truthy(val: RoxType) -> bool {
    match val {
        RoxType::Boolean(b) => b,
        RoxType::Nil => false,
        _ => true
    }
}

fn isEqual(lval: RoxType, rval: RoxType) -> bool {
    match (lval, rval) {
        (RoxType::String(lstr), RoxType::String(rstr)) => lstr == rstr,
        (RoxType::Nil, RoxType::Nil) => true,
        (RoxType::Nil, _) => false,
        (RoxType::Number(lnum), RoxType::Number(rnum)) => lnum == rnum,
        (RoxType::Boolean(lbool), RoxType::Boolean(rbool)) => lbool == rbool,
        (_,_) => false,
    }
}