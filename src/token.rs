use crate::token_type::TokenType;
use std::fmt;
use convert_case::{Case, Casing};

#[derive(Debug)]
pub enum LoxType {
    Text(String),
    Number(f64),
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
        let dttype: String = self.ttype
            .to_string()
            .to_case(Case::ScreamingSnake)
            .replace("LOX_", "");

        write!(f, "{} {} {}",
               dttype, self.lexeme, self.literal)
    }
}

impl fmt::Display for LoxType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LoxType::Number(c) => write!(f, "{:.1}", c),
            LoxType::Text(c) if c.is_empty() => write!(f, "null"),
            LoxType::Text(c) => write!(f, "{}", c),
        }
    }
}
