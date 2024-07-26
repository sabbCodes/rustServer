use server::Server;
mod server;

use http::Method;
use http::Request;
mod http;

fn main() {
    let server = Server::new("127.0.0.1:8000".to_string());
    server.run();
}

// GET /search?name=myBook HTTP/1.1