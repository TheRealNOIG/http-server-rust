use std::{
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
};

use http_server_starter_rust::{HttpError, HttpErrorCode, StartLine};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(_stream) => {
                println!("accepted new connection");
                handle_stream(_stream);
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

// The Book pg 463
fn handle_stream(mut stream: TcpStream) -> Result<(), HttpError> {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();
    if http_request.is_empty() {
        return Err(HttpError::HttpParseError(
            "Empty request".to_string(),
            HttpErrorCode::BadRequest,
        ));
    }
    println!("request: {:#?}", http_request);

    let start_line = StartLine::new(&http_request[0].clone())?;

    let response: &[u8] = if start_line.path == "/" {
        b"HTTP/1.1 200 OK\r\n\r\n"
    } else {
        b"HTTP/1.1 404 Not Found\r\n\r\n"
    };

    stream
        .write_all(response)
        .map_err(|e| HttpError::HttpParseError(e.to_string(), HttpErrorCode::BadRequest))
}

