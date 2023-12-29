use std::{sync::{Arc, Mutex}, net::SocketAddr};


use hyper::server::conn::http1::Builder ;
use hyper_util::rt::TokioIo;
use tokio::net::TcpListener;

use log::{error, info};

use crate::store::Engine;

use super::EngineService;

#[allow(dead_code)]
pub struct WebServer {
    engine: Arc<Mutex<Engine>>,
    addr: SocketAddr,
    tcp_listener: TcpListener,

    service: EngineService,
}

impl WebServer {

    pub async fn new(engine: Engine, port: u16) -> Self {
        let engine = Arc::new(Mutex::new(engine));
        let addr = SocketAddr::from(([127, 0, 0, 1], port));
        let tcp_listener = TcpListener::bind(&addr).await;

        if let Err(_) = tcp_listener {
            error!("Failed to start server on port {}", port);
            panic!("Shutdown");
        }

        let tcp_listener = tcp_listener.unwrap();

        let service = EngineService::new(Arc::clone(&engine));
    
        Self {
            engine,
            addr,
            tcp_listener,
            service
        }
    }

    pub async fn run(self) {
        info!("Successfully started server on port {}", self.addr.port());
        
        loop {
            let accepted = self.tcp_listener.accept().await;
            let service_clone = self.service.clone();
            tokio::task::spawn(async move {

                if let Err(_) = accepted {
                    error!("Failed to accept connection");
                    return;
                }

                let (tcp_stream, _) = accepted.unwrap();

                let io = TokioIo::new(tcp_stream);
            
                let builder = Builder::new();

                let conn = builder.serve_connection(io, service_clone);

                if let Err(_) = conn.await {
                    error!("Failed to serve connection");
                    return;
                }
                
            });   
        }
    }
    
}

