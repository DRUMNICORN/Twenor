mod client;
use client::Client;

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut client = Client::new();

    client.set_on_server_connected(|| {
        println!("Connected to server");
    });

    client.set_on_message_received(|message| {
        println!("Received message from server: '{}'", message);
    });

    println!("Connecting to server...");
    client.connect("127.0.0.1:8080").await?;

    println!("Sending message to server: 'Hello, server!'");
    client.send("Hello, server!").await?;

    println!("Starting to receive messages from server...");
    client.start().await?;

    println!("Disconnected from server");

    Ok(())
}