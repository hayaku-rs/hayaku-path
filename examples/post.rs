extern crate hayaku_http;
extern crate hayaku_path;

use hayaku_http::{Http, Request, Response};
use hayaku_path::Router;

use std::sync::Arc;

fn main() {
    let addr = "127.0.0.1:3000".parse().unwrap();

    let mut router = Router::new();
    router.get("/", Arc::new(hello_handler)).unwrap();
    router.post("/", Arc::new(post_handler)).unwrap();

    Http::new(router, ()).threads(4).listen_and_serve(addr);
}

fn hello_handler(_req: &Request, res: &mut Response, _ctx: &()) {
    let data = b"<form method=\"POST\" action=\"\"><input name=\"a\" type=\"text\" /><button name=\"action\" type=\"submit\">Submit</button></form>";
    res.body(data);
}

fn post_handler(req: &Request, _res: &mut Response, _ctx: &()) {
    let body = String::from_utf8_lossy(req.body().unwrap());
    println!("{}", body);
}
