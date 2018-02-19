extern crate hyper;
extern crate futures;
extern crate url;

mod json;

use futures::future::Future;

use hyper::{Method, StatusCode};
use hyper::server::{Http, Request, Response, Service,};
use std::fs::File;
use std::error::Error;
use std::io::prelude::*;

struct Route;

impl Service for Route {

    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    type Future = Box<Future<Item=Self::Response, Error=Self::Error>>;

    fn call(&self, req: Request) -> Self::Future {
        let mut response = Response::new();

         match (req.method(), req.path()) {
            (&Method::Get, "/") => {
                let path = "./web/index.html";

                let mut file = match File::open(&path) {
                    Err(why) => panic!("couldn't open {}: {}", path, why.description()),
                    Ok(file) => file,
                };
                let mut contents = String::new();
                match file.read_to_string(&mut contents) {
                    Err(why) => panic!("couldn't read {}: {}", path, why.description()),
                    Ok(_) => {
                        response.set_body(contents);
                    }
                }
            },
            (&Method::Post, "/rust.php") => {
                response.set_body(json::json_to_csv());
            },
            _ => {
                response.set_status(StatusCode::NotFound);
            },
        };

        Box::new(futures::future::ok(response))
    }

}


fn main() {
    let addr = "127.0.0.1:3000".parse().unwrap();
    let server = Http::new().bind(&addr, || Ok(Route)).unwrap();
    server.run().unwrap();
}