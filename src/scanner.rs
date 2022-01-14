use crate::token_type::TokenType;
use crate::token_type::TokenType::*;
use crate::error::*;

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
            match self.scan_token(&source) {
                Ok(()) => (),
                Err(e) => println!("{}", e),
            }
        }

        self.tokens
    }

    fn scan_token(&mut self, source: &str) -> LoxResult {
        let c = self.advance(source);
        let mut error = None;

        println!("scanning token : {:?}", &c);
        match c {
            Some('(') => self.add_token(LeftParen, source),
            Some(')') => self.add_token(RightParen, source),
            Some('{') => self.add_token(LeftBrace, source),
            Some('}') => self.add_token(RightBrace, source),
            Some(',') => self.add_token(Comma, source),
            Some('.') => self.add_token(Dot, source),
            Some('-') => self.add_token(Minus, source),
            Some('+') => self.add_token(Plus, source),
            Some(';') => self.add_token(Semicolon, source),
            Some('*') => self.add_token(Star, source),
            Some(_) => {
                error = Some(LoxError {
                    line: self.line as i32,
                    place: String::from(""),
                    message: String::from("Invalid Character")
                });
            },
            None => (),
        };

        if let Some(e) = error {
            Err(e)
        } else {
            Ok(())
        }
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

#[cfg(test)]
mod tests {
    use super::Scanner;
    use crate::token_type::TokenType;
    use crate::token_type::TokenType::*;

    #[test]
    fn test_scan_tokens_all_single_character_tokens() {
        assert!(token_scanned("(", LeftParen));
        assert!(token_scanned(")", RightParen));
        assert!(token_scanned("{", LeftBrace));
        assert!(token_scanned("}", RightBrace));
        assert!(token_scanned(",", Comma));
        assert!(token_scanned(".", Dot));
        assert!(token_scanned("-", Minus));
        assert!(token_scanned("+", Plus));
        assert!(token_scanned(";", Semicolon));
        assert!(token_scanned("*", Star));
    }

    #[test]
    fn test_scan_token_invalid_token_returns_err() {
        let bad_tokens = Scanner::new().scan_token("?");
        assert!(bad_tokens.is_err());
        assert_eq!(bad_tokens.unwrap_err().message, "Invalid Character");
    }

    fn token_scanned(value: &str, ttype: TokenType) -> bool {
        let tokens = Scanner::new().scan_tokens(String::from(value));
        tokens[0].ttype == ttype
    }
}
