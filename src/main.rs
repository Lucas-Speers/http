use std::fs;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Settings {
    port: u16,
    ip: String
}

fn main() {
    let file = fs::read("settings.toml")
        .unwrap()
        .iter()
        .map(|c| *c as char)
        .collect::<String>();

    let settings: Settings = toml::from_str(&file).unwrap();
    
    println!("{:?}", settings);
}
