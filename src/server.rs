use std::io::{Read, Write};
use crate::http::{Request, Response, StatusCode};
use std::net::{TcpListener, TcpStream};

use std::convert::TryFrom;

pub struct Server {
    addr: String
}

impl Server {
    pub fn new(addr: String) -> Server {
        Server { addr }
    }

    pub fn run(self){
        println!("Listening on {}", self.addr);

        let listener = TcpListener::bind(&self.addr).unwrap();

        loop {
            let res = listener.accept();

            match res {
                Ok((mut stream, addr)) => {
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            let data_received = String::from_utf8_lossy(&buffer);
                            // println!("Received data {}", data_received);
                            match Request::try_from(&buffer[..]) {
                                Ok(req)=>{
                                    println!("Request = {:?}", req);
                                    let res = Response::new(
                                        StatusCode::OK,
                                        Some("<h1>Hello World, I built my first server successfully!!<h1>".to_string()),
                                    );
                                    res.send(&mut stream);
                                }
                                Err(_)=>{
                                    let res = Response::new(StatusCode::BadRequest, None);
                                    res.send(&mut stream);
                                }
                            }
                        }
                        Err(e) => println!("Failed to read from connection {}", e)
                    }
                },
                Err(e)=>{
                    println!("Error occured {}", e)
                }
            }
        }
    }
}