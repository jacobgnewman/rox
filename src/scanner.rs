use crate::error;
use crate::rox_type::RoxType;
use crate::token::Token;

use crate::token_type::TokenType;
use std::collections::HashMap;


/* Notes
- start current & line are usize, max ideal input?
*/

pub struct Scanner {
    source: Vec<char>,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: u32,
}

lazy_static!{
    static ref IDENTIFIERS : HashMap<&'static str, TokenType> = HashMap::from([
        ("and",    TokenType::And),
        ("class",  TokenType::Class),
        ("else",   TokenType::Else),
        ("false",  TokenType::False),
        ("for",    TokenType::For),
        ("fun",    TokenType::Fun),
        ("if",     TokenType::If),
        ("nil",    TokenType::Nil),
        ("or",     TokenType::Or),
        ("print",  TokenType::Print),
        ("return", TokenType::Return),
        ("super",  TokenType::Super),
        ("this",   TokenType::This),
        ("true",   TokenType::True),
        ("var",    TokenType::Var),
        ("while",  TokenType::While),
    ]);
}

impl Scanner {
    pub fn new(source: &String) -> Scanner {
        Scanner {
            source: source.chars().collect::<Vec<char>>(),
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) -> Vec<Token> {
        while !self.at_end() {
            self.start = self.current;
            self.scan_token()
        }

        self.tokens
            .push(Token::new(TokenType::EOF, "".to_string(), RoxType::Nil, 23));

        return self.tokens[..].to_vec();
    }

    fn scan_token(&mut self) {
        let c = self.advance();
        match c {
            '(' => self.add(TokenType::LeftParen),
            ')' => self.add(TokenType::RightParen),
            '{' => self.add(TokenType::LeftBrace),
            '}' => self.add(TokenType::RightBrace),
            ',' => self.add(TokenType::Comma),
            '.' => self.add(TokenType::Dot),
            '-' => self.add(TokenType::Minus),
            '+' => self.add(TokenType::Plus),
            ';' => self.add(TokenType::Semicolon),
            '*' => self.add(TokenType::Star),
            '!' => {
                if self.match_next('=') {
                    self.add(TokenType::BangEqual)
                } else {
                    self.add(TokenType::Bang)
                }
            }
            '=' => {
                if self.match_next('=') {
                    self.add(TokenType::EqualEqual)
                } else {
                    self.add(TokenType::Equal)
                }
            }
            '<' => {
                if self.match_next('=') {
                    self.add(TokenType::LessEqual)
                } else {
                    self.add(TokenType::Less)
                }
            }
            '>' => {
                if self.match_next('=') {
                    self.add(TokenType::GreaterEqual)
                } else {
                    self.add(TokenType::Greater)
                }
            }
            '/' => {
                if self.match_next('/') {
                    // comment, consume line
                    while self.peek() != '\n' && !self.at_end() {
                        self.advance();
                    }
                } else {
                    self.add(TokenType::Slash)
                }
            }
            ' ' => (),
            '\r' => (),
            '\t' => (),
            '\n' => self.line += 1,
            '"' => self.scn_string(),
            '0'..='9' => self.scn_number(),

            other => {
                if is_alphanumeric(other) {
                    self.scn_identifier()
                } else {
                    error(self.line, "Unexpected character")
                }
            }
        }
    }

    // Used for non literal symbols
    fn add(&mut self, tokentype: TokenType) {
        self.add_token(tokentype, RoxType::Nil)
    }

    fn add_token(&mut self, token_type: TokenType, literal: RoxType) {
        let text = self.source[self.start..self.current]
            .iter()
            .collect::<String>();
        self.tokens
            .push(Token::new(token_type, text, literal, self.line))
    }

    fn scn_identifier(&mut self) {
        while is_alphanumeric(self.peek()) {
            self.advance();
        }

        let identifier = self.source[self.start..self.current]
            .iter()
            .collect::<String>();
        let id = identifier.as_str();

        match IDENTIFIERS.get(id){
            Some(token_type) => self.add(token_type.clone()), //clone nessicary?
            None => self.add(TokenType::Identifier),
        }
    }

    fn scn_string(&mut self) {
        while self.peek() != '"' && !self.at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.at_end() {
            error(self.line, "unterminated string");
            return;
        }

        self.advance(); // consume '"'

        let string_lit = self.source[self.start + 1..self.current - 1]
            .iter()
            .collect::<String>();

        let literal = RoxType::String(string_lit);
        self.add_token(TokenType::String, literal)
    }

    // Identifies a number lexeme
    fn scn_number(&mut self) {
        while is_digit(self.peek()) {
            self.advance();
        }

        // look for fractional part past '.'
        if self.peek() == '.' && is_digit(self.peek_next()) {
            self.advance();
            while is_digit(self.peek()) {
                self.advance();
            }
        }

        // TODO: error handle
        let number = self.source[self.start..self.current]
            .iter()
            .collect::<String>()
            .parse::<f64>()
            .unwrap();
        self.add_token(TokenType::Number, RoxType::Number(number))
    }

    // Advances if the character matches expected
    fn match_next(&mut self, expected: char) -> bool {
        if self.at_end() {
            return false;
        }
        if self.source[self.current] != expected {
            return false;
        }
        self.current += 1;
        true
    }

    // moves current +1 and returns character at current -1
    fn advance(&mut self) -> char {
        self.current += 1; //increment position, move to match?
        self.source[self.current - 1]
    }

    fn peek(&self) -> char {
        if self.at_end() {
            return '\0';
        } else {
            return self.source[self.current];
        }
    }
    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0';
        } else {
            return self.source[self.current + 1];
        }
    }

    fn at_end(&self) -> bool {
        return self.current >= self.source.len();
    }
}

fn is_alphanumeric(ch: char) -> bool {
    return is_alpha(ch) || is_digit(ch);
}

fn is_alpha(ch: char) -> bool {
    return ('a' <= ch && ch <= 'z') || ('A' <= ch && ch <= 'z') || (ch == '_');
}

fn is_digit(ch: char) -> bool {
    return '0' <= ch && ch <= '9';
}
