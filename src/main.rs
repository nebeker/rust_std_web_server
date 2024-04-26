use std::{
    fs,
    io::{Read, Write},
    net::{SocketAddr, TcpStream},
    thread,
};

fn get_response(request: &str) -> Result<String, &'static str> {
    if request.contains("GET /") {
        Ok(read_file("index.html"))
    } else {
        Err("No route")
    }
}

fn read_file(filename: &str) -> String {
    match fs::read_to_string(filename) {
        Ok(text) => format!("HTTP 1.1 200 OK\r\nContent-Type: text/html\r\n\r\n{}", text),
        Err(e) => {
            eprint!("Error reading file: {e:?}");
            String::from("HTTP 1.1 500 Internal Server Error\r\n500 Internal Server Error")
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    println!("Connected to client: {stream:?}");
    let mut buffer = [0; 1024];
    match stream.read(&mut buffer) {
        Ok(_) => {
            let request = String::from_utf8_lossy(&buffer);
            print!("request: {}", request);
            if let Ok(response) = get_response(&request) {
                println!("response: {response}");
                match stream.write_all(response.as_bytes()) {
                    Ok(_) => match stream.flush() {
                        Ok(_) => println!("Sent response to client"),
                        Err(e) => eprintln!("Failed to send response to client: {e:?}"),
                    },
                    Err(e) => eprintln!("Failed to send response to client: {e:?}"),
                }
            }
        }
        Err(e) => {
            eprintln!("Error: {e:?}")
        }
    }
}

fn main() {
    let listener = std::net::TcpListener::bind("127.0.0.1:8080").unwrap();

    println!("Listening on 127.0.0.1:8080 - Ctrl+C to stop");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| {
                    handle_connection(stream);
                });
            }
            Err(e) => println!("Failed to connect to client: {e:?}"),
        }
    }
}
