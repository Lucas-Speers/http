mod thread;
mod tcp;
mod http;

use std::{fs, net::TcpListener, path::Path, sync::mpsc};

use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Settings {
    ip: String,
    port: u16,
    threads: usize,
    path: Box<Path>,
}

fn main() {
    let config_path = Path::new("settings.toml");
    assert!(Path::new("settings.toml").exists(), "settings.toml does not exist");

    let file = fs::read(config_path)
        .expect("could not read settings.toml")
        .iter()
        .map(|c| *c as char)
        .collect::<String>();

    let settings: Settings = toml::from_str(&file).expect("settings.toml is not in propper format");

    assert!(settings.path.exists(), "path: {:?} does not exist", &settings.path);

    println!("{:?}", settings);

    let (tx, rx) = mpsc::channel();

    let thread_handler = thread::ThreadHandler::new(rx, settings.threads);
    std::thread::spawn(move || { thread_handler.run();});

    let listener = TcpListener::bind(settings.ip + ":" + &settings.port.to_string()).unwrap();
    let tcp_handler = tcp::TcpHandler::new(tx, listener);

    tcp_handler.run();
}
