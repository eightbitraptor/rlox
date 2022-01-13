use crate::token_type::TokenType;
use crate::token_type::TokenType::*;

use crate::token::Token;

#[derive(Debug)]
pub struct Scanner {
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Default for Scanner {
    fn default() -> Scanner {
        Scanner::new()
    }
}

impl Scanner {
    pub fn new() -> Self {
        let tokens: Vec<Token> = vec!();
        Scanner { tokens, start: 0, current: 0, line: 1 }
    }

    // TODO: Maybe impl Iterator?
    pub fn scan_tokens(mut self, source: String) -> Vec<Token> {
        while !self.end_of_source(&source) {
            self.start = self.current;
            self.scan_token(&source);
        }

        self.tokens
    }

    fn scan_token(&mut self, source: &str) {
        let c = self.advance(source);

        match c {
            Some('(') => self.add_token(LeftParen, source),
            Some(_) => (),
            None => (),
        };
    }

    fn add_token(&mut self, ttype: TokenType, source: &str) {
        let lexeme = String::from(&source[self.start..self.current]);
        let token = Token::new(ttype, lexeme, String::from(""), self.line);

        self.tokens.push(token);
    }

    fn end_of_source(&self, source: &str) -> bool {
        (self.current as usize) >= source.len()
    }



    fn advance(&mut self, source: &str) -> Option<char> {
        let c = source.chars().nth(self.current);
        self.current += 1;

        c
    }
}
