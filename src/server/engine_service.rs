use std::{sync::{Arc, Mutex}, pin::Pin, future::Future};


use http_body_util::{Full, BodyExt as _};
use hyper::{body::{Bytes, Incoming}, service::Service, Error, Request as HttpRequest, Response as HttpResponse};
use serde_json::Error as SerdeError;


use crate::store::Engine;
use super::{Request, Respond};

use log::error;


#[derive(Clone)]
pub struct EngineService {
    engine: Arc<Mutex<Engine>>,
}

impl EngineService {
    pub fn new(engine: Arc<Mutex<Engine>>) -> Self {
        Self {
            engine,
        }
    }
}


impl Service<HttpRequest<Incoming>> for EngineService {
    type Response = HttpResponse<Full<Bytes>>;

    type Error = Error;

    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn call(&self, req: HttpRequest<Incoming>) -> Self::Future {
        let engine = self.engine.clone();
        Box::pin(async move {
            let mut incoming = req.into_body();

            let mut incoming_bytes: Vec<u8>  = Vec::new();

            while let Some(frame) = incoming.frame().await {
                let frame = frame.unwrap();
                let bytes = frame.into_data();

                if let Err(_) = bytes {
                    break;
                }
                
                incoming_bytes.extend(bytes.unwrap());
            }

            let incoming_request: Result<Request, SerdeError> = serde_json::from_slice(&incoming_bytes);

            if let Err(e) = incoming_request {
                error!("Failed to parse request: {}", e);
                let respond = Respond::Err("Failed to parse request".to_string());
                let respond_bytes = serde_json::to_vec(&respond);
                if let Err(e) = respond_bytes {
                    error!("Failed to serialize response: {}", e);
                    return Ok(HttpResponse::builder().status(500).body("".into()).unwrap());
                }
                let respond_bytes = respond_bytes.unwrap();
                return Ok(HttpResponse::builder().status(400).body(respond_bytes.into()).unwrap());
            }

            let incoming_request = incoming_request.unwrap();

            
            let respond = match incoming_request {
                Request::Put(key, value) => {
                    let engine = engine.lock();
                    if let Err(e) = engine {
                        error!("Failed to lock engine: {}", e);
                        return Ok(HttpResponse::builder().status(500).body("".into()).unwrap());
                    }
                    let mut engine = engine.unwrap();
                    if let Err(e) = engine.put(key, value) {
                        error!("Failed to put value: {}", e);
                        return Ok(HttpResponse::builder().status(500).body("".into()).unwrap());
                    }
                    Respond::Ok
                },
                Request::Get(key) => {
                    let engine = engine.lock();
                    if let Err(e) = engine {
                        error!("Failed to lock engine: {}", e);
                        return Ok(HttpResponse::builder().status(500).body("".into()).unwrap());
                    }
                    let engine = engine.unwrap();
                    let value = engine.get(&key);
                    if let Err(_) = value {
                        Respond::Err("Could not fetch!".to_string())
                    } else {
                        Respond::Value(value.unwrap())
                    }
                },
                Request::Del(key) => {
                    let engine = engine.lock();
                    if let Err(e) = engine {
                        error!("Failed to lock engine: {}", e);
                        return Ok(HttpResponse::builder().status(500).body("".into()).unwrap());
                    }
                    let engine = engine.unwrap();
                    if let Err(e) = engine.del(&key) {
                        error!("Failed to delete value: {}", e);
                        return Ok(HttpResponse::builder().status(500).body("".into()).unwrap());
                    }
                    Respond::Ok
                },
                Request::List => {
                    let engine = engine.lock();
                    if let Err(e) = engine {
                        error!("Failed to lock engine: {}", e);
                        return Ok(HttpResponse::builder().status(500).body("".into()).unwrap());
                    }
                    let engine = engine.unwrap();
                    let list = engine.list();
                    if let Err(e) = list {
                        Respond::Err(format!("Failed to list keys: {}", e).to_string())
                    } else {
                        Respond::Keys(list.unwrap())
                    }
                }
            };

            let respond_bytes = serde_json::to_vec(&respond);

            if let Err(e) = respond_bytes {
                error!("Failed to serialize response: {}", e);
                return Ok(HttpResponse::builder().status(500).body("".into()).unwrap());
            }

            let respond_bytes = respond_bytes.unwrap();
        
            return Ok(HttpResponse::builder().status(200).body(respond_bytes.into()).unwrap());
        })
    }
}