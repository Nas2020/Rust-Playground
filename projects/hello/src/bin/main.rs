use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;
use std::time::Duration;

use hello::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();

        // TcpStream is Send so it is safe to send across threads.
        pool.execute(|| {
            handle_connection(stream);
        });
    }

    // Gracefully shut down the thread pool.
    drop(pool);

    println!("Shutting down.");
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    let bytes_read = stream.read(&mut buffer).unwrap();

    // This is a basic example and assumes that the entire request will be read in one call to read.
    // In a production scenario, you would need to handle partial reads and implement proper HTTP request parsing.
    let request = String::from_utf8_lossy(&buffer[..bytes_read]);
    if let Some(index) = request.find("\r\n\r\n") {
        let header = &request[..index]; // Get the request header

        let get = "GET / HTTP/1.1\r\n";
        let sleep = "GET /sleep HTTP/1.1\r\n";

        let (status_line, filename) = if header.starts_with(get) {
            ("HTTP/1.1 200 OK", "hello.html")
        } else if header.starts_with(sleep) {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "hello.html")
        } else {
            ("HTTP/1.1 404 NOT FOUND", "404.html")
        };

        let contents = fs::read_to_string(filename).unwrap();

        let response = format!(
            "{}\r\nContent-Length: {}\r\n\r\n{}",
            status_line,
            contents.len(),
            contents
        );

        stream.write_all(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}
