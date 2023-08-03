use crate::http::api;
use crate::http::HttpStatus;
use crate::http::Request;
use crate::http::Response;
use crate::http::Result;
use std::io::Read;
use std::net::TcpListener;

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

            let response: Response = match request.method() {
                crate::http::Method::GET => match request.path().as_str() {
                    "/api/" => api::index(),
                    "/api/hello" => api::hello(),
                    "/" => {
                        let get_body = include_str!("../web/index.html");
                        // get web page from web/index.html
                        Response::new(HttpStatus::OK, Some(get_body.to_string()))
                    }
                    "/about" => Response::new(
                        HttpStatus::OK,
                        // get error page from web/404.html
                        Some(include_str!("../web/about.html").to_string()),
                    ),

                    _ => Response::new(
                        HttpStatus::NotFound,
                        Some(include_str!("../web/404.html").to_string()),
                    ),
                },
                _ => Response::new(
                    HttpStatus::NotFound,
                    Some(include_str!("../web/404.html").to_string()),
                ),
            };
            response.send(&mut stream)?
        }

        Ok(())
    }
}
