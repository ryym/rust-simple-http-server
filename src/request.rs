use std::io::prelude::*;
use std::io::BufReader;
use super::AppError;
use super::AppResult;

#[derive(Debug)]
pub struct Request {
    method: String, // enum?
    path: String,
    version: String,
}

impl Request {
    pub fn from_stream(stream: &mut Read) -> AppResult<Request> {
        let mut reader = BufReader::new(stream);
        let mut buf = String::new();
        reader.read_line(&mut buf)?;

        let parts: Vec<&str> = buf.split(" ").collect();
        if parts.len() != 3 {
            return Err(AppError::new(format!("Invalid request header: {}", buf)));
        }

        Ok(Request {
            method: parts[0].to_string(),
            path: parts[1].to_string(),
            version: parts[2].to_string(),
        })
    }

    pub fn path(&self) -> String {
        self.path.clone()
    }
}
