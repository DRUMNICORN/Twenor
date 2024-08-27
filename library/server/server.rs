use std::collections::HashMap;
use std::sync::{ Arc, Mutex };
use tokio::net::TcpListener;
use tokio::prelude::*;
use tokio::sync::mpsc;

pub struct Server {
    listener: TcpListener,
    clients: Arc<Mutex<HashMap<usize, mpsc::UnboundedSender<String>>>>,
    on_message_received: Arc<Mutex<Option<fn(String, usize) -> ()>>>,
    on_client_connected: Arc<Mutex<Option<fn(usize) -> ()>>>,
}

impl Server {
    pub async fn new(addr: &str) -> Server {
        let listener = TcpListener::bind(addr).await.unwrap_or_else(|e| {
            eprintln!("Unable to bind address: {}", e);
            std::process::exit(1);
        });

        let clients = Arc::new(Mutex::new(HashMap::new()));
        let on_message_received = Arc::new(Mutex::new(None));
        let on_client_connected = Arc::new(Mutex::new(None));
        Server {
            listener,
            clients,
            on_message_received,
            on_client_connected,
        }
    }

    pub fn set_on_message_received(&mut self, f: fn(String, usize) -> ()) {
        let mut on_message_received = self.on_message_received.lock().unwrap();
        *on_message_received = Some(f);
    }

    pub fn set_on_client_connected(&mut self, f: fn(usize) -> ()) {
        let mut on_client_connected = self.on_client_connected.lock().unwrap();
        *on_client_connected = Some(f);
    }

    pub async fn run(self) {
        let Server { mut listener, clients, on_message_received, on_client_connected } = self;

        let mut id_counter = 0;

        loop {
            let (socket, _) = listener.accept().await.unwrap();
            let id = id_counter;
            id_counter += 1;

            let (tx, rx) = mpsc::unbounded_channel();
            {
                let mut clients = clients.lock().unwrap();
                clients.insert(id, tx);
            }

            let on_message_received = on_message_received.clone();
            let on_client_connected = on_client_connected.clone();
            let clients = clients.clone();
            tokio::spawn(async move {
                let mut socket = socket;
                let (mut rx, mut tx) = socket.split();

                let mut buf = vec![0; 1024];
                let mut incoming = String::new();

                if let Some(f) = on_client_connected.lock().unwrap().as_ref() {
                    f(id);
                }

                loop {
                    let n = match rx.read(&mut buf).await {
                        Ok(n) if n == 0 => {
                            break;
                        }
                        Ok(n) => n,
                        Err(e) => {
                            eprintln!("Error reading from socket: {}", e);
                            break;
                        }
                    };

                    incoming.extend(buf[..n].iter().map(|&b| b as char));

                    if incoming.ends_with('\n') {
                        let message = incoming.trim_end_matches('\n').to_string();
                        incoming.clear();

                        if let Some(f) = on_message_received.lock().unwrap().as_ref() {
                            f(message, id);
                        }
                    }
                }

                let mut clients = clients.lock().unwrap();
                clients.remove(&id);
            });
        }
    }

    pub fn send_message(&self, client_id: usize, message: String) {
        let mut clients = self.clients.lock().unwrap();
        if let Some(tx) = clients.get(&client_id) {
            tx.send(message).unwrap();
        }
    }
}