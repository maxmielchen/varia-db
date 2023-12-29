use std::{sync::{Arc, Mutex}, pin::Pin, future::Future};


use http_body_util::{Full, BodyExt as _};
use hyper::{body::{Bytes, Incoming}, service::Service, Error, Request as HttpRequest, Response as HttpResponse};
use serde_json::Error as SerdeError;


use crate::store::Engine;
use super::{Request as DeserializedRequest, Respond as SerializedRespond};

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
        let engine_clone = self.engine.clone();
        Box::pin(async move {

            let deserialized_request = bytes_to_deserialized_request(
                http_request_to_bytes(req).await
            );

            if let Err(e) = deserialized_request {
                let msg = format!("Failed to parse request: {}", e);
                error!("{}", msg);
                return Ok(
                    text_to_http_response(
                        msg,
                        400
                    )
                );
            }

            let deserialized_request = deserialized_request.unwrap();

            let engine = engine_clone.lock();
            if let Err(e) = &engine {
                let msg = format!("Failed to lock engine: {}", e);
                error!("{}", msg);
                return Ok(
                    text_to_http_response(
                        msg,
                        500
                    )
                );
            }
            let mut engine = engine.unwrap();

            let serialized_respond = match deserialized_request {
                DeserializedRequest::Put(key, value) => {
           
                    if let Err(e) = engine.put(key, value) {
                        let msg = format!("Failed to put value: {}", e.to_string());
                        error!("{}", msg);
                        return Ok(
                            text_to_http_response(
                                msg,
                                500
                            )
                        );
                    }

                    SerializedRespond::Ok
                },
                DeserializedRequest::Get(key) => {
                    let value = engine.get(&key);

                    if let Err(e) = value {
                        let msg = format!("Failed to get value: {}", e.to_string());
                        error!("{}", msg);
                        return Ok(
                            text_to_http_response(
                                msg,
                                500
                            )
                        );
                    } 

                    SerializedRespond::Value(value.unwrap().clone())
                },
                DeserializedRequest::Del(key) => {

                    if let Err(e) = engine.del(&key) {
                        let msg = format!("Failed to delete value: {}", e.to_string());
                        error!("{}", msg);
                        return Ok(
                            text_to_http_response(
                                msg,
                                500
                            )
                        );
                    }

                    SerializedRespond::Ok
                },
                DeserializedRequest::List => {
                    let list = engine.list();

                    if let Err(e) = list {
                        let msg = format!("Failed to list values: {}", e.to_string());
                        error!("{}", msg);
                        return Ok(
                            text_to_http_response(
                                msg,
                                500
                            )
                        );
                    }

                    SerializedRespond::Keys(list.unwrap().clone())
                }
            };

            return Ok(
                bytes_to_http_response(
                    serialized_respond_to_bytes(
                        serialized_respond
                    )
                )
            );
        })
    }
}

async fn http_request_to_bytes(req: HttpRequest<Incoming>) -> Vec<u8> {
    let mut body = req.into_body();

    let mut bytes_vec: Vec<u8>  = Vec::new();

    while let Some(frame) = body.frame().await {
        let frame = frame.unwrap();
        let bytes = frame.into_data();

        if let Err(_) = bytes {
            break;
        }
        
        bytes_vec.extend(bytes.unwrap());
    }

    bytes_vec
}

fn bytes_to_http_response(bytes: Vec<u8>) -> HttpResponse<Full<Bytes>> {
    HttpResponse::builder().status(200).body(bytes.into()).unwrap()
}

fn text_to_http_response(text: String, exit: u16) -> HttpResponse<Full<Bytes>> {
    HttpResponse::builder().status(exit).body(text.into()).unwrap()
}

fn bytes_to_deserialized_request(bytes: Vec<u8>) -> Result<DeserializedRequest, SerdeError> {
    serde_json::from_slice(&bytes)
}

fn serialized_respond_to_bytes(res: SerializedRespond) -> Vec<u8> {
    let bytes = serde_json::to_vec(&res);
    bytes.expect("Failed to serialize respond")
}