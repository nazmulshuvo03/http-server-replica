use std::{
    env,
    io::{BufRead, BufReader, Read, Write},
    net::{TcpListener, TcpStream},
    thread,
};

mod utils;
use crate::utils::*;

fn main() {
    println!("Logs from your program will appear here!");

    let args: Vec<String> = env::args().collect();
    println!("Args: {:?}", args);

    let directory: Option<String> = if args.len() > 2 {
        Some(args.get(2).unwrap().clone())
    } else {
        None
    };
    println!("Directory provided: {:?}", directory);

    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("accepted new connection");
                let dir_clone = directory.clone();
                thread::spawn(move || {
                    handle_connection(stream, dir_clone);
                });
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

fn handle_connection(mut stream: TcpStream, directory: Option<String>) {
    let mut buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .by_ref()
        .lines()
        .map(|res| res.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();
    println!("HTTP Request: {:?}", http_request);

    let request_line = &http_request[0];
    let _host_line = if http_request.len() > 1 {
        &http_request[1]
    } else {
        ""
    };
    let user_agent_line = if http_request.len() > 2 {
        &http_request[2]
    } else {
        ""
    };
    let content_length_line = if http_request.len() > 3 {
        &http_request[3]
    } else {
        ""
    };

    let request_items: Vec<&str> = request_line.split_whitespace().collect();
    let user_agent_items: Vec<&str> = user_agent_line.split(": ").collect();
    let content_length_items: Vec<&str> = content_length_line.split(": ").collect();

    let method = request_items[0];
    let path = request_items[1];
    let version = request_items[2];
    println!("Method: {}, path: {}, version: {}", method, path, version);

    let user_agent = if user_agent_items.len() > 1 {
        user_agent_items[1]
    } else {
        ""
    };
    println!("User agent: {}", user_agent);

    let content_length = if content_length_items.len() > 1 {
        content_length_items[1]
    } else {
        ""
    };
    println!("Content length: {}", content_length);

    let path_params: Vec<&str> = path.split('/').collect();
    println!("Path params: {:?}", path_params);

    let (mut file_response_code, mut file_response_content_type, mut file_content) =
        ("".to_string(), None, "".to_string());
    if path_params.len() > 2 {
        if method == "POST" {
            (file_response_code, file_response_content_type, file_content) =
                post_file_response_header(&directory, path_params[2], buf_reader, content_length);
            println!(
                "post file response code: {:?}, file response content type: {:?}",
                file_response_code, file_response_content_type
            );
        } else {
            (file_response_code, file_response_content_type, file_content) =
                get_file_response_header(&directory, path_params[2]);
            println!(
                "get file response code: {:?}, file response content type: {:?}",
                file_response_code, file_response_content_type
            );
        }
    }

    let code = match path_params[1] {
        "" | "echo" | "user-agent" => "200 OK".to_string(),
        "files" => file_response_code,
        _ => "404 NOT FOUND".to_string(),
    };
    println!("Status code: {}", code);

    let content = match path_params[1] {
        "echo" => {
            let params = path_params.clone();
            echo_response(params)
        }
        "user-agent" => user_agent.to_string(),
        "files" => file_content,
        _ => "".to_string(),
    };
    println!("Content: {}", content);

    let response = response_string(code, version, content, file_response_content_type);
    stream.write_all(response.as_bytes()).unwrap();
}
