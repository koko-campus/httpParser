use std::env;
use dotenv::dotenv;
use std::io::prelude::*;
use std::net::TcpStream;
use std::net::TcpListener;


mod http_parser;

fn main() {
	dotenv().ok();

	let ip_address = env::var("IP_ADDRESS").expect("IP_ADDRESS is not defined");
	let http_port = env::var("HTTP_PORT").expect("HTTP_PORT is not defined");
	println!("{}", format!("IP_ADDRESS -> {}", ip_address));
	println!("{}", format!("HTTP_PORT -> {}", http_port));
	tcp_listener(ip_address, http_port)
}


fn tcp_listener(ip_address: String, http_port: String) {
	let listener = TcpListener::bind(format!("{}:{}", ip_address, http_port)).unwrap();

	for stream in listener.incoming() {
		let stream = stream.unwrap();

		for stream in listener.incoming() {
			let stream = stream.unwrap();
	
			handle_connection(stream);
		}
	}
}


fn handle_connection(mut stream: TcpStream) {
	let mut buffer = [0; 1024];

	stream.read(&mut buffer).unwrap();

	println!(" ::::: REQUEST START ::::: ");
	println!("{}", String::from_utf8_lossy(&buffer[..]));
	println!(" ::::: REQUEST END... ::::: ");

	let response = "HTTP/1.1 200 OK\r\n\r\nHELLO";
	stream.write(response.as_bytes()).unwrap();
	stream.flush().unwrap();
}

