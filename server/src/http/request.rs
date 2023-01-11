use super::method::Method;
use std::str;
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
    type Error = ParseError;

    // example incoming Request: GET /search?name=sometext&sort=1 HTTP/1.1
    fn try_from(buffer: &[u8]) -> Result<Self, Self::Error> {
        match str::from_utf8(buffer) {
            Ok(request) => {

                match parse_request_header(request) {
                    Some(request) =>{ 
                        unimplemented!();

                    }
                    None => { return Err(ParseError::InvalidRequest)
                    }
                }
            },
            Err(_) => {return Err(ParseError::InvalidEncoding)},
        }


    }
}

// explode the received request string into its three components
// example: GET src/some/index.html HTTP/1.1 -> ("GET", "src/some/index.html", "HTTP/1.1")
fn parse_request_header(request: &str) -> Option<(&str, &str, &str)>{

    // split returns an iterator object
    let mut iterator = request.split_whitespace();

    // unwrapping the request fragments
    let method: &str = iterator.next().unwrap();
    let path: &str = iterator.next().unwrap();
    let protocol: &str = iterator.next().unwrap();

    return Some((method, path, protocol));
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
