// expose submodules directly
pub use method::Method;
pub use request::Request;
pub use request::ParseError;
pub use query_string::{QueryString, Value as QueryStringValue};

// maintain long invocation for compatibility 
pub mod method;
pub mod query_string;
pub mod request;