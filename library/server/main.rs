use std::env;
use std::net::SocketAddr;

mod server;
use server::Server;

#[tokio::main]
async fn main() {
    let addr: SocketAddr = env
        ::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:8080".parse().unwrap())
        .parse()
        .unwrap();

    let mut server = Server::new(&addr.to_string()).await;

    server.set_on_client_connected(|client_id| {
        println!("Client {} connected", client_id);
    });

    server.set_on_message_received(|message, client_id| {
        println!("Received message '{}' from client {}", message, client_id);
    });

    server.run().await;
}