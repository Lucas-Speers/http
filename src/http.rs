use std::{io::Write, net::TcpStream};



pub fn handle_connection(mut stream: TcpStream) -> String {
    let mut responce = String::from("HTTP/1.1 200 OK\r\n\r\n");

    stream.write_all(responce.as_bytes()).unwrap();

    responce
}