extern crate simple_http_server;

use std::net::{TcpListener, TcpStream};
use std::io::prelude::*;
use std::io::{BufReader, BufWriter};
use std::fs::File;
use std::path::PathBuf;
use std::env;
use simple_http_server::{dir_html, AppResult, Request, Response, Status};
use std::error::Error;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").expect("Couldn't bind to address");
    println!("Server started on {}", "127.0.0.1:8080");

    let cwd = env::current_dir().expect("Couldn't read current directory");
    for stream in listener.incoming() {
        let mut stream = stream.expect("Couldn't handle TCP stream");
        handle_request(&mut stream, &cwd)
            .or_else(|err| -> AppResult<()> {
                let mut res = Response::new(Status::ServerErr);
                let mes = format!("Internal Server Error: {}", err.description());
                res.set_body_string(mes);
                BufWriter::new(&mut stream).write(&res.into_string()?.into_bytes())?;
                Ok(())
            })
            .expect("Couldn't respond server error...");
    }
}

fn handle_request(stream: &mut TcpStream, cwd: &PathBuf) -> AppResult<()> {
    // BufReader は Read 型の値を受け取る。
    // TcpStream と &TcpStream の両方とも Read を実装しているので、
    // BufReader には所有権を移す事も、貸すだけにする事もできる。
    // stream は write 処理でも使うので、今回は貸すだけにする。
    let req = Request::from_stream(stream)?;
    let path = cwd.join(req.path().trim_matches('/'));

    let mut res = match path.canonicalize() {
        Err(_) => Response::new(Status::NotFound),
        Ok(path) => {
            if !is_allowed_path(&path, &cwd) {
                Response::new(Status::NotFound)
            } else if path.is_dir() {
                let html = dir_html::generate(&path, &cwd)?;
                let mut res = Response::new(Status::Ok);
                res.set_body_string(html);
                res.add_header("Content-Type", "text/html");
                res
            } else {
                let file = File::open(&path)?;
                let mut res = Response::new(Status::Ok);
                res.set_body(Box::new(BufReader::new(file)));
                res
            }
        }
    };

    let res = res.into_string()?.into_bytes();
    BufWriter::new(stream).write(&res)?;

    Ok(())
}

fn is_allowed_path(path: &PathBuf, cwd: &PathBuf) -> bool {
    path.starts_with(cwd)
}
