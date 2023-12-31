use crate::rox_type::RoxType;
use crate::token::Token;

#[derive(Debug, Clone)]
pub enum Expr {
    Binary(Box<Expr>, Token, Box<Expr>),
    Grouping(Box<Expr>),
    Literal(RoxType),
    Unary(Token, Box<Expr>)
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

fn expr_paren(name: String, exprs: Vec<&Box<Expr>>) -> String {
    let mut paren = format!("({}", name);
    for exp in exprs {
        paren.push_str(" ");
        paren.push_str(&exp.to_string());
    }
    paren.push_str(")");
    return paren;
}
