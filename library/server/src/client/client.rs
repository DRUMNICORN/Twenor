use std::io::{ self, BufRead, BufReader, Write };
use std::net::TcpStream;
use std::sync::{ Arc, Mutex };
use tokio::prelude::*;

pub struct Client {
    stream: TcpStream,
    rx: BufReader<TcpStream>,
    tx: TcpStream,
    on_message_received: Arc<Mutex<Option<Box<dyn Fn(String) + Send + Sync>>>>,
    on_server_connected: Arc<Mutex<Option<Box<dyn Fn() + Send + Sync>>>>,
}

impl Client {
    pub fn new() -> Client {
        Client {
            stream: TcpStream::new(),
            rx: BufReader::new(TcpStream::new()),
            tx: TcpStream::new(),
            on_message_received: Arc::new(Mutex::new(None)),
            on_server_connected: Arc::new(Mutex::new(None)),
        }
    }

    pub async fn connect(&mut self, addr: &str) -> io::Result<()> {
        self.stream = TcpStream::connect(addr).await?;
        let (rx, tx) = self.stream.split();
        self.rx = BufReader::new(rx);
        self.tx = tx;

        if let Some(f) = self.on_server_connected.lock().unwrap().as_ref() {
            f();
        }

        Ok(())
    }

    pub fn set_on_message_received<F>(&mut self, f: F) where F: Fn(String) + Send + Sync + 'static {
        *self.on_message_received.lock().unwrap() = Some(Box::new(f));
    }

    pub fn set_on_server_connected<F>(&mut self, f: F) where F: Fn() + Send + Sync + 'static {
        *self.on_server_connected.lock().unwrap() = Some(Box::new(f));
    }

    pub async fn send(&mut self, message: &str) -> io::Result<()> {
        self.tx.write_all(message.as_bytes()).await?;
        self.tx.write_all(b"\n").await?;
        Ok(())
    }

    pub async fn receive(&mut self) -> io::Result<()> {
        let mut incoming = String::new();
        loop {
            match self.rx.read_line(&mut incoming).await {
                Ok(0) => {
                    break;
                }
                Ok(_) => {
                    if incoming.ends_with('\n') {
                        let message = incoming.trim_end_matches('\n').to_string();
                        incoming.clear();
                        if let Some(f) = self.on_message_received.lock().unwrap().as_ref() {
                            f(message);
                        }
                    }
                }
                Err(e) => {
                    return Err(e);
                }
            }
        }
        Ok(())
    }

    pub async fn start(&mut self) -> io::Result<()> {
        loop {
            match self.rx.read_line(&mut incoming).await {
                Ok(0) => {
                    break;
                }
                Ok(_) => {
                    if incoming.ends_with('\n') {
                        let message = incoming.trim_end_matches('\n').to_string();
                        incoming.clear();
                        if let Some(f) = self.on_message_received.lock().unwrap().as_ref() {
                            f(message);
                        }
                    }
                }
                Err(e) => {
                    return Err(e);
                }
            }
        }
        Ok(())
    }
}