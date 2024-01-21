use std::{pin::Pin, future::Future, sync::Arc};


use http_body_util::Full;
use hyper::{body::{Bytes, Incoming}, service::Service, Error as HyperError, Request as HttpRequest, Response as HttpResponse, Method};

use crate::store::Engine;

use super::{
    GetPathing, put_pathing, get_pathing, del_pathing, 
    Respond,

    http_request_to_bytes, bytes_to_http_response, text_to_http_response, http_request_validate_cors, cors_preflight_http_response,
    bytes_to_deserialized_value, serialized_respond_to_bytes
};


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

            let cors_valid = http_request_validate_cors(req, cors_allowed_origins.clone());

            let req = match cors_valid {
                Ok(req) => req,
                Err(e) => {
                    return Ok(text_to_http_response(e.to_string(), 401, cors_allowed_origins));
                }
            };
            
            let method = req.method().clone();
            let path = req.uri().path().to_string();
            let bytes = http_request_to_bytes(req).await;

            return match method {
                Method::OPTIONS => {
                    Ok(cors_preflight_http_response(cors_allowed_origins))
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

                    let result = engine.put(key, value).await;

                    if let Err(e) = result {
                        return Ok(text_to_http_response(e.to_string(), 500, cors_allowed_origins));
                    }

                    let respond = Respond::Value(result.unwrap());

                    Ok(bytes_to_http_response(serialized_respond_to_bytes(respond), 200, cors_allowed_origins))
                },
                Method::GET => {
                    
                    let pathing = get_pathing(path);

                    if let Err(e) = pathing {
                        return Ok(text_to_http_response(e.to_string(), 400, cors_allowed_origins));
                    }

                    let pathing = pathing.unwrap();

                    match pathing {
                        GetPathing::Get(key) => {
                            let result = engine.get(key).await;

                            if let Err(e) = result {
                                return Ok(text_to_http_response(e.to_string(), 500, cors_allowed_origins));
                            }

                            let respond = Respond::Value(result.unwrap());
                            
                            Ok(bytes_to_http_response(serialized_respond_to_bytes(respond), 200, cors_allowed_origins))
                        },
                        GetPathing::List => {
                            let result = engine.list().await;

                            if let Err(e) = result {
                                return Ok(text_to_http_response(e.to_string(), 500, cors_allowed_origins));
                            }

                            let respond = Respond::Array(result.unwrap());
                            
                            Ok(bytes_to_http_response(serialized_respond_to_bytes(respond), 200, cors_allowed_origins))
                        }
                    }
                },

                Method::DELETE => {
                        let pathing = del_pathing(path);
    
                        if let Err(e) = pathing {
                            return Ok(text_to_http_response(e.to_string(), 400, cors_allowed_origins));
                        }
    
                        let key = pathing.unwrap();

                        let result = engine.del(key).await;

                        if let Err(e) = result {
                            return Ok(text_to_http_response(e.to_string(), 500, cors_allowed_origins));
                        }

                        let respond = Respond::Value(result.unwrap());

                        Ok(bytes_to_http_response(serialized_respond_to_bytes(respond), 200, cors_allowed_origins))
                },
                _ => {
                    Ok(text_to_http_response("Method not allowed".to_string(), 405, cors_allowed_origins))
                }
            }
        })
    }
}
