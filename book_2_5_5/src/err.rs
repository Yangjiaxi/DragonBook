#[derive(Debug)]
pub enum ParserError {
    SyntaxError(String),
}

impl From<&'static str> for ParserError {
    fn from(e: &'static str) -> ParserError {
        ParserError::SyntaxError(e.to_string())
    }
}

impl From<String> for ParserError {
    fn from(e: String) -> ParserError {
        ParserError::SyntaxError(e)
    }
}
