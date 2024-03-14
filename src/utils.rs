use std::{
    fs,
    io::{BufReader, Read},
    net::TcpStream,
};

pub fn response_string(
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

pub fn echo_response(mut params: Vec<&str>) -> String {
    if params.len() > 2 {
        params.drain(0..2);
    }
    let result: String = params.join("/");
    println!("Echo response: {}", result);
    result
}

pub fn get_file_response_header(
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

pub fn post_file_response_header(
    directory: &Option<String>,
    filename: &str,
    mut buf_reader: BufReader<&mut TcpStream>,
    content_length_str: &str,
) -> (String, Option<String>, String) {
    if let Some(dir_path) = directory {
        if let Ok(_metadata) = fs::metadata(dir_path) {
            let content_length: usize = content_length_str.parse().unwrap();
            let mut buffer = vec![0; content_length];
            println!("{:?}", buffer);
            let _ = buf_reader
                .read_exact(&mut buffer)
                .map_err(|err| println!("Error reading file: {:?}", err));
            println!("{:?}", buffer);
            match fs::write(format!("{dir_path}{filename}"), &buffer) {
                Ok(value) => println!("Written {:?}", value),
                Err(err) => println!("Error to write: {err}"),
            }
            (
                "201 CREATED".to_string(),
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
