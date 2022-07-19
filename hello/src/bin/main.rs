use std::{
    fs::File,
    io::Read,
    io::Write,
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

use hello::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let threadpool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        threadpool.execute(|| handle_connection(stream));
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];

    let root = b"GET / ";
    let sleep = b"GET /sleep ";
    stream.read(&mut buffer).unwrap();

    let (response_code, file) = if buffer.starts_with(root) {
        ("200 OK", "hello.html")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        ("200 OK", "hello.html")
    } else {
        ("404 NOT FOUND", "404.html")
    };
    let mut file = File::open(file).unwrap();
    let mut file_string = String::new();
    file.read_to_string(&mut file_string).unwrap();
    stream
        .write(format!("HTTP/1.1 {}\r\n\r\n{}", response_code, file_string).as_bytes())
        .unwrap();
    stream.flush().unwrap();
}
