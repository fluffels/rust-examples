use http::Request;
use http::Method;
use server::Server;

mod http;
mod server;

fn main() {
    let addr = String::from(
        "0.0.0.0:8080"
    );
    let server = Server::new(
        addr
    );
    server.run();
}
