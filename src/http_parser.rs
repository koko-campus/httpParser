

pub mod http_parser {
    //use std::io::BufReader;
    //use std::net::TcpStream;
    use std::io::BufRead;
    use std::{io::BufReader, net::TcpStream};


    pub enum HttpMethod {
        Get,
        Post,
    }

    pub struct HttpRequestStruct {
        method: HttpMethod,
        path: String,  
    }


    pub fn parse(stream: TcpStream) {
        let mut stream_reader = BufReader::new(stream);
    
        let mut first_line = String::new();
        if let Err(err) = stream_reader.read_line(&mut first_line) {
            panic!("error during receive a line: {}", err);
        }
    
        let mut params = first_line.split_whitespace();
        let method = params.next().unwrap();
        let path = params.next().unwrap();

        println!("{}", format!("METHOD -> {}", method));
        println!("{}", format!("PATH -> {}", path));

    }

}

