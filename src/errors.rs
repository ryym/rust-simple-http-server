use std::io;
use std::fmt;
use std::error::Error;

#[derive(Debug)]
pub struct AppError {
    message: String,
    cause: Option<Box<Error>>,
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut ret = write!(f, "{}", &self.message);
        if let Some(ref err) = self.cause {
            ret = err.fmt(f);
        }
        ret
    }
}

impl Error for AppError {
    fn description(&self) -> &str {
        self.cause
            .as_ref()
            .map_or_else(|| self.message.as_str(), |e| e.description())
    }

    fn cause(&self) -> Option<&Error> {
        self.cause.as_ref().map(|e| &**e)
    }
}

impl From<io::Error> for AppError {
    fn from(err: io::Error) -> AppError {
        AppError::by(Box::new(err))
    }
}

impl AppError {
    pub fn new<S: Into<String>>(message: S) -> AppError {
        AppError {
            message: message.into(),
            cause: None,
        }
    }

    pub fn by(cause: Box<Error>) -> AppError {
        AppError {
            message: String::new(),
            cause: Some(cause),
        }
    }
}
