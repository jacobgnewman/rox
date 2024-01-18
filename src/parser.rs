use crate::rox_type::RoxType;
use crate::token::Token;
use crate::token_type::TokenType::{self, *};

use std::string::String;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

#[derive(Debug)]
pub struct ParseErr {
    pub err_token: Token,
    pub err_msg: String,
}

#[derive(Debug, Clone)]
pub enum Expr {
    Assign(Token, Box<Expr>),
    Binary(Box<Expr>, Token, Box<Expr>),
    Grouping(Box<Expr>),
    Literal(RoxType),
    Unary(Token, Box<Expr>),
    Variable(Token)
}

pub enum Stmt {
    Expression(Box<Expr>),
    Print(Box<Expr>),
    Var(Token, Option<Box<Expr>>)
}


pub type ParseExprResult = Result<Expr, ParseErr>;

impl Parser {

    pub fn new(list: Vec<Token>) -> Parser {
        Parser {
            tokens: list,
            current: 0,
        }
    }

    pub fn parse(&mut self) -> Result<Vec<Stmt>, ParseErr> {
        let mut statements = Vec::new();
        while !self.at_end() {
            statements.push(self.declaration()?)
        }
        Ok(statements)
    }

    fn declaration(&mut self) -> Result<Stmt, ParseErr> {
        let result;
        if self.match_tokens(&[Var]) {
            result = self.var_declaration();
        } else {
            result = self.statement();
        }
        if let Err(e) = result {
            self.synchronize();
            return Err(e)
        }
        result
    }

    fn var_declaration(&mut self) -> Result<Stmt, ParseErr> {
        let name = self.consume(Identifier, "Expect variable name.")?.clone();
        let mut initalizer = None;
        if self.match_tokens(&[Equal]) {
            initalizer = Some(self.expression()?.into());
        }
        Ok(Stmt::Var(name, initalizer))
    }

    fn statement(&mut self) -> Result<Stmt, ParseErr>{
        if self.match_tokens(&[Print]) {
            return self.print_statement()
        }
        self.expression_statement()
    }

    fn print_statement(&mut self) -> Result<Stmt, ParseErr> {
        let val = self.expression()?;
        self.consume(Semicolon, "Expect ';' after value.")?;
        Ok(Stmt::Print(val.into()))
    }

    fn expression_statement(&mut self) -> Result<Stmt, ParseErr> {
        let val = self.expression()?;
        self.consume(Semicolon, "Expect ';' after value.")?;
        Ok(Stmt::Expression(val.into()))
    }

    fn expression(&mut self) -> ParseExprResult {
        self.equality()
    }

    fn assign(&mut self) -> ParseExprResult {
        
    }

    fn equality(&mut self) -> ParseExprResult {
        let mut expr = self.comparison()?;

        while self.match_tokens(&[BangEqual, EqualEqual]) {
            let operator = self.previous().clone();
            let right = self.comparison()?;
            expr = Expr::Binary(expr.into(), operator, right.into());
        }
        Ok(expr)
    }

    fn comparison(&mut self) -> ParseExprResult {
        let mut expr = self.term()?;
        while self.match_tokens(&[Greater, GreaterEqual, Less, LessEqual]) {
            let operator = self.previous().clone();
            let right = self.term()?;
            expr = Expr::Binary(expr.into(), operator, right.into());
        }
        Ok(expr)
    }

    fn term(&mut self) -> ParseExprResult {
        let mut expr = self.factor()?;
        while self.match_tokens(&[Minus, Plus]) {
            let operator = self.previous().clone();
            let right = self.factor()?;
            expr = Expr::Binary(expr.into(), operator, right.into());
        }
        Ok(expr)
    }

    fn factor(&mut self) -> ParseExprResult {
        let mut expr = self.unary()?;
        while self.match_tokens(&[Slash, Star]) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            expr = Expr::Binary(expr.into(), operator, right.into());
        }
        Ok(expr)
    }

    fn unary(&mut self) -> ParseExprResult {
        if self.match_tokens(&[Bang, Minus]) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            return Ok(Expr::Unary(operator, right.into()));
        }
        return self.primary();
    }

    fn primary(&mut self) -> ParseExprResult {
        if self.match_tokens(&[False]) {
            return Ok(Expr::Literal(RoxType::Boolean(true)))
        } 
        if self.match_tokens(&[True]) {
            return Ok(Expr::Literal(RoxType::Boolean(false)))
        } 
        if self.match_tokens(&[Nil]) {
            return Ok(Expr::Literal(RoxType::Nil))
        } 
        if self.match_tokens(&[Number, String]) {
            return Ok(Expr::Literal(self.previous().literal.clone()))
        } 
        if self.match_tokens(&[Identifier]) {
            return Ok(Expr::Variable(self.previous().clone()));
        }
        if self.match_tokens(&[LeftParen]) {
            let expr = self.expression()?;
            let rpar = self.consume(RightParen, "Expect ')' after expression.");
            match rpar {
                Ok(_) => return Ok(Expr::Grouping(expr.into())),
                Err(e) => return Err(e),
            }
        } 

        return Err(ParseErr {
            err_token: self.peek().clone(),
            err_msg: String::from("Expect Expression"),
        })
        
    }

    fn synchronize(&mut self) {
        self.advance();

        while !self.at_end() {
            if self.previous().token_type == Semicolon {
                return;
            }
            if let Return = self.peek().token_type {
                return
            }

            self.advance();
        }
    }

    fn consume(&mut self, token_type: TokenType, message: &str) -> Result<&Token, ParseErr> {
        if self.check(token_type) {
            return Ok(self.advance());
        }
        Err(ParseErr {
            err_token: self.peek().clone(),
            err_msg: String::from(message),
        })
    }

    fn match_tokens(&mut self, tokens: &[TokenType]) -> bool {
        for token_type in tokens {
            if self.check(token_type.clone()) {
                self.advance();
                return true;
            }
        }
        return false;
    }

    fn check(&mut self, token_type: TokenType) -> bool {
        if self.at_end() {
            return false;
        } else {
            return self.peek().token_type == token_type;
        }
    }

    fn advance(&mut self) -> &Token {
        if !self.at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn at_end(&mut self) -> bool {
        self.peek().token_type == EOF
    }

    fn peek(&self) -> &Token {
        self.tokens.get(self.current).unwrap()
    }

    fn previous(&mut self) -> &Token {
        return self.tokens.get(self.current - 1).unwrap();
    }
}
