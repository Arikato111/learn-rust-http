pub mod error;
pub mod server;
pub use error::Error;
pub use server::Server;

pub type Result<T> = std::result::Result<T, Error>;
