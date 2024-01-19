use std::{pin::Pin, future::Future, sync::Arc, io::{Error, ErrorKind}};


use http_body_util::{Full, BodyExt as _};
use hyper::{body::{Bytes, Incoming}, service::Service, Error as HyperError, Request as HttpRequest, Response as HttpResponse, Method};
use serde_json::Error as SerdeError;


use crate::store::{Engine, Value};


#[derive(Clone)]
pub struct EngineService {
    engine: Arc<Engine>,
    cors_allowed_origins: Vec<String>,
}

impl EngineService {
    pub fn new(engine: Engine, cors_allowed_origins: Vec<String>) -> Self {
        Self {
            engine: Arc::new(engine),
            cors_allowed_origins,
        }
    }
}


impl Service<HttpRequest<Incoming>> for EngineService {
    type Response = HttpResponse<Full<Bytes>>;

    type Error = HyperError;

    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn call(&self, req: HttpRequest<Incoming>) -> Self::Future {
        
        let cors_allowed_origins = self.cors_allowed_origins.clone();
        let engine = self.engine.clone();
        
        Box::pin(async move {
            
            let method = req.method().clone();
            let path = req.uri().path().to_string();
            let bytes = http_request_to_bytes(req).await;

            return match method {
                Method::OPTIONS => {
                    Ok(cors_preflight_response(cors_allowed_origins))
                },
                Method::PUT => {
                    
                    let key = put_pathing(path);
                    let value = bytes_to_deserialized_value(bytes);
                    
                    if let Err(e) = key {
                        return Ok(text_to_http_response(e.to_string(), 400, cors_allowed_origins));
                    }

                    if let Err(e) = value {
                        return Ok(text_to_http_response(e.to_string(), 400, cors_allowed_origins));
                    }

                    let key = key.unwrap();
                    let value = value.unwrap();

                    let res = engine.put(key, value).await;

                    if let Err(e) = res {
                        return Ok(text_to_http_response(e.to_string(), 500, cors_allowed_origins));
                    }

                    let body;

                    if let Some(value) = res.unwrap() {
                        body = serialized_value_to_bytes(value);
                    } else {
                        body = "".as_bytes().to_vec();
                    }
                    
                    Ok(bytes_to_http_response(body, cors_allowed_origins))
                },
                Method::GET => {
                    
                    let pathing = get_pathing(path);

                    if let Err(e) = pathing {
                        return Ok(text_to_http_response(e.to_string(), 400, cors_allowed_origins));
                    }

                    let pathing = pathing.unwrap();

                    match pathing {
                        GetPathing::Get(key) => {
                            let res = engine.get(key).await;

                            if let Err(e) = res {
                                return Ok(text_to_http_response(e.to_string(), 500, cors_allowed_origins));
                            }

                            let body;

                            if let Some(value) = res.unwrap() {
                                body = serialized_value_to_bytes(value);
                            } else {
                                body = "".as_bytes().to_vec();
                            }
                            
                            Ok(bytes_to_http_response(body, cors_allowed_origins))
                        },
                        GetPathing::List => {
                            let res = engine.list().await;

                            if let Err(e) = res {
                                return Ok(text_to_http_response(e.to_string(), 500, cors_allowed_origins));
                            }

                            let res = res.unwrap();

                            let body = serialized_array_to_bytes(res);
                            
                            Ok(bytes_to_http_response(body, cors_allowed_origins))
                        }
                    }
                    
                },

                Method::DELETE => {
                    
                    let key = del_pathing(path);
                    
                    if let Err(e) = key {
                        return Ok(text_to_http_response(e.to_string(), 400, cors_allowed_origins));
                    }

                    let key = key.unwrap();

                    let res = engine.del(key).await;

                    if let Err(e) = res {
                        return Ok(text_to_http_response(e.to_string(), 500, cors_allowed_origins));
                    }

                    let body;

                    if let Some(value) = res.unwrap() {
                        body = serialized_value_to_bytes(value);
                    } else {
                        body = "".as_bytes().to_vec();
                    }
                    
                    Ok(bytes_to_http_response(body, cors_allowed_origins))
                    
                },
                _ => {
                    Ok(text_to_http_response("Method not allowed".to_string(), 405, cors_allowed_origins))
                }
            }
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

fn bytes_to_http_response(bytes: Vec<u8>, cors_allowed_origins: Vec<String>) -> HttpResponse<Full<Bytes>> {
    HttpResponse::builder().header("Access-Control-Allow-Origin", cors_allowed_origins.join(",")).header("Content-Type", "application/json").status(200).body(bytes.into()).unwrap()
}

fn text_to_http_response(text: String, exit: u16, cors_allowed_origins: Vec<String>) -> HttpResponse<Full<Bytes>> {
    HttpResponse::builder().header("Access-Control-Allow-Origin", cors_allowed_origins.join(",")).status(exit).body(text.into()).unwrap()
}

fn put_pathing(path: String) -> Result<String, Error> {
    let segments = path.split("/");
    let slice_all = segments.clone().collect::<Vec<&str>>();
    if slice_all.len() != 3 {
        return Err(
            Error::new(
                ErrorKind::InvalidInput,
                format!("Invalid path: {}", path),
            ),
        );
    }
    if slice_all.get(1).unwrap() != &"put" {
        return Err(
            Error::new(
                ErrorKind::InvalidInput,
                format!("Invalid path: {}", path),
            ),
        );
    }
    Ok(slice_all.get(2).unwrap().to_string())
}

enum GetPathing {
    Get(String),
    List,
}

fn get_pathing(path: String) -> Result<GetPathing, Error> {
    let segments = path.split("/");
    let slice_all = segments.clone().collect::<Vec<&str>>();
    let operator = slice_all.get(1);
    if operator.is_none() {
        return Err(
            Error::new(
                ErrorKind::InvalidInput,
                format!("Invalid path: {}", path),
            ),
        );
    }
    let operator = operator.unwrap();
    match operator {
        &"get" => {
            if slice_all.len() != 3 {
                return Err(
                    Error::new(
                        ErrorKind::InvalidInput,
                        format!("Invalid path: {}", path),
                    ),
                );
            }
            Ok(GetPathing::Get(slice_all.get(2).unwrap().to_string()))
        },
        &"list" => {
            Ok(GetPathing::List)
        },
        _ => {
            Err(
                Error::new(
                    ErrorKind::InvalidInput,
                    format!("Invalid path: {}", path),
                ),
            )
        }
    }
}

fn del_pathing(path: String) -> Result<String, Error> {
    let segments = path.split("/");
    let slice_all = segments.clone().collect::<Vec<&str>>();
    if slice_all.len() != 3 {
        return Err(
            Error::new(
                ErrorKind::InvalidInput,
                format!("Invalid path: {}", path),
            ),
        );
    }
    if slice_all.get(1).unwrap() != &"del" {
        return Err(
            Error::new(
                ErrorKind::InvalidInput,
                format!("Invalid path: {}", path),
            ),
        );
    }
    Ok(slice_all[2].to_string())
}

fn bytes_to_deserialized_value(bytes: Vec<u8>) -> Result<Value, SerdeError> {
    serde_json::from_slice(&bytes)
}

fn serialized_value_to_bytes(res: Value) -> Vec<u8> {
    let bytes = serde_json::to_vec(&res);
    bytes.expect("Failed to serialize respond")
}

fn serialized_array_to_bytes(res: Vec<String>) -> Vec<u8> {
    let bytes = serde_json::to_vec(&res);
    bytes.expect("Failed to serialize respond")
}

fn cors_preflight_response(cors_allowed_origins: Vec<String>) -> HttpResponse<Full<Bytes>> {
    HttpResponse::builder()
        .status(200)
        .header("Access-Control-Allow-Origin", cors_allowed_origins.join(","))
        .header("Access-Control-Allow-Methods", "DELETE, GET, HEAD, OPTIONS, PATCH, POST, PUT")
        .header("Access-Control-Allow-Headers", "Content-Type")
        .body("".into())
        .unwrap()
}