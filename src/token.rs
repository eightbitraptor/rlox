use crate::token_type::TokenType;

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
