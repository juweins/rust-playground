/* This file contains the incoming HTTP Request handling logic.
//
// The request is parsed by taking advantage of the iterator from split_whitespace()
// By defining custom Error types, basic error handling is achieved.
// 
*/

use super::method::{Method, MethodError};
use super::QueryString;
use std::str;
use std::str::Utf8Error;
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Display, Debug, Formatter};
use std::fmt::Result as FmtResult;

#[derive(Debug)]
pub struct Request<'buffer> {
    path: &'buffer str,
    method: Method,
    query: Option<QueryString<'buffer>>,
}

impl<'buffer> TryFrom<&'buffer[u8]> for Request<'buffer> {
    type Error = ParseError;

    // example incoming Request: GET /search?name=sometext&sort=1 HTTP/1.1
    fn try_from(buffer: &'buffer [u8]) -> Result<Self, Self::Error> {
        let request = str::from_utf8(buffer)?; 
                match parse_request_header(request) {
                    // 0: Method, 1: Path 2: Protocol
                    Some(request) =>{ 
                        let method = request.0;
                        // check if protocol can be handled by server
                        if request.2 != "HTTP/1.1" {
                            return Err(ParseError::InvalidProtocol);
                        }            
                        // parse method into enum method type
                        let method: Method = request.0.parse()?;
                        
                        // explode query string
                        let mut path = request.1;
                        let mut query = None;

                        if let Some(index) = path.find("?") {
                            // since the "?" is utf8 encoded, adding one byte is considered safe
                            query = Some(QueryString::from(&path[index + 1..]));
                            path = &path[..index];

                        }
                        Ok(Self {
                            path,
                            query,
                            method,
                        })
                    }

                    None => { return Err(ParseError::InvalidRequest);
                    }
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

    return Some((method, path, protocol))
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

// custom error types implementation
impl From<MethodError> for ParseError {
    fn from(_: MethodError) -> Self {
        Self::InvalidMethod
    }
}

impl From<Utf8Error> for ParseError {
    fn from(_: Utf8Error) -> Self {
        Self::InvalidEncoding
    }
}

impl Error for ParseError {}