use std::io::prelude::*;
use std::collections::HashMap;
use super::{Status, AppResult};

type Headers = HashMap<String, String>;

pub struct Response<'a> {
    version: &'a str,
    status: Status,
    body: Option<Box<Read>>,
    headers: Headers,
}

impl<'a> Response<'a> {
    pub fn new(status: Status) -> Response<'a> {
        let mut res = Response {
            version: "HTTP/1.1",
            status,
            body: None,
            headers: HashMap::new(),
        };
        Response::add_default_headers(&mut res);
        res
    }

    fn add_default_headers(res: &mut Response) {
        res.add_header("Server", "rust-simple-http-server");
        res.add_header("Connection", "Close");
        res.add_header("Content-Type", "text/plain");
    }

    pub fn set_body(&mut self, body: Box<Read>) {
        self.body = Some(body);
    }

    pub fn set_body_string(&mut self, body: String) {
        use std::io::Cursor;
        let body = Cursor::new(body.into_bytes());
        self.body = Some(Box::new(body));
    }

    pub fn add_header<K: Into<String>, V: Into<String>>(&mut self, key: K, value: V) {
        self.headers.insert(key.into(), value.into());
    }

    pub fn into_string(&mut self) -> AppResult<String> {
        let body = match self.body.take() {
            None => "".to_string(),
            Some(mut body) => {
                let mut buf = String::new();
                let size = body.read_to_string(&mut buf)?;
                self.add_header("Content-Length", size.to_string());
                format!("\n{}", buf)
            }
        };
        let status = self.make_statusline();
        let headers = self.headers_to_string();
        Ok(format!("{}\n{}\n{}", status, headers, body))
    }

    fn make_statusline(&self) -> String {
        format!("{} {} {}",
                self.version,
                self.status.code(),
                self.status.name())
    }

    fn headers_to_string(&self) -> String {
        self.headers.iter()
            .map(|(k, v)| make_header(&k, &v))
            .collect::<Vec<_>>()
            .join("\n")
    }
}

fn make_header(key: &str, value: &str) -> String {
    format!("{}: {}", key, value)
}
