use std::io::prelude::*;
use super::{Status, AppResult};

pub struct Response<'a> {
    version: &'a str,
    status: Status,
    body: Option<Box<Read>>,
}

impl<'a> Response<'a> {
    pub fn new(status: Status) -> Response<'a> {
        Response {
            version: "HTTP/1.1",
            status,
            body: None,
        }
    }

    pub fn set_body(&mut self, body: Box<Read>) {
        self.body = Some(body);
    }

    // Maybe Response implements Read trait?
    pub fn into_string(&mut self) -> AppResult<String> {
        let sline = self.make_statusline();
        let res = match self.body.take() {
            None => sline,
            Some(mut body) => {
                let mut buf = String::new();
                body.read_to_string(&mut buf)?;
                sline + "\n" + &buf
            }
        };
        Ok(res)
    }

    fn make_statusline(&self) -> String {
        format!("{} {} {}\n",
                self.version,
                self.status.code(),
                self.status.name())
    }
}