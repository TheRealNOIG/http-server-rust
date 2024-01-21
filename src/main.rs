use std::{
    io::{BufRead, BufReader, Read, Write},
    net::{TcpListener, TcpStream},
    thread,
};

use http_server_starter_rust::{
    HttpError, HttpRequestCode, RepresentationHeader, RequestHeader, StartLine,
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        thread::spawn(|| match stream {
            Ok(_stream) => {
                println!("accepted new connection");
                if let Err(e) = handle_stream(_stream) {
                    println!("Error handling stream: {}", e);
                }
            }
            Err(e) => {
                println!("error: {}", e);
            }
        });
    }
}

// The Book pg 463
// TODO: Learn Tokio (easy right?)
fn handle_stream(mut stream: TcpStream) -> Result<(), HttpError> {
    let (start_line, request_header, _http_header, _body) = match parse_http_request(&stream) {
        Ok((start_line, request_header, headers, body)) => {
            (start_line, request_header, headers, body)
        }
        Err(e) => return Err(e),
    };

    let response: String = match start_line.path.as_str() {
        "/" => status_code(&start_line, HttpRequestCode::Ok) + "\r\n",
        path if path.starts_with("/echo/") => handle_echo(path, &start_line),
        path if path.starts_with("/user-agent") => user_agent(&request_header, &start_line),
        path if path.starts_with("/files/") => get_file(path, &start_line),
        _ => status_code(&start_line, HttpRequestCode::NotFound) + "\r\n",
    };
    println!("response: {:#?}", response);

    stream
        .write_all(response.as_bytes())
        .map_err(|e| HttpError::HttpParseError(e.to_string(), HttpRequestCode::InternalServerError))
}

fn status_code(start_line: &StartLine, code: HttpRequestCode) -> String {
    let (number, phrase) = code.to_tuple();
    format!("{} {} {}\r\n", start_line.version, number, phrase)
}

fn handle_echo(path: &str, start_line: &StartLine) -> String {
    let reply = path.splitn(3, '/').skip(2).collect::<Vec<&str>>().join("/");
    let status_line = status_code(start_line, HttpRequestCode::Ok);
    let header = RepresentationHeader::new("text/plain", reply.len());
    let formatted_response = format!("{}{}{}\r\n", status_line, header, reply,);

    formatted_response
}

fn user_agent(request_header: &RequestHeader, start_line: &StartLine) -> String {
    if let Some(user_agent) = request_header.get_header("User-Agent") {
        let status_line = status_code(start_line, HttpRequestCode::Ok);
        let header = RepresentationHeader::new("text/plain", user_agent.len());

        format!(
            "{}{}{}\r\n",
            status_line,
            header, // Use Display trait implementation directly
            user_agent
        )
    } else {
        let status_line = status_code(start_line, HttpRequestCode::BadRequest);
        let header = RepresentationHeader::new("text/plain", 0);

        format!("{}{}{}\r\n", status_line, header, "")
    }
}

fn get_file(path: &str, start_line: &StartLine) -> String {
    let file_name = path.replace("/files/", "");
    //get args stolen from https://github.com/junioramilson/codecrafters-http-server-rust/blob/c7abf1b7f330e2b16f5ea3e261bfddc75d39958d/src/main.rs
    let args = std::env::args().collect::<Vec<_>>();
    let file_directory: Option<String> = std::env::args()
        .find(|arg| arg == "--directory")
        .and_then(|arg| std::env::args().nth(&args.iter().position(|x| x == &arg).unwrap() + 1));
    println!("file directory: {:?}", file_directory);

    let file_path = format!("{}{}", file_directory.clone().unwrap(), file_name);
    println!("path: {:?}", file_path);

    match std::fs::read_to_string(&file_path) {
        Ok(file) => {
            let status_line = status_code(start_line, HttpRequestCode::Ok);
            let header = RepresentationHeader::new("application/octet-stream", file.len());
            format!("{}{}{}", status_line, header, file)
        }
        Err(_) => {
            let status_line = status_code(start_line, HttpRequestCode::NotFound);
            let header = RepresentationHeader::new("text/plain", 0);
            format!("{}{}", status_line, header)
        }
    }
}

fn parse_http_request(
    stream: &TcpStream,
) -> Result<(StartLine, RequestHeader, Vec<String>, String), HttpError> {
    let mut buf_reader = BufReader::new(stream);
    let mut header = Vec::new();
    let mut body = String::new();

    loop {
        let mut line = String::new();
        let bytes_read = buf_reader.read_line(&mut line)?;
        if bytes_read == 0 || line == "\r\n" {
            break;
        }
        header.push(line.trim_end().to_string());
    }
    if header.is_empty() {
        return Err(HttpError::HttpParseError(
            "Empty request".to_string(),
            HttpRequestCode::BadRequest,
        ));
    }
    println!("Headers: {:?}", header);

    let start_line = StartLine::new(&header[0])?;
    let request_header = RequestHeader::from_http_request(&header);
    println!("Request: {:#?}", header);
    println!("{:#?}", request_header);

    if let Some(content_length_str) = request_header.get_header("Content-Length") {
        if let Ok(content_length) = content_length_str.parse::<usize>() {
            let mut body_bytes = vec![0; content_length];
            buf_reader.read_exact(&mut body_bytes)?;
            body = String::from_utf8(body_bytes).map_err(|_| {
                HttpError::HttpParseError(
                    "Invalid UTF-8 sequence in body".to_string(),
                    HttpRequestCode::BadRequest,
                )
            })?;
        }
    }
    println!("Body: {}", body);

    Ok((start_line, request_header, header, body))
}

