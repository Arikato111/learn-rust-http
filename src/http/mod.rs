pub mod error;
pub use error::Error;

pub mod api;

pub mod server;
pub use server::Server;

pub mod method;
pub use method::Method;

pub mod query_string;
pub use query_string::QueryString;

pub mod response;
pub use response::Response;

pub mod status;
pub use status::HttpStatus;

pub mod request;
pub use request::Request;

pub type Result<T> = std::result::Result<T, Error>;
