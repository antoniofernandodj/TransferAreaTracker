use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::iter::Iterator;

struct Server {
    listener: TcpListener,
    conn: Option<TcpStream>,
}

impl Server {
    fn new(host: &str, port: u16) -> Self {
        let listener = TcpListener::bind((host, port)).expect("Could not bind");
        println!("Server listening on {}:{}", host, port);
        Server { listener, conn: None }
    }

    fn accept(&mut self) {
        match self.listener.accept() {
            Ok((conn, addr)) => {
                println!("Connected by {:?}", addr);
                self.conn = Some(conn);
            }
            Err(e) => {
                eprintln!("Could not accept connection: {}", e);
            }
        }
    }

    fn send(&mut self, data: &[u8]) {
        if let Some(ref mut conn) = self.conn {
            conn.write_all(data).expect("Failed to send data");
        }
    }
}

impl Iterator for Server {
    type Item = Vec<u8>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut buffer = [0; 1024];
        if let Some(ref mut conn) = self.conn {
            match conn.read(&mut buffer) {
                Ok(0) => {
                    // Connection closed by client
                    self.conn = None;
                    None
                }
                Ok(n) => Some(buffer[..n].to_vec()),
                Err(e) => {
                    eprintln!("Failed to read from connection: {}", e);
                    None
                }
            }
        } else {
            self.accept();
            self.next()
        }
    }
}

fn main() {
    let mut server = Server::new("127.0.0.1", 65430);
    for packet in &mut server {
        let data = String::from_utf8_lossy(&packet);
        println!("{}", data);
        server.send(&packet);
    }
}
