use std::fmt;

pub type LoxResult<T> = std::result::Result<T, LoxError>;

// TODO: use &str here to remove allocations. Relies on understanding
// lifetimes, which I currently don't
#[derive(Debug)]
pub struct LoxError {
    pub line: i32,
    pub place: Option<String>,
    pub message: Option<String>,
}

impl LoxError {
    pub fn new(l: i32, message: &str) -> Self {
        LoxError {
            line: l,
            place: None,
            message: Some(String::from(message)),
        }
    }
}

impl fmt::Display for LoxError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}] Error {}: {}",
               self.line,
               self.place.as_ref().unwrap_or(&String::from("?")),
               self.message.as_ref().unwrap_or(&String::from("No message")),
        )
    }
}
