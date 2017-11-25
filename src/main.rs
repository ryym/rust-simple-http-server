use std::net::{TcpListener, TcpStream};
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080")
        .expect("Couldn't bind to address");
    println!("Server started on {}", "127.0.0.1:8080");

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        loop {
            let head = read_first_line(&stream);
            stream.write(&head.into_bytes()).unwrap();
            stream.flush().unwrap();
        }
    }
}

fn read_first_line(stream: &TcpStream) -> String {
    // BufReader は Read 型の値を受け取る。
    // TcpStream と &TcpStream の両方とも Read を実装しているので、
    // BufReader には所有権を移す事も、貸すだけにする事もできる。
    // stream は write 処理でも使うので、今回は貸すだけにする。
    let mut reader = BufReader::new(stream);
    let mut buf = String::new();
    reader.read_line(&mut buf).expect("Couldn't read_line");
    buf
}
