use std::fmt;
use crate::rox_type::RoxType;

use crate::token_type::TokenType;

#[derive(Clone)]
pub struct Token {
    token_type: TokenType,
    lexeme: String,
    line: u32,
    literal: RoxType,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, literal: RoxType, line: u32) -> Token {
        Token {
            token_type,
            lexeme,
            line,
            literal,
        }
    }
}

// print out for tokens
impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?} {} {:#?}", self.token_type, self.lexeme, self.literal)
    }
}