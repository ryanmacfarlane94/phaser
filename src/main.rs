use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream}, fs,
};

use phaser::util::server_error::ServerError;
use phaser::http::request::Request;

fn main() {
    let bind_result = TcpListener::bind("127.0.0.1:7878");
    let listener = match bind_result {
        Ok(listener) => listener,
        Err(_) => panic!("Error binding to port 7878")
    };

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }

    fn handle_connection(mut stream: TcpStream) {
        let buffer_reader = BufReader::new(&mut stream);
        let http_request: Vec<_> = buffer_reader
            .lines()
            .map(|result| result.unwrap())
            .take_while(|line| !line.is_empty())
            .collect();

        let request = match Request::new(http_request) {
            Ok(request) => request,
            Err(error) => {
                let bad_request = error.get_response_start();
                stream.write_all(bad_request.as_bytes()).unwrap();
                return;
            }
        };



        let mut file_contents = fs::read_to_string("index.html").unwrap();        

        let status_line = "HTTP/1.1 200 OK";
        let length = file_contents.len();

        let header_line = 
            format!("Content-Type: text/html; charset=UTF-8\nContent-Length: {length}\nServer: phaser");
        
        let response =
            format!("{status_line}\r\n{header_line}\r\n\r\n{file_contents}");

        stream.write_all(response.as_bytes()).unwrap();

    }
}
