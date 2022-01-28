use crate::token_type::TokenType;
use std::fmt;

#[derive(Debug)]
pub enum LoxType {
    Text(String),
    Number(f64),
    None,
}

#[derive(Debug)]
pub struct Token {
    pub ttype: TokenType,
    pub lexeme: String,
    // The original book uses Object for this. Idk how to make an equivalent in Rust yet
    pub literal: LoxType,
    pub line: usize,
}

impl Token {
    pub fn new(ttype: TokenType, lexeme: String, literal: LoxType, line: usize) -> Self {
        Token { ttype, lexeme, literal, line }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.ttype, self.lexeme, self.literal)
    }
}

impl fmt::Display for LoxType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LoxType::Number(c) => {
                if c.fract() == 0.0 {
                    write!(f, "{:.1}", c)
                } else {
                    write!(f, "{}", c)
                }
            }
            LoxType::Text(c) => write!(f, "{}", c),
            LoxType::None => write!(f, "null"),
        }
    }
}
