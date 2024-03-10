// Uncomment this block to pass the first stage
use std::{
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
};

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage
    //
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("accepted new connection");
                handle_connection(stream);
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    // let http_request: Vec<_> = buf_reader
    //     .lines()
    //     .map(|res| res.unwrap())
    //     .take_while(|line| !line.is_empty())
    //     .collect();
    let mut buf_reader_lines = buf_reader.lines();
    let request_line = buf_reader_lines.next().unwrap().unwrap();
    let _host_line = buf_reader_lines.next().unwrap().unwrap();
    let user_agent_line = buf_reader_lines.next().unwrap().unwrap();

    let request_items: Vec<&str> = request_line.split_whitespace().collect();
    let user_agent_items: Vec<&str> = user_agent_line.split(": ").collect();

    let method = request_items[0];
    let path = request_items[1];
    let version = request_items[2];
    println!("Method: {}, path: {}, version: {}", method, path, version);

    let user_agent = user_agent_items[1];
    println!("User agent: {}", user_agent);

    let path_params: Vec<&str> = path.split('/').collect();
    println!("Path params: {:?}", path_params);

    let code = match path_params[1] {
        "" | "echo" | "user-agent" => "200 OK",
        _ => "404 NOT FOUND",
    };
    println!("Status code: {}", code);

    let content = match path_params[1] {
        "echo" => {
            let params = path_params.clone();
            echo_response(params)
        }
        "user-agent" => user_agent.to_string(),
        _ => "".to_string(),
    };
    println!("Content: {}", content);

    let response = response_string(code, version, content);
    stream.write_all(response.as_bytes()).unwrap();
}

fn response_string(code: &str, version: &str, content: String) -> String {
    let crlf = "\r\n";
    let content_type = "text/plain";
    let formatted = format!(
        "{} {}{}Content-Type: {}{}Content-Length: {}{}{}{}",
        version,
        code,
        crlf,
        content_type,
        crlf,
        content.len(),
        crlf,
        crlf,
        content
    );
    println!("Formatted response: {:?}", formatted);
    formatted
}

fn echo_response(mut params: Vec<&str>) -> String {
    if params.len() > 2 {
        params.drain(0..2);
    }
    let result: String = params.join("/");
    println!("Echo response: {}", result);
    result
}
