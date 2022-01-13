use crate::token_type::TokenType;

#[derive(Debug)]
pub struct Token {
    ttype: TokenType,
    lexeme: String,
    // The original book uses Object for this. Idk how to make an equivalent in Rust yet
    literal: String,
    line: usize,
}

impl Token {
    pub fn new(ttype: TokenType, lexeme: String, literal: String, line: usize) -> Self {
        Token { ttype, lexeme, literal, line }
    }
}
