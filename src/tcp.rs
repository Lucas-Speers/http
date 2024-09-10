use std::{net::{TcpListener, TcpStream}, sync::mpsc};


pub struct TcpHandler {
    tx: mpsc::Sender<TcpStream>,
    listener: TcpListener,
}

impl TcpHandler {
    pub fn new(tx: mpsc::Sender<TcpStream>, listener: TcpListener) -> TcpHandler {
        TcpHandler {tx, listener}
    }
    pub fn run(&self) {
        for stream in self.listener.incoming() {
            match stream {
                Ok(x) => self.tx.send(x).unwrap(),
                Err(x) => {
                    println!("{x}");
                    panic!();
                },
            }
        }
    }
}