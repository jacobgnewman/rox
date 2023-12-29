use std::any::Any;

use crate::token::Token;
use crate::token_type::TokenType::{*, self};
use crate::expr::Expr;
use crate::rox_type::RoxType;


pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(list: Vec<Token>) -> Parser {
        Parser { tokens: list, current: 0 }
    }

    fn expression(&mut self) -> Expr {
        self.equality()
    }

    fn equality(&mut self) -> Expr {
        let mut expr = self.comparison();

        while self.match_tokens(&[BangEqual, EqualEqual])  {
            let operator = self.previous().clone();
            let right = self.comparison();
            expr = Expr::Binary(expr.into(), operator, right.into());
        }
        expr
    }

    fn comparison(&mut self) -> Expr {
        let mut expr = self.term();
        while self.match_tokens(&[Greater, GreaterEqual, Less, LessEqual]) {
            let operator = self.previous().clone();
            let right = self.term();
            expr = Expr::Binary(expr.into(), operator, right.into());
        }
        expr
    }

    fn term(&mut self) -> Expr {
        let mut expr = self.factor();
        while self.match_tokens(&[Minus, Plus]) {
            let operator = self.previous().clone();
            let right = self.factor();
            expr = Expr::Binary(expr.into(), operator, right.into());
        }
        expr
    }

    fn factor(&mut self) -> Expr {
        let mut expr = self.unary();
        while self.match_tokens(&[Slash, Star]) {
            let operator = self.previous().clone();
            let right = self.unary();
            expr = Expr::Binary(expr.into(), operator, right.into());
        }
        expr
    }
    
    fn unary(&mut self) -> Expr {
        if self.match_tokens(&[Bang, Minus]) {
            let operator = self.previous().clone();
            let right = self.unary();
            return Expr::Unary(operator, right.into());
        }
        return self.primary();
    }

    fn primary(&mut self) -> Expr {
        if self.match_tokens(&[False]) {
            return Expr::Literal(RoxType::Boolean(true))
        }
        if self.match_tokens(&[True]) {
            return Expr::Literal(RoxType::Boolean(false))
        }
        if self.match_tokens(&[Nil]) {
            return Expr::Literal(RoxType::Nil)
        }
        if self.match_tokens(&[Number, String]){
            return Expr::Literal(self.previous().literal.clone())
        }

        if self.match_tokens(&[LeftParen]) {
            let expr = self.expression();
            self.consume(RightParen, "Expect ')' after expression.");
            return Expr::Grouping(expr.into());
        }

        panic!("failed to match token")
    }

    fn synchronize(&mut self) {
        self.advance();
        
        while !self.at_end() {
            if self.previous().token_type == Semicolon {
                return;
            }
            match self.peek().token_type {
                Return => {return},
                _ => ()
            }

            self.advance();
        }
    }

    fn consume(&mut self, token_type: TokenType, message: &str) -> &Token {
        if self.check(token_type) {
            return self.advance()
        }
        self.error(self.peek(), message);
        panic!("error handle later") // not sure what to do here
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

    fn error(&self, token: &Token, message: &str) {
        println!("Error: {} \n Token: {}", message, token);
    }


    fn check(&mut self, token_type: TokenType) -> bool {
        if self.at_end() {
            return false
        } else {
            return self.peek().token_type == token_type
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

    fn previous(&mut self) -> & Token {
        return self.tokens.get(self.current - 1).unwrap()
    }
}