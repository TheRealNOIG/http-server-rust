use std::{
    io::Write,
    net::{TcpListener, TcpStream},
};

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

fn handle_stream(mut stream: TcpStream) {
    let response = b"HTTP/1.1 200 OK\r\n\r\n";

    match stream.write(response) {
        Ok(_) => {
            stream.flush().unwrap();
        }
        Err(err) => {
            eprintln!("Error writing to stream: {}", err);
        }
    }
}

