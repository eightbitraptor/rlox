use std::fmt;

pub type LoxResult = std::result::Result<(), LoxError>;

// TODO: use &str here to remove allocations. Relies on understanding
// lifetimes, which I currently don't
#[derive(Debug)]
pub struct LoxError {
    pub line: i32,
    pub place: String,
    pub message: String,
}

impl fmt::Display for LoxError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}] Error {}: {}", self.line, self.place, self.message)
    }
}
