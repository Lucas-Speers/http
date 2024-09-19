use std::{ffi::OsStr, io::{BufRead, BufReader, Read, Write}, net::TcpStream, path::{Component, Path, PathBuf}};

use mime_guess::MimeGuess;

pub enum HttpError {
    E404,
    E500,
    CannotReturn,
}

/// Checks that the path only consists of 'normal' components.
/// 
/// Reterns false if path has `..`.
fn check_file_safety(path: &Path) -> bool {
    // TODO: make this better
    path.components()
        .all(|x| {
            if let Component::Normal(y) = x {return y != OsStr::new("~");}
            if let Component::RootDir = x {return true;}
            if let Component::CurDir = x {return true;}
            false
        })
}

fn get_true_path(path: &Path) -> Result<PathBuf, ()> {

    let mut final_path = super::SETTINGS.path.join(path.strip_prefix("/").unwrap());

    if check_file_safety(&final_path) == false {
        return Err(());
    }

    if !final_path.is_file() {
        println!("not file");
        println!("{:?}", final_path.with_added_extension("html"));
        if final_path.with_added_extension("html").is_file() {
            println!("is file");
            final_path = final_path.with_added_extension("html");
        } else if final_path.join("index.html").is_file() {
            final_path = final_path.join("index.html");
        }
    }

    Ok(final_path)
}

pub fn handle_connection_with_error(mut stream: TcpStream) {
    match handle_connection(&mut stream) {
        Ok(_) => (),
        Err(e) => {
            match e {
                HttpError::E404 => {
                    stream.write_all(String::new().as_bytes());
                },
                HttpError::E500 => todo!(),
                HttpError::CannotReturn => (),
            }
        },
    }
}

/// Takes a `TcpStream` and handles the http protocol
pub fn handle_connection(stream: &mut TcpStream) -> Result<(), HttpError> {

    let mut reader = BufReader::new(&stream);

    let mut request = String::new();
    match reader.read_line(&mut request) {
        Ok(_) => (),
        Err(_) => return Err(HttpError::E500),
    }

    println!("aaaa '{:?}'", request.split("\r").next());

    let mut path;
    match request.split(" ").nth(1) {
        Some(x) =>path = PathBuf::from(x),
        _ => {return Ok(());}
    }

    match get_true_path(&path) {
        Ok(x) => path = x,
        Err(_) => return Ok(()),
    }

    let mut content = Vec::new();
    println!("{:?}", path);
    let mut file = match std::fs::File::open(&path) {
        Ok(x) => x,
        Err(_) => return Err(HttpError::E404),
    };
    match file.read_to_end(&mut content) {
        Ok(_) => (),
        Err(_) => return Err(HttpError::E500),
    }

    let guess = MimeGuess::from_path(path).first().unwrap().to_string();
    println!("{guess}");

    let mut responce: Vec<u8> = Vec::new();

    responce.append(&mut "HTTP/1.1 200 OK\r\nContent-Type: ".as_bytes().to_vec());
    responce.append(&mut guess.as_bytes().to_vec());
    responce.append(&mut "\r\nContent-Length: ".as_bytes().to_vec());
    responce.append(&mut content.len().to_string().as_bytes().to_vec());
    responce.append(&mut "\r\n\r\n".as_bytes().to_vec());
    responce.append(&mut content);

    match stream.write_all(&responce) {
        Ok(_) => (),
        Err(_) => return Err(HttpError::CannotReturn),
    }

    Ok(())
}