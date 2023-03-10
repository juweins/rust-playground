// expose submodules directly
pub use method::Method;
pub use request::Request;
pub use response::Response;
pub use request::ParseError;
pub use query_string::{QueryString, Value as QueryStringValue};
pub use status_code::StatusCode;

// maintain long invocation for compatibility 
pub mod method;
pub mod query_string;
pub mod request;
pub mod response;
pub mod status_code;