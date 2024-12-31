use std::{io::Read, net::TcpListener};

#[allow(dead_code, unused)]
pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self { addr }
    }

    pub fn run(&self) {
        // Create TCP Listener
        let listener = TcpListener::bind(&self.addr).unwrap();
        println!("Server running on {}", self.addr);

        // Listen incomming requests
        loop {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!(">> REC REQ:{}", String::from_utf8_lossy(&buffer));
                        }
                        Err(err) => println!("Failed to read from connection stream: {}", err),
                    }
                }
                Err(err) => println!("Failed to establish a connection {}", err),
            }
        }
    }
}
