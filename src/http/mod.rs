pub mod error;
pub mod server;
pub use error::Error;
pub use server::Server;
pub mod method;

pub mod query_string;
pub use query_string::QueryString;

pub mod request;
pub use method::Method;
pub use request::Request;
pub type Result<T> = std::result::Result<T, Error>;
