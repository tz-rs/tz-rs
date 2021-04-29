pub use parse_error::ParseError;
pub use rpc_error::RpcError;

mod rpc_error {
    use std::{error::Error, fmt};
    #[derive(Debug)]
    pub enum RpcError {
        RequestError(reqwest::Error),
        ParsingError(super::ParseError),
        Other(String),
    }

    impl From<super::ParseError> for RpcError {
        fn from(parse_error: super::ParseError) -> Self {
            Self::ParsingError(parse_error)
        }
    }

    impl From<reqwest::Error> for RpcError {
        fn from(error: reqwest::Error) -> Self {
            Self::RequestError(error)
        }
    }

    impl From<url::ParseError> for RpcError {
        fn from(url_parse_error: url::ParseError) -> Self {
            let parse_error = super::ParseError::from(url_parse_error);
            Self::ParsingError(parse_error)
        }
    }

    impl Error for RpcError {}
    impl fmt::Display for RpcError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            let error_string = match &self {
                Self::RequestError(request_error) => request_error.to_string(),
                Self::ParsingError(parse_error) => parse_error.to_string(),
                Self::Other(error_string) => error_string.to_owned(),
            };
            write!(f, "failed in rpc call. detail: {}", &error_string)
        }
    }
}

mod parse_error {
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

    impl Error for ParseError {}
    impl fmt::Display for ParseError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Error parsing or flattening bulk array JSON text")
        }
    }
}
