use http_server::http::Server;
fn main() {
    let server = Server::new("localhost:8000".to_string());
    if let Err(e) = server.run() {
        println!("Error {}", e);
    }
}
