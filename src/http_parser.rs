

pub mod http_parser {
  //use std::io::BufReader;
  //use std::net::TcpStream;
  use std::io::BufRead;
  use std::{io::BufReader, net::TcpStream};


  #[derive(Debug)]
  pub enum HttpMethod {
    Get,
    Post,
    Unknown,
  }

  pub struct HttpRequestStruct {
    pub method: HttpMethod,
    pub path: String,
  }


  pub fn parse(stream: TcpStream) -> HttpRequestStruct {
    let mut stream_reader = BufReader::new(stream);

    let mut first_line = String::new();
    if let Err(err) = stream_reader.read_line(&mut first_line) {
      panic!("error during receive a line: {}", err);
    }

    let mut params = first_line.split_whitespace();
    let method = params.next().unwrap();
    let path = params.next().unwrap();

    let http_request_data = HttpRequestStruct {
      method: if method == "GET" {HttpMethod::Get} else if method == "POST" {HttpMethod::Post} else {HttpMethod::Unknown},
      path: path.to_string(),
    };

    return http_request_data;
  }

}

