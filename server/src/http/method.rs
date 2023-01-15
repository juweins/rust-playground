use std::str::FromStr;

#[derive(Debug)]
pub enum Method {
    GET,
    POST,
    DELETE,
    CONNECT,
    HEAD,
    OPTIONS,
    PATCH,
    PUT,
    TRACE,
}

impl FromStr for Method {

    type Err = MethodError;

    fn from_str(s: &str) -> Result<Self, Self::Err>{
        match s {
            "GET" => Ok(Self::GET),
            "POST" => Ok(Self::POST),
            "DELETE" => Ok(Self::DELETE),
            "CONNECT" => Ok(Self::CONNECT),
            "HEAD" => Ok(Self::HEAD),
            "OPTIONS" => Ok(Self::OPTIONS),
            "PATCH" => Ok(Self::PATCH),
            "PUT" => Ok(Self::PUT),
            "TRACE" => Ok(Self::TRACE),
            _ => Err(MethodError)
        }
    }

}

pub struct MethodError;

