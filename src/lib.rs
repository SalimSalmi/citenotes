use std::fmt;
use std::error::Error;

#[derive(Debug)]
pub struct CiteNoteError {
    message: String,
}

pub type Result<T> = std::result::Result<T, CiteNoteError>;

impl Error for CiteNoteError {}

impl fmt::Display for CiteNoteError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let message = format!("Error: {}", self.message);
        write!(f, "{}", message)
    }
}
impl CiteNoteError{
    pub fn new(message: &str) -> CiteNoteError {
        CiteNoteError { message: String::from(message) }
    }
}
