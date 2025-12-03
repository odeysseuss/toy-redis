use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};

fn handle_conn(mut stream: TcpStream) {
    let mut buf = [0u8; 64];
    let wbuf = b"Hola, sparrow\n";

    match stream.read_exact(&mut buf) {
        Ok(_) => {
            match str::from_utf8(&buf) {
                Ok(s) => eprintln!("Client says: {s}"),
                Err(_) => eprintln!("Client sent non ut8 data"),
            }

            if let Err(e) = stream.write_all(wbuf) {
                eprintln!("write: {e}");
            }
        }
        Err(e) => eprintln!("read: {e}"),
    }
}

fn main() {
    let listener = TcpListener::bind("0.0.0.0:1234").expect("Failed to bind to port 1234");

    loop {
        match listener.accept() {
            Ok((stream, addr)) => {
                eprintln!("Connected with: {addr}");
                handle_conn(stream);
            }
            Err(e) => {
                eprintln!("accept: {e}");
                continue;
            }
        }
    }
}
