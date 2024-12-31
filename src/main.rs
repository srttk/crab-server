#[allow(unused_imports)]
use http::HttpMethod;
#[allow(unused_imports)]
use http::Request;
mod env;
mod http;
mod server;
fn main() {
    let port = env::get_port();
    let addr = format!("0.0.0.0:{}", port);
    let server = server::Server::new(addr);
    server.run();
}
