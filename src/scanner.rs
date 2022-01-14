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

            // These tokens consist of two specific characters
            Some('!') => {
                let token = if self.match_next('=', source) { BangEqual } else { Bang };
                self.add_token(token, source)
            },
            Some('=') => {
                let token = if self.match_next('=', source) { EqualEqual } else { Equal };
                self.add_token(token, source)
            },
            Some('<') => {
                let token = if self.match_next('=', source) { LessEqual } else { Less };
                self.add_token(token, source)
            },
            Some('>') => {
                let token = if self.match_next('=', source) { GreaterEqual } else { Greater };
                self.add_token(token, source)
            },

            // tokens with peekahead
            Some('/') => {
                if self.match_next('/', source) {
                    while !self.peek(source).contains(&'\n')  && !self.end_of_source(source) {
                        self.advance(source);
                    }
                } else {
                    self.add_token(Slash, source);
                }
            }

            // Defaults, and unknowns
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

    // TODO: I feel there should be a more idiomatic way of doing this
    fn match_next(&mut self, m: char,  source: &str) -> bool {
        if self.end_of_source(source) { return false; };
        if source.chars().nth(self.current) != Some(m) { return false };

        self.current += 1;
        true
    }

    fn advance(&mut self, source: &str) -> Option<char> {
        let c = source.chars().nth(self.current);
        self.current += 1;

        c
    }

    fn peek(&self, source: &str) -> Option<char> {
        if self.end_of_source(source) {
            Some('\0')
        } else {
            source.chars().nth(self.current)
        }
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
        assert!(token_scanned("/", Slash));
    }

    #[test]
    fn test_scan_tokens_two_character_tokens() {
        assert!(token_scanned("!=", BangEqual));
        assert!(token_scanned("==", EqualEqual));
        assert!(token_scanned("<=", LessEqual));
        assert!(token_scanned(">=", GreaterEqual));
    }

    #[test]
    fn test_scan_tokens_slash_with_following_chars() {
        assert!(token_scanned("/foo", Slash));
    }

    #[test]
    fn test_scan_tokens_slash_with_following_slash_is_a_comment() {
        let tokens = Scanner::new()
            .scan_tokens(
                String::from("// Comments are ignored")
            );
        assert!(tokens.is_empty());
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
