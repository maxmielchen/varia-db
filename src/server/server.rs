use std::{sync::{Arc, Mutex}, net::SocketAddr, convert::Infallible};


use http_body_util::{Full, BodyExt};
use hyper::{body::{Bytes, Incoming}, server::conn::http1::Builder, service::service_fn, Request, Response};
use hyper_util::rt::TokioIo;
use tokio::net::TcpListener;

use crate::store::Engine;
use log::{error, info};

#[allow(dead_code)]
pub struct Server {
    engine: Arc<Mutex<Engine>>,
    addr: SocketAddr,
    tcp_listener: TcpListener,
}

impl Server {

    pub async fn new(engine: Engine, port: u16) -> Self {
        info!("Setting up server...");
        let engine = Arc::new(Mutex::new(engine));

        info!("Starting server on port {}...", port);
        let addr = SocketAddr::from(([127, 0, 0, 1], port));
        let tcp_listener = TcpListener::bind(&addr).await;

        if let Err(_) = tcp_listener {
            error!("Failed to start server on port {}", port);
            panic!("Shutdown");
        }

        let tcp_listener = tcp_listener.unwrap();

        info!("Server listening on port {}", port);
    
        Self {
            engine,
            addr,
            tcp_listener,
        }
    }

    pub async fn run(self) {
        info!("Successfully started!");
        
        loop {
            let accepted = self.tcp_listener.accept().await;

            tokio::task::spawn(async move {

                if let Err(_) = accepted {
                    error!("Failed to accept connection");
                    return;
                }

                let (tcp_stream, _) = accepted.unwrap();

                let io = TokioIo::new(tcp_stream);

                let service = service_fn(Self::service);
                
                let builder = Builder::new();

                let conn = builder.serve_connection(io, service);

                if let Err(_) = conn.await {
                    error!("Failed to serve connection");
                    return;
                }
                
            });   
        }
    }

    async fn service(request: Request<Incoming>) -> Result<Response<Full<Bytes>>, Infallible> {
        let mut incoming = request.into_body();

        let mut incoming_bytes: Vec<u8>  = Vec::new();

        while let Some(frame) = incoming.frame().await {
            let frame = frame.unwrap();
            let bytes = frame.into_data();

            if let Err(_) = bytes {
                break;
            }
            
            incoming_bytes.extend(bytes.unwrap());
        }

        info!("Received request: {:?}", String::from_utf8(incoming_bytes.clone()));

        let response = Response::builder()
            .status(200)
            .body(Full::new(incoming_bytes.into()))
            .unwrap();

        Ok(response)
    }
    
}