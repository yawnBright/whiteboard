mod server;
mod user;
#[tokio::main]
async fn main() {
    let _server = server::Server::bind("127.0.0.1:8080".to_string());
    _server.run().await;
}
