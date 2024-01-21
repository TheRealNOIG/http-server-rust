use std::{
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
};

use http_server_starter_rust::{
    HttpError, HttpRequestCode, RepresentationHeader, RequestHeader, StartLine,
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(_stream) => {
                println!("accepted new connection");
                if let Err(e) = handle_stream(_stream) {
                    println!("Error handling stream: {}", e);
                }
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
    let request_header = RequestHeader::from_http_request(&http_request);
    println!("Headers: {:#?}", request_header);

    let response: String = match start_line.path.as_str() {
        "/" => status_code(&start_line, HttpRequestCode::Ok) + "\r\n",
        path if path.starts_with("/echo/") => handle_echo(path, &start_line),
        path if path.starts_with("/user-agent") => user_agent(&request_header, &start_line),
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

