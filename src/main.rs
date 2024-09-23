#![feature(path_add_extension)]

mod thread;
mod tcp;
mod http;

use std::{fs, net::TcpListener, path::Path, sync::{mpsc, LazyLock}};

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Settings {
    ip: String,
    port: u16,
    threads: usize,
    path: Box<Path>,
    #[serde(default)]
    teapot: bool,
}

pub static SETTINGS: LazyLock<Settings> = LazyLock::new(|| {
    let config_path = Path::new("settings.toml");
    assert!(Path::new("settings.toml").exists(), "settings.toml does not exist");
    
    let file = fs::read(config_path)
        .expect("could not read settings.toml")
        .iter()
        .map(|c| *c as char)
        .collect::<String>();
    
    let s: Settings = toml::from_str(&file).expect("settings.toml is not in proper format");
    
    assert!(s.path.exists(), "path: {:?} does not exist", &s.path);

    s
});

fn main() {

    let (tx, rx) = mpsc::channel();

    let thread_handler = thread::ThreadHandler::new(rx, SETTINGS.threads);

    std::thread::spawn(move || { thread_handler.run();});

    let listener = TcpListener::bind(SETTINGS.ip.clone() + ":" + &SETTINGS.port.to_string()).unwrap();
    let tcp_handler = tcp::TcpHandler::new(tx, listener);

    tcp_handler.run();
}
