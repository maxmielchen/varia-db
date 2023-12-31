use std::net::SocketAddr;


use hyper::server::conn::http1::Builder ;
use hyper_util::rt::TokioIo;
use tokio::net::TcpListener;

use log::{error, info};

use super::EngineService;

#[allow(dead_code)]
pub struct WebServer {
    addr: SocketAddr,
    tcp_listener: TcpListener,

    engine_service: EngineService,
}

impl WebServer {

    pub async fn new(engine_service: EngineService, port: u16) -> Self {
        let addr = SocketAddr::from(([0, 0, 0, 0], port));
        let tcp_listener = TcpListener::bind(&addr).await;

        if let Err(_) = tcp_listener {
            error!("Failed to start server on port {}", port);
            panic!("Shutdown");
        }

        let tcp_listener = tcp_listener.unwrap();
    
        Self {
            addr,
            tcp_listener,
            engine_service,
        }
    }

    pub async fn run(self) {
        info!("Successfully started server on port {}", self.addr.port());
        
        loop {
            let accepted = self.tcp_listener.accept().await;
            let service_clone = self.engine_service.clone();
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

