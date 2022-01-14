use crate::token_type::TokenType;
use crate::token_type::TokenType::*;
use crate::error::*;

use crate::token::{Token, LoxType};

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
    pub fn scan_tokens(&mut self, source: String) -> &Vec<Token> {
        while !self.end_of_source(&source) {
            self.start = self.current;
            match self.scan_token(&source) {
                Ok(()) => (),
                Err(e) => println!("{}", e),
            }
        }

        &self.tokens
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
            },

            // Whitespace
            Some(c) if c.is_whitespace() => {
                if c == '\n' { self.line+=1 };
            },

            // Types
            Some('"') => {
                match self.string(source) {
                    Ok(v) => self.add_token_stringish(LoxString, v, source),
                    Err(e) => error = Some(e),
                }
            },
            Some(d) if d.is_numeric() => {
                match self.number(source) {
                    Ok(v) => self.add_token_numeric(Number, v, source),
                    Err(e) => error = Some(e),
                }
            },
            Some(c) if c.is_alphabetic() || c == '_' => {
                match self.identifier(source) {
                    Ok(v) => self.add_token_stringish(self.keywords(&v).unwrap(), v, source),
                    Err(e) => error = Some(e),
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

    fn string(&mut self, source: &str) -> Result<String, LoxError> {
        while !self.peek(source).contains(&'"') && !self.end_of_source(source) {
            if self.peek(source).contains(&'\n') { self.line+= 1 }
            self.advance(source);
        }

        if self.end_of_source(source) {
            return Err(LoxError { line: self.line as i32,
                                  place: String::from(""),
                                  message: String::from("Unterminated String")});
        }

        self.advance(source);

        Ok(String::from(&source[self.start+1..self.current-1]))
    }

    fn number(&mut self, source: &str) -> Result<f64, LoxError> {
        while self.peek(source)
            .ok_or(LoxError { line: self.line as i32,
                              place: String::from(""),
                              message: String::from("")})?
            .is_numeric() {
                self.advance(source);
            }

        if self.peek(source).contains(&'.') && self.peek_next(source).unwrap().is_numeric() {
            self.advance(source);

            while self.peek(source)
                .ok_or(LoxError { line: self.line as i32,
                                  place: String::from(""),
                                  message: String::from("")})?
                .is_numeric() {
                    self.advance(source);
                }
        }

        Ok(String::from(&source[self.start..self.current]).parse().unwrap())
    }



    fn identifier(&mut self, source: &str) -> Result<String, LoxError> {
        while is_alphanumeric(&self.peek(source)
            .ok_or(LoxError { line: self.line as i32,
                                  place: String::from(""),
                                  message: String::from("")})?) {
            self.advance(source);
        }

        Ok(String::from(&source[self.start..self.current]))
    }

    fn add_token_stringish(&mut self, ttype: TokenType, literal: String, source: &str) {
        let literal = LoxType::Text(literal);
        let lexeme = String::from(&source[self.start..self.current]);
        let token = Token::new(ttype, lexeme, literal, self.line);

        self.tokens.push(token);
    }

    fn add_token_numeric(&mut self, ttype: TokenType, literal: f64, source: &str) {
        let literal = LoxType::Number(literal);
        let lexeme = String::from(&source[self.start..self.current]);
        let token = Token::new(ttype, lexeme, literal, self.line);

        self.tokens.push(token);
    }

    fn add_token(&mut self, ttype: TokenType, source: &str) {
        let lexeme = String::from(&source[self.start..self.current]);
        let token = Token::new(ttype, lexeme, LoxType::Text(String::from("")), self.line);

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

    fn peek_next(&self, source: &str) -> Option<char> {
        if self.current + 1 as usize >= source.len() {
            Some('\0')
        } else {
            source.chars().nth(self.current + 1)
        }
    }

    fn keywords(&self, i: &str) -> Result<TokenType, LoxError> {
        match i {
            "and" => Ok(And),
            "class" => Ok(Class),
            "else" => Ok(Else),
            "false" => Ok(False),
            "for" => Ok(For),
            "fun" => Ok(Fun),
            "if" => Ok(If),
            "nil" => Ok(Nil),
            "or" => Ok(Or),
            "print" => Ok(Print),
            "return" => Ok(Return),
            "super" => Ok(Super),
            "this" => Ok(This),
            "true" => Ok(True),
            "var" => Ok(Var),
            "while" => Ok(While),
            _ => Ok(Identifier),
        }
    }
}

fn is_alpha(c: &char) -> bool {
    c.is_alphabetic() || c == &'_'
}

fn is_alphanumeric(c: &char) -> bool {
    is_alpha(c) || c.is_numeric()
}



#[cfg(test)]
mod tests {
    use super::Scanner;
    use crate::token::LoxType;
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
        let mut scanner = Scanner::new();
        let tokens = scanner
            .scan_tokens(
                String::from("// Comments are ignored")
            );
        assert!(tokens.is_empty());
    }

    #[test]
    fn test_scan_tokens_seperated_by_whitespace() {
        let mut scanner = Scanner::new();
        let tokens = scanner.scan_tokens(String::from("! \t*"));
        assert_eq!(2, tokens.len());
        assert_eq!(Bang, tokens[0].ttype);
        assert_eq!(Star, tokens[1].ttype);
    }

    #[test]
    fn test_scan_tokens_seperated_by_newlines_increments_line() {
        let mut scanner = Scanner::new();
        let tokens = scanner.scan_tokens(String::from("!\n*"));
        assert_eq!(2, tokens.len());
        assert_eq!(2, scanner.line)
    }

    #[test]
    fn test_scan_tokens_strings() {
        let mut scanner = Scanner::new();
        let tokens = scanner.scan_tokens(String::from("\"Lox Strings are double quoted\""));
        assert_eq!(1, tokens.len());
        assert_eq!(LoxString, tokens[0].ttype);

        match &tokens[0].literal {
            LoxType::Text(s) => assert_eq!("Lox Strings are double quoted", s),
            _ => { panic!("unexpected variant in LoxType") },
        }
    }

    #[test]
    fn test_scan_tokens_strings_with_newlines() {
        let mut scanner = Scanner::new();
        let tokens = scanner.scan_tokens(String::from("\"Lox Strings are\n double quoted\""));
        assert_eq!(1, tokens.len());
        assert_eq!(LoxString, tokens[0].ttype);

        match &tokens[0].literal {
            LoxType::Text(s) => assert_eq!("Lox Strings are\n double quoted", s),
            _ => { panic!("unexpected variant in LoxType"); },
        }
        assert_eq!(2, scanner.line);
    }

    #[test]
    fn test_scan_tokens_strings_with_valid_tokens_within() {
        let mut scanner = Scanner::new();
        let tokens = scanner.scan_tokens(String::from("\"Lox *Strings* are\n -double- quoted\""));
        assert_eq!(1, tokens.len());
        assert_eq!(LoxString, tokens[0].ttype);

        match &tokens[0].literal {
            LoxType::Text(s) => assert_eq!("Lox *Strings* are\n -double- quoted", s),
            _ => { panic!("unexpected variant in LoxType"); },
        }
        assert_eq!(2, scanner.line);
    }

    #[test]
    fn test_scan_tokens_numbers() {
        let mut scanner = Scanner::new();
        let tokens = scanner.scan_tokens(String::from("123"));
        assert_eq!(1, tokens.len());
        assert_eq!(Number, tokens[0].ttype);

        match &tokens[0].literal {
            LoxType::Number(s) => assert_eq!(123_f64, *s),
            _ => { panic!("unexpected variant in LoxType") },
        }
    }

    #[test]
    fn test_scan_tokens_floating_point_numbers() {
        let mut scanner = Scanner::new();
        let tokens = scanner.scan_tokens(String::from("123.456"));
        assert_eq!(1, tokens.len());
        assert_eq!(Number, tokens[0].ttype);

        match &tokens[0].literal {
            LoxType::Number(s) => assert_eq!(123.456, *s),
            _ => { panic!("unexpected variant in LoxType") },
        }
    }

    #[test]
    fn test_scan_tokens_identifiers_and_keywords() {
        let mut scanner = Scanner::new();
        let tokens = scanner.scan_tokens(String::from("fun function_name"));
        assert_eq!(2, tokens.len());
        assert_eq!(Fun, tokens[0].ttype);
        assert_eq!(Identifier, tokens[1].ttype);

        match &tokens[0].literal {
            LoxType::Text(s) => assert_eq!("fun", s),
            _ => { panic!("unexpected variant in LoxType") },
        }

        match &tokens[1].literal {
            LoxType::Text(s) => assert_eq!("function_name", s),
            _ => { panic!("unexpected variant in LoxType") },
        }
    }

    #[test]
    fn test_scan_token_invalid_token_returns_err() {
        let bad_tokens = Scanner::new().scan_token("?");
        assert!(bad_tokens.is_err());
        assert_eq!(bad_tokens.unwrap_err().message, "Invalid Character");
    }

    fn token_scanned(value: &str, ttype: TokenType) -> bool {
        let mut scanner = Scanner::new();
        let tokens = scanner.scan_tokens(String::from(value));
        tokens[0].ttype == ttype
    }
}
