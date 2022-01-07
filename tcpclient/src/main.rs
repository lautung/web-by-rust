use std::io::{Read, Write};
use std::net::TcpStream;
use std::str;

fn main() {
    let mut stream = TcpStream::connect("localhost:3000").unwrap();
    stream.write("Hello World!".as_bytes()).unwrap();

    let mut buffer = [0;15];
    stream.read(&mut buffer).unwrap();
    println!("Response from server:{:?}",str::from_utf8(&buffer).unwrap());
}
