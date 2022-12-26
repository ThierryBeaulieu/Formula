use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;
use std::fs;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming(){
        let stream = stream.unwrap();
        println!("A connection was made");
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    // Faire l'impression des requête
    println!(
        "Request: {}",
        String::from_utf8_lossy(&buffer[..])
    );

    let get = b"GET / HTTP/1.1\r\n";
    let get_javascript = b"GET /script.js HTTP/1.1\r\n";
    let get_css = b"GET /styles.css HTTP/1.1\r\n";
    let status_line;
    let contents;
    
    if buffer.starts_with(get) {
        contents = fs::read_to_string("index.html").unwrap();
        status_line = "HTTP/1.1 200 OK";
    } else if buffer.starts_with(get_css) {
        contents = fs::read_to_string("styles.css").unwrap();
        status_line = "HTTP/1.1 200 OK";
    } else if buffer.starts_with(get_javascript) {
        contents = fs::read_to_string("script.js").unwrap();
        status_line = "HTTP/1.1 200 OK";
    } else {
        contents = fs::read_to_string("error.html").unwrap();
        status_line = "HTTP/1.1 404 NOT FOUND";
    }
    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}