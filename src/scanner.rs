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

    pub fn scan_tokens(&mut self, source: String) -> &Vec<Token> {
        while !self.end_of_source(&source) {
            self.start = self.current;
            match self.scan_token(&source) {
                Ok(()) => (),
                Err(e) => println!("{}", e),
            }
        }

        self.add_token(EOF, &source).expect("");

        &self.tokens
    }

    fn scan_token(&mut self, source: &str) -> LoxResult<()> {
        let c = self.advance(source);

        match c {
            Some('(') => self.add_token(LEFT_PAREN, source),
            Some(')') => self.add_token(RIGHT_PAREN, source),
            Some('{') => self.add_token(LEFT_BRACE, source),
            Some('}') => self.add_token(RIGHT_BRACE, source),
            Some(',') => self.add_token(COMMA, source),
            Some('.') => self.add_token(DOT, source),
            Some('-') => self.add_token(MINUS, source),
            Some('+') => self.add_token(PLUS, source),
            Some(';') => self.add_token(SEMICOLON, source),
            Some('*') => self.add_token(STAR, source),

            // These tokens consist of two specific characters
            Some('!') if self.match_next('=', source) => self.add_token (BANG_EQUAL, source),
            Some('!') => self.add_token(BANG, source),

            Some('=') if self.match_next('=', source) => self.add_token(EQUAL_EQUAL, source),
            Some('=') => self.add_token(EQUAL, source),

            Some('<') if self.match_next('=', source)=> self.add_token(LESS_EQUAL, source),
            Some('<') => self.add_token(LESS, source),

            Some('>') if self.match_next('=', source) => self.add_token(GREATER_EQUAL, source),
            Some('>') => self.add_token(GREATER, source),

            // tokens with peekahead
            Some('/') => {
                if self.match_next('/', source) {
                    while !self.peek(source).contains(&'\n')  && !self.end_of_source(source) {
                        self.advance(source);
                    }
                    Ok(())
                } else {
                    self.add_token(SLASH, source)
                }
            },

            // Whitespace
            Some(c) if c.is_whitespace() => {
                if c == '\n' { self.line+=1 };
                Ok(())
            },

            // Types
            Some('"') => {
                self.string(source)
                    .and_then(|v| self.add_token_stringish(STRING, v, source))
            },
            Some(d) if d.is_numeric() => {
                self.number(source)
                    .and_then(|v| self.add_token_numeric(NUMBER, v, source))
            },
            Some(c) if c.is_alphabetic() || c == '_' => {
                self.identifier(source)
                    .and_then(|v| self.add_token_stringish(self.keywords(&v).unwrap(), v, source))
            }

            // Defaults, and unknowns
            Some(_) => { Err(LoxError::new(self.line as i32, "Invalid Character")) },
            None => Ok(()),
        }
    }

    fn string(&mut self, source: &str) -> LoxResult<String> {
        while !self.peek(source).contains(&'"') && !self.end_of_source(source) {
            if self.peek(source).contains(&'\n') { self.line+= 1 }
            self.advance(source);
        }

        if self.end_of_source(source) {
            return Err(LoxError::new(self.line as i32, "Unterminated String"));
        }

        self.advance(source);

        Ok(String::from(&source[self.start+1..self.current-1]))
    }

    fn number(&mut self, source: &str) -> LoxResult<f64> {
        while self.peek(source)
            .ok_or(LoxError::new(self.line as i32, "Invalid peek into source"))?
            .is_numeric() {
                self.advance(source);
            }

        if self.peek(source).contains(&'.') && self.peek_next(source).unwrap().is_numeric() {
            self.advance(source);

            while self.peek(source)
                .ok_or(LoxError::new(self.line as i32, "Invalid peek into source"))?
                .is_numeric() {
                    self.advance(source);
                }
        }

        Ok(String::from(&source[self.start..self.current]).parse().unwrap())
    }

    fn identifier(&mut self, source: &str) -> LoxResult<String> {
        while is_alphanumeric(
            &self.peek(source).ok_or(LoxError::new(self.line as i32, "Invalid peek into source"))?
        ) {
            self.advance(source);
        }

        Ok(String::from(&source[self.start..self.current]))
    }

    fn add_token_stringish(&mut self, ttype: TokenType, literal: String, source: &str) -> LoxResult<()> {
        let literal = match ttype {
            TokenType::STRING => LoxType::Text(literal),
            _ => LoxType::None,
        };
        let lexeme = String::from(&source[self.start..self.current]);
        let token = Token::new(ttype, lexeme, literal, self.line);

        self.tokens.push(token);
        Ok(())
    }

    fn add_token_numeric(&mut self, ttype: TokenType, literal: f64, source: &str) -> LoxResult<()> {
        let literal = LoxType::Number(literal);
        let lexeme = String::from(&source[self.start..self.current]);
        let token = Token::new(ttype, lexeme, literal, self.line);

        self.tokens.push(token);
        Ok(())
    }

    fn add_token(&mut self, ttype: TokenType, source: &str) -> LoxResult<()> {
        let lexeme = match ttype {
            TokenType::EOF => String::from(""),
            _ => String::from(&source[self.start..self.current])
        };
        let token = Token::new(ttype, lexeme, LoxType::None, self.line);

        self.tokens.push(token);
        Ok(())
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

    fn keywords(&self, i: &str) -> LoxResult<TokenType> {
        match i {
            "and" => Ok(AND),
            "class" => Ok(CLASS),
            "else" => Ok(ELSE),
            "false" => Ok(FALSE),
            "for" => Ok(FOR),
            "fun" => Ok(FUN),
            "if" => Ok(IF),
            "nil" => Ok(NIL),
            "or" => Ok(OR),
            "print" => Ok(PRINT),
            "return" => Ok(RETURN),
            "super" => Ok(SUPER),
            "this" => Ok(THIS),
            "true" => Ok(TRUE),
            "var" => Ok(VAR),
            "while" => Ok(WHILE),
            _ => Ok(IDENTIFIER),
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
        assert!(token_scanned("(", LEFT_PAREN));
        assert!(token_scanned(")", RIGHT_PAREN));
        assert!(token_scanned("{", LEFT_BRACE));
        assert!(token_scanned("}", RIGHT_BRACE));
        assert!(token_scanned(",", COMMA));
        assert!(token_scanned(".", DOT));
        assert!(token_scanned("-", MINUS));
        assert!(token_scanned("+", PLUS));
        assert!(token_scanned(";", SEMICOLON));
        assert!(token_scanned("*", STAR));
        assert!(token_scanned("/", SLASH));
    }

    #[test]
    fn test_scan_tokens_two_character_tokens() {
        assert!(token_scanned("!=", BANG_EQUAL));
        assert!(token_scanned("==", EQUAL_EQUAL));
        assert!(token_scanned("<=", LESS_EQUAL));
        assert!(token_scanned(">=", GREATER_EQUAL));
    }

    #[test]
    fn test_scan_tokens_slash_with_following_chars() {
        assert!(token_scanned("/foo", SLASH));
    }

    #[test]
    fn test_scan_tokens_slash_with_following_slash_is_a_comment() {
        let mut scanner = Scanner::new();
        let tokens = scanner
            .scan_tokens(
                String::from("// Comments are ignored")
            );
        assert_eq!(1, tokens.len());
    }

    #[test]
    fn test_scan_tokens_seperated_by_whitespace() {
        let mut scanner = Scanner::new();
        let tokens = scanner.scan_tokens(String::from("! \t*"));
        assert_eq!(3, tokens.len());
        assert_eq!(BANG, tokens[0].ttype);
        assert_eq!(STAR, tokens[1].ttype);
    }

    #[test]
    fn test_scan_tokens_seperated_by_newlines_increments_line() {
        let mut scanner = Scanner::new();
        let tokens = scanner.scan_tokens(String::from("!\n*"));
        assert_eq!(3, tokens.len());
        assert_eq!(2, scanner.line)
    }

    #[test]
    fn test_scan_tokens_strings() {
        let mut scanner = Scanner::new();
        let tokens = scanner.scan_tokens(String::from("\"Lox Strings are double quoted\""));
        assert_eq!(2, tokens.len());
        assert_eq!(STRING, tokens[0].ttype);

        match &tokens[0].literal {
            LoxType::Text(s) => assert_eq!("Lox Strings are double quoted", s),
            _ => { panic!("unexpected variant in LoxType") },
        }
    }

    #[test]
    fn test_scan_tokens_strings_with_newlines() {
        let mut scanner = Scanner::new();
        let tokens = scanner.scan_tokens(String::from("\"Lox Strings are\n double quoted\""));
        assert_eq!(2, tokens.len());
        assert_eq!(STRING, tokens[0].ttype);

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
        assert_eq!(2, tokens.len());
        assert_eq!(STRING, tokens[0].ttype);

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
        assert_eq!(2, tokens.len());
        assert_eq!(NUMBER, tokens[0].ttype);

        match &tokens[0].literal {
            LoxType::Number(s) => assert_eq!(123_f64, *s),
            _ => { panic!("unexpected variant in LoxType") },
        }
    }

    #[test]
    fn test_scan_tokens_floating_point_numbers() {
        let mut scanner = Scanner::new();
        let tokens = scanner.scan_tokens(String::from("123.456"));
        assert_eq!(2, tokens.len());
        assert_eq!(NUMBER, tokens[0].ttype);

        match &tokens[0].literal {
            LoxType::Number(s) => assert_eq!(123.456, *s),
            _ => { panic!("unexpected variant in LoxType") },
        }
    }

    #[test]
    fn test_scan_tokens_identifiers_and_keywords() {
        let mut scanner = Scanner::new();
        let tokens = scanner.scan_tokens(String::from("fun function_name"));
        assert_eq!(3, tokens.len());
        assert_eq!(FUN, tokens[0].ttype);
        assert_eq!(IDENTIFIER, tokens[1].ttype);

        match &tokens[0].literal {
            LoxType::None => assert!(true),
            _ => { panic!("unexpected variant in LoxType") },
        }

        match &tokens[1].literal {
            LoxType::None => assert!(true),
            _ => { panic!("unexpected variant in LoxType") },
        }
    }

    #[test]
    fn test_scan_token_invalid_token_returns_err() {
        let bad_tokens = Scanner::new().scan_token("?");
        assert!(bad_tokens.is_err());
        assert_eq!(bad_tokens.unwrap_err().message.unwrap(), "Invalid Character");
    }

    fn token_scanned(value: &str, ttype: TokenType) -> bool {
        let mut scanner = Scanner::new();
        let tokens = scanner.scan_tokens(String::from(value));
        tokens[0].ttype == ttype
    }
}
