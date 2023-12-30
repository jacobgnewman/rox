use crate::rox_type::RoxType;
use crate::token::Token;
use std::rc::Rc;

#[derive(Debug)]
pub enum Expr {
    Binary(Rc<Expr>, Token, Rc<Expr>),
    Literal(RoxType),
    Grouping(Rc<Expr>),
    Unary(Token, Rc<Expr>)
}

impl Expr {
    fn to_string(&self) -> String {
        match self {
            Expr::Binary(x, t, y) => expr_paren(t.lexeme.clone(), vec![x, y]),
            Expr::Literal(x) => x.to_string(),
            Expr::Grouping(x) => format!("(group {})", x.to_string()),
            Expr::Unary(t, x) => format!("({} {})", t.lexeme, x.to_string()),
        }
    }
}

fn expr_paren(name: String, exprs: Vec<&Rc<Expr>>) -> String {
    let mut paren = format!("({}", name);
    for exp in exprs {
        paren.push_str(" ");
        paren.push_str(&exp.to_string());
    }
    paren.push_str(")");
    return paren;
}
