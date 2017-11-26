extern crate simple_http_server;

use std::net::{TcpListener, TcpStream};
use std::io::prelude::*;
use std::io::{BufReader, BufWriter};
use std::fs::File;
use std::path::PathBuf;
use std::env;
use simple_http_server::{Status, Request, Response};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080")
        .expect("Couldn't bind to address");
    println!("Server started on {}", "127.0.0.1:8080");

    let cwd = env::current_dir().unwrap();
    for stream in listener.incoming() {
        handle_request(&mut stream.unwrap(), &cwd);
    }
}

fn handle_request(stream: &mut TcpStream, cwd: &PathBuf) {
    // BufReader は Read 型の値を受け取る。
    // TcpStream と &TcpStream の両方とも Read を実装しているので、
    // BufReader には所有権を移す事も、貸すだけにする事もできる。
    // stream は write 処理でも使うので、今回は貸すだけにする。
    let req = Request::from_stream(stream);
    let path = cwd.join(req.path().trim_matches('/'));

    let mut res = if path.exists() && path.is_file() {
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
