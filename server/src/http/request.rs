use super::method::Method;
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::Display;
use std::fmt::Debug;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;

pub struct Request {
    path: String,
    method: Method,
    query_string: Option<String>,
}

impl TryFrom<&[u8]> for Request {
    type Error = String;

    // example incoming Request: GET /search?name=sometext&sort=1 HTTP/1.1
    fn try_from(buffer: &[u8]) -> Result<Self, Self::Error> {
        unimplemented!()
    }
}

pub enum ParseError {
     InvalidRequest,
     InvalidEncoding,
     InvalidProtocol,
     InvalidMethod,
}
impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }

}

impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }

}

impl ParseError {
    fn message(&self) -> &str {
        match self {
            Self::InvalidRequest => "Invalid Request",
            Self::InvalidEncoding => "Invalid Encoding",
            Self::InvalidProtocol => "Invalid Protocol",
            Self::InvalidMethod => "Invalid Method",
        }

    }
}

impl Error for ParseError {}
