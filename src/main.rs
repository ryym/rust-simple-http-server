use std::net::{TcpListener, TcpStream};
use std::io::prelude::*;
use std::io::{BufReader, BufWriter};
use std::fs::File;
use std::env;

#[derive(Debug)]
struct Request {
    method: String, // enum?
    path: String,
    version: String,
}

enum Status {
    Ok,
    NotFound,
}

impl Status {
    fn code(&self) -> u16 {
        match *self {
            Status::Ok => 200,
            Status::NotFound => 404,
        }
    }

    fn name(&self) -> &'static str {
        match *self {
            Status::Ok => "OK",
            Status::NotFound => "NotFound",
        }
    }
}

struct Response<'a> {
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
    pub fn into_string(&mut self) -> String {
        let sline = self.make_statusline();
        match self.body.take() {
            None => sline,
            Some(mut body) => {
                let mut buf = String::new();
                body.read_to_string(&mut buf).unwrap();
                sline + "\n" + &buf
            }
        }
    }

    fn make_statusline(&self) -> String {
        format!("{} {} {}\n",
                self.version,
                self.status.code(),
                self.status.name())
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080")
        .expect("Couldn't bind to address");
    println!("Server started on {}", "127.0.0.1:8080");

    let cwd = env::current_dir().unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        let req = parse_request(&stream);
        let path = cwd.join(req.path.trim_matches('/'));

        let mut res = if path.exists() {
            let file = File::open(&path).unwrap();
            let mut res = Response::new(Status::Ok);
            res.set_body(Box::new(BufReader::new(file)));
            res
        } else {
            Response::new(Status::NotFound)
        };

        let res = res.into_string().into_bytes();
        BufWriter::new(stream).write(&res).unwrap();
    }
}

fn parse_request(stream: &TcpStream) -> Request {
    // BufReader は Read 型の値を受け取る。
    // TcpStream と &TcpStream の両方とも Read を実装しているので、
    // BufReader には所有権を移す事も、貸すだけにする事もできる。
    // stream は write 処理でも使うので、今回は貸すだけにする。
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
