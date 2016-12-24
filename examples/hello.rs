#![feature(proc_macro)]

extern crate hayaku_http;
extern crate hayaku_path;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use hayaku_http::{Http, Request, Response};
use hayaku_path::Router;

use std::sync::Arc;

fn main() {
    let addr = "127.0.0.1:3000".parse().unwrap();

    let mut router = Router::new();
    router.get("/", Arc::new(hello_handler)).unwrap();
    router.get("/plaintext", Arc::new(plain_handler)).unwrap();
    router.get("/json", Arc::new(json_handler)).unwrap();

    Http::new(router, ()).threads(4).listen_and_serve(addr);
}

fn hello_handler(_req: &Request, res: &mut Response, _ctx: &()) {
    res.body(b"hello, world!");
}

fn plain_handler(_req: &Request, res: &mut Response, _ctx: &()) {
    res.add_header("Content-Type".to_string(), "text/plain".to_string());
    res.body(b"hello, world!");
}

#[derive(Serialize, Deserialize)]
struct Message {
    message: String,
}

fn json_handler(_req: &Request, res: &mut Response, _ctx: &()) {
    let msg = Message { message: "Hello, World!".to_string() };
    let data = serde_json::to_vec(&msg).unwrap();

    res.add_header("Content-Type".to_string(), "application/json".to_string());
    res.body(&data);
}
