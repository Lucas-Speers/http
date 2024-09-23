use std::{ffi::OsStr, io::{BufRead, BufReader, Read, Write}, net::TcpStream, path::{Component, Path, PathBuf}};

use mime_guess::MimeGuess;

use crate::SETTINGS;

#[derive(Debug)]
pub enum HttpError {
    E400,
    E403,
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
        if final_path.with_added_extension("html").is_file() {
            final_path = final_path.with_added_extension("html");
        } else if final_path.join("index.html").is_file() {
            final_path = final_path.join("index.html");
        }
    }

    Ok(final_path)
}

/// Takes a `TcpStream` and handles the http protocol
pub fn handle_connection(mut stream: TcpStream) -> Result<(), HttpError> {
    
    let mut error: HttpError = HttpError::E404;

    'okay: {
        let mut reader = BufReader::new(&stream);
    
        let mut request = String::new();
        match reader.read_line(&mut request) {
            Ok(_) => (),
            Err(_) => {
                error = HttpError::E500;
                break 'okay;
            }
        }
    
        let mut path;
        match request.split(" ").nth(1) {
            Some(x) =>path = PathBuf::from(x),
            _ => {
                error = HttpError::E400;
                break 'okay;
            }
        }
    
        match get_true_path(&path) {
            Ok(x) => path = x,
            Err(_) => {
                error = HttpError::E403;
                break 'okay;
            },
        }
    
        let mut content = Vec::new();
        let mut file = match std::fs::File::open(&path) {
            Ok(x) => x,
            Err(_) => {
                error = HttpError::E404;
                break 'okay;
            },
        };
        match file.read_to_end(&mut content) {
            Ok(_) => (),
            Err(_) => {
                error = HttpError::E500;
                break 'okay;
            },
        }
    
        let guess = MimeGuess::from_path(path).first().unwrap().to_string();
    
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
    };
    
    let mut responce: Vec<u8> = Vec::new();
    
    match error {
        HttpError::E400 => {
            responce.append(&mut "HTTP/1.1 400 Bad Request\r\n\r\n\r\n".as_bytes().to_vec());
        },
        HttpError::E403 => {
            responce.append(&mut "HTTP/1.1 403 Forbidden\r\n\r\n\r\n".as_bytes().to_vec());
        },
        HttpError::E404 => {
            match SETTINGS.teapot {
                true => responce.append(&mut "HTTP/1.1 418 I'm a teapot\r\n\r\n\r\n".as_bytes().to_vec()),
                false => responce.append(&mut "HTTP/1.1 404 Not Found\r\n\r\n\r\n".as_bytes().to_vec()),
            }
        },
        HttpError::E500 => {
            responce.append(&mut "HTTP/1.1 500 Internal Server Error\r\n\r\n\r\n".as_bytes().to_vec());
        },
        HttpError::CannotReturn => return Err(error),
    }
    
    match stream.write_all(&responce) {
        Ok(_) => (),
        Err(_) => return Err(HttpError::CannotReturn),
    }

    Ok(())
}