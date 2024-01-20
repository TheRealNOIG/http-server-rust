use std::{
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
};

use http_server_starter_rust::{HttpError, HttpRequestCode, StartLine};

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
            HttpRequestCode::BadRequest,
        ));
    }
    println!("request: {:#?}", http_request);

    let start_line = StartLine::new(&http_request[0].clone())?;

    let response: String = match start_line.path.as_str() {
        "/" => status_code(&start_line, HttpRequestCode::Ok) + "\r\n",
        path if path.starts_with("/echo/") => handle_echo(path, &start_line),
        _ => status_code(&start_line, HttpRequestCode::NotFound) + "\r\n",
    };
    println!("response: {:#?}", response);

    stream
        .write_all(&response.as_bytes())
        .map_err(|e| HttpError::HttpParseError(e.to_string(), HttpRequestCode::BadRequest))
}

fn status_code(start_line: &StartLine, code: HttpRequestCode) -> String {
    let (number, phrase) = code.to_tuple();
    format!("{} {} {}\r\n", start_line.version, number, phrase)
}
//TODO: make header struct and have it generate the header string
fn handle_echo(path: &str, start_line: &StartLine) -> String {
    let reply = path.splitn(3, '/').skip(2).collect::<Vec<&str>>().join("/");
    let status_line = status_code(start_line, HttpRequestCode::Ok);
    let formatted_response = format!(
        "{}{}Content-Length: {}\r\n\r\n{}\r\n",
        status_line,
        "Content-Type: text/plain\r\n",
        reply.len(),
        reply,
    );
    formatted_response
}

