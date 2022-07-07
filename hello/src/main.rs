use std::{
    fs::File,
    io::Read,
    io::Write,
    net::{TcpListener, TcpStream},
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];

    let root = b"GET / ";
    stream.read(&mut buffer).unwrap();

    let (response_code, file) = match buffer.starts_with(root) {
        true => ("200 OK", "hello.html"),
        false => ("404 NOT FOUND", "404.html"),
    };
    let mut file = File::open(file).unwrap();
    let mut file_string = String::new();
    file.read_to_string(&mut file_string).unwrap();
    stream
        .write(format!("HTTP/1.1 {}\r\n\r\n{}", response_code, file_string).as_bytes())
        .unwrap();
    stream.flush().unwrap();
}
