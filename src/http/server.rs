use crate::http::Request;
use crate::http::Result;
use std::io::Read;
use std::net::TcpListener;
use std::str;

#[derive(std::fmt::Debug)]

pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self { addr }
    }
    pub fn run(&self) -> Result<()> {
        println!("listening on {}", self.addr);
        let listener = TcpListener::bind(&self.addr)?;

        for stream in listener.incoming() {
            let mut stream = stream?;
            let mut buf: [u8; 1024] = [0; 1024];
            stream.read(&mut buf)?;
            let request = Request::try_from(&buf[..])?;
            println!("{:#?}", request);

            match request.method() {
                crate::http::Method::GET => match request.path().as_str() {
                    "/" => {}
                    "/hello" => {}
                    _ => {}
                },
                _ => {}
            }
        }

        Ok(())
    }
}
