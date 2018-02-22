mod status;
mod request;
mod response;
mod errors;
pub mod headers;
pub mod dir_html;

pub use self::status::Status;
pub use self::request::Request;
pub use self::response::Response;
pub use self::errors::AppError;

pub type AppResult<T> = std::result::Result<T, errors::AppError>;
