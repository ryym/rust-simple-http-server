use std::io::prelude::*;
use std::io::BufReader;

#[derive(Debug)]
pub struct Request {
    method: String, // enum?
    path: String,
    version: String,
}

impl Request {
    pub fn from_stream(stream: &mut Read) -> Request {
        let mut reader = BufReader::new(stream);
        let mut buf = String::new();
        reader.read_line(&mut buf).expect("Couldn't read_line");

        let parts: Vec<&str> = buf.split(" ").collect();
        if parts.len() != 3 {
            panic!(format!("Invalid request header: {}", buf));
        }

        Request {
            method: String::from(parts[0]),
            path: String::from(parts[1]),
            version: String::from(parts[2]),
        }
    }

    pub fn path(&self) -> String {
        self.path.clone()
    }
}
