use std::{error::Error, fmt};

#[derive(Debug)]
pub enum ParseError {
    ResponseParsingError(String),
    RequestParsingError(String),
}

impl From<std::num::ParseIntError> for ParseError {
    fn from(parse_error: std::num::ParseIntError) -> Self {
        Self::ResponseParsingError(parse_error.to_string())
    }
}

impl From<url::ParseError> for ParseError {
    fn from(url_parse_error: url::ParseError) -> Self {
        Self::RequestParsingError(url_parse_error.to_string())
    }
}

impl From<serde_json::Error> for ParseError {
    fn from(serde_error: serde_json::Error) -> Self {
        Self::ResponseParsingError(serde_error.to_string())
    }
}

impl Error for ParseError {}
impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error parsing or flattening JSON array text")
    }
}
