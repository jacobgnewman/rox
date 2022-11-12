use std::rc::Rc;
use crate::token::Token;
use crate::rox_type::RoxType;

pub enum Expr {
    Binary(Rc<Expr>, Token, Rc<Expr>),
    Literal(RoxType),
    Grouping(Rc<Expr>),
    Unary(Token,Rc<Expr>)
}

impl Expr{

    fn accept(&self) {
        match *self {
            Expr::Binary(x, t, y) => String::from("a"),
            _ => ()
        }
    }
    fn to_string(&self) -> String{
        match *self {
            Expr::Binary(x, t, y) => expr_paren(t.lexeme, vec![x,y]),

            _ => panic!("Extreme error")
        }

    }
}

fn expr_paren(name: String, exprs: Vec<Rc<Expr>>) -> String {
    let mut paren = format!("({} ", name);
    for exp in exprs {
        paren.push_str(expr.)
    }



    return  paren;
}