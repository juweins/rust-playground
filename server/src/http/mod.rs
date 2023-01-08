// expose submodules directly
pub use method::Method;
pub use request::Request;
pub use request::ParseError;

// maintain long invocation for compatibility 
pub mod method;
pub mod request;