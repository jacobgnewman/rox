use std::rc::Rc;
use crate::token::Token;
use crate::rox_type::RoxType;

pub enum Expr {
    Binary(Rc<Expr>, Token, Rc<Expr>),
    Literal(RoxType),
    Grouping(Rc<Expr>),
    Unary(Token,Rc<Expr>)
}