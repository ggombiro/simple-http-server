use std::net::{TcpListener, TcpStream};

pub struct Server {
    address: String,
}

impl Server {
    pub fn new(address: String) -> Self {
        Self { address }
    }

    pub fn run(self) {
        println!("Listening on {}", self.address);

        let listener = TcpListener::bind(&self.address).unwrap();

        loop{
            let result = listener.accept();

            match result{
                Err(e) =>  println!("Failed to establish a connection: {}", e),
                Ok((stream, _)) => {
                    println!("Ok");
                },
            }
        }
    }
}
