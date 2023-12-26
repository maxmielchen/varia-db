use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

enum Value {
    Text(String),
}

struct Storage {
    store: HashMap<String, Value>,
}

impl Storage {
    fn new() -> Self {
        Self {
            store: HashMap::new(),
        }
    }

    fn set(&mut self, key: String, value: Value) {
        self.store.insert(key, value);
    }

    fn get(&self, key: &str) -> Option<&Value> {
        self.store.get(key)
    }
}
#[tokio::main]
async fn main() {
    println!("Starting server...");

    let storage = Arc::new(Mutex::new(Storage::new()));
    println!("Storage created");

    let listener = TcpListener::bind("127.0.0.1:6543").await.expect("Failed to start server on port 6543");
    println!("Server started on port 6543");

    loop {
        let (mut socket, _) = listener.accept().await.expect("Failed to accept connection");
        let storage = Arc::clone(&storage);

        tokio::spawn(async move {


            let mut buf = [0; 1024];
            

            loop {
                let n = match socket.read(&mut buf).await {
                    Ok(n) if n == 0 => return,
                    Ok(n) => n,
                    Err(e) => {
                        eprintln!("Error reading socket: {:?}", e);
                        return;
                    }
                };

                let input = String::from_utf8_lossy(&buf[0..n]);

                if input.starts_with("get") {
       
                    let key = input.split_whitespace().nth(1);

                    if key == None {
                        eprintln!("No key provided");
                        continue;
                    }

                    let key = key.unwrap();

                    let value = {
                        let storage_lock = storage.lock().unwrap();
                        let value = storage_lock.get(key);
                        match value {
                            Some(Value::Text(value)) => value.clone(),
                            None => {
                                eprintln!("Key not found");
                                continue;
                            }
                        }
                    };

                    if let Err(e) = socket.write_all(
                        format!("{}\n", value).as_bytes()
                    ).await {
                        eprintln!("Error while writing to socket: {:?}", e);
                        return;
                    }
                }

                if input.starts_with("set") {
                    let mut input = input.split_whitespace();
                    input.next();

                    let key = input.next();
                    let value = input.next();

                    if key == None || value == None {
                        eprintln!("No key or value provided");
                        continue;
                    }

                    let key = key.unwrap().to_string();
                    let value = value.unwrap().to_string();

                    let mut storage_lock = storage.lock().unwrap();
                    storage_lock.set(key, Value::Text(value));
                }
            }
        });
    }
}