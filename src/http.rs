use std::{ffi::OsStr, io::{BufRead, BufReader, Read, Write}, net::TcpStream, path::{Component, Path}};

/// Checks that the path only consists of 'normal' components.
/// 
/// Reterns false if path has `..`.
fn check_file_safety(path: &Path) -> bool {
    path.components()
        .all(|x| {
            if let Component::Normal(y) = x {return y != OsStr::new("~");}
            if let Component::RootDir = x {return true;}
            false
        })
}

/// Takes a `TcpStream` and handles the http protocol
pub fn handle_connection(mut stream: TcpStream) -> String {

    let mut reader = BufReader::new(&stream);

    let mut request = String::new();
    reader.read_line(&mut request).unwrap();

    let path = Path::new(request.split(" ").nth(1).unwrap());

    let mut content = String::new();
    if check_file_safety(path) {
        if path.is_file() {
            let mut file = std::fs::File::create(path).unwrap();
            std::fs::File::read_to_string(&mut file, &mut content).unwrap();
        } else if Path::new(&(path.to_str().unwrap().to_owned() + ".html")).is_file() {
            let mut file = std::fs::File::create(Path::new(&(path.to_str().unwrap().to_owned() + ".html"))).unwrap();
            std::fs::File::read_to_string(&mut file, &mut content).unwrap();
        } else {
            content = String::from("dir");
        }
    } else {
        content = String::from("did you really think that would work");
    }

    let responce = format!("HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}", content.len(), &content);
    stream.write_all(responce.as_bytes()).unwrap();

    responce
}