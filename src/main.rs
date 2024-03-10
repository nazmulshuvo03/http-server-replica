use std::{
    env,
    fs::{self},
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
    thread,
};

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

    let user_agent = if user_agent_items.len() > 1 {
        user_agent_items[1]
    } else {
        ""
    };
    println!("User agent: {}", user_agent);

    let path_params: Vec<&str> = path.split('/').collect();
    println!("Path params: {:?}", path_params);

    let (mut file_response_code, mut file_response_content_type, mut file_content) =
        ("".to_string(), None, "".to_string());
    if path_params.len() > 2 {
        (file_response_code, file_response_content_type, file_content) =
            get_file_response_header(&directory, path_params[2]);
        println!(
            "file response code: {:?}, file response content type: {:?}",
            file_response_code, file_response_content_type
        );
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
        // "files" => get_file_content(directory, path_params[2]),
        "files" => file_content,
        _ => "".to_string(),
    };
    println!("Content: {}", content);

    let response = response_string(code, version, content, file_response_content_type);
    stream.write_all(response.as_bytes()).unwrap();
}

fn response_string(
    code: String,
    version: &str,
    content: String,
    custom_content_type: Option<String>,
) -> String {
    let crlf = "\r\n";
    let content_type = custom_content_type.unwrap_or("text/plain".to_string());
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

fn get_file_response_header(
    directory: &Option<String>,
    filename: &str,
) -> (String, Option<String>, String) {
    if let Some(dir_path) = directory {
        if let Ok(_metadata) = fs::metadata(format!("{}{}", dir_path.as_str(), filename)) {
            (
                "200 OK".to_string(),
                Some("application/octet-stream".to_string()),
                fs::read_to_string(format!("{}{}", dir_path.as_str(), filename)).unwrap(),
            )
        } else {
            println!("Directory does not exist at path: {}", dir_path);
            ("404 NOT FOUND".to_string(), None, "".to_string())
        }
    } else {
        println!("Directory path is not specified.");
        ("404 NOT FOUND".to_string(), None, "".to_string())
    }
}
