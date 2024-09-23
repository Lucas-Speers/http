use std::{net::TcpStream, sync::mpsc, thread::JoinHandle};

use crate::http;


pub struct ThreadHandler {
    threads: Vec<(
        JoinHandle<()>,
        mpsc::Sender<TcpStream>
    )>,
    rx: mpsc::Receiver<TcpStream>,
}

impl ThreadHandler {
    pub fn new(rx: mpsc::Receiver<TcpStream>, num_threads: usize) -> ThreadHandler {
        let mut threads = Vec::new();
        for _ in 0..num_threads {
            let (tx, rx) = mpsc::channel::<TcpStream>();
            let thread = std::thread::spawn(move || {job_handler(rx)});
            threads.push((thread, tx))
        }
        ThreadHandler {threads, rx}
    }
    pub fn run(&self) {
        // simple round robin
        let mut index = 0;
        loop {
            let x = self.rx.recv();
            if x.is_err() {
                println!("{}", x.unwrap_err());
                panic!();
            }
            let x = x.unwrap();
            self.threads[index].1.send(x).unwrap();
            index = (index + 1) % self.threads.len();
        }
    }
}

fn job_handler(rx: mpsc::Receiver<TcpStream>) {
    loop {
        let x = rx.recv();
        if x.is_err() {
            println!("{}", x.unwrap_err());
            panic!();
        }
        let x = x.unwrap();

        match http::handle_connection(x) {
            Ok(_) => (),
            Err(x) => println!("{:?}", x),
        }
    }
}