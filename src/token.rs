use crate::token_type::TokenType;
use std::fmt;

pub trait LoxType: fmt::Display + fmt::Debug {
    fn lox_print(&self) -> String;
}

impl LoxType for String {
    fn lox_print(&self) -> String {
        self.to_string()
    }
}

impl LoxType for f64 {
    fn lox_print(&self) -> String {
        match self.fract() {
            x if x > 0.0 => format!("{}", self),
            _ =>  format!("{:.1}", self),
        }
    }
}

#[derive(Debug)]
pub struct Token {
    pub ttype: TokenType,
    pub lexeme: String,
    // Replace the use of Java's Object, with an object stored in the
    // heap that implements a trait. This way we can implement LoxType
    // for all types we end up using.
    pub literal: Box<dyn LoxType>,
    pub line: usize,
}

impl Token {
    pub fn new(ttype: TokenType, lexeme: String, literal: Box<dyn LoxType>, line: usize) -> Self {
        Token { ttype, lexeme, literal, line }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.ttype, self.lexeme, self.literal.lox_print())
    }
}
