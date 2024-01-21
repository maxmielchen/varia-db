use std::io::{Error, ErrorKind};

use http_body_util::{BodyExt as _, Full};
use hyper::{Response, Request, body::{Incoming, Bytes}};

pub async fn http_request_to_bytes(req: Request<Incoming>) -> Vec<u8> {
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

pub fn bytes_to_http_response(bytes: Vec<u8>, exit: u16, cors_allowed_origins: Vec<String>) -> Response<Full<Bytes>> {
    Response::builder().header("Access-Control-Allow-Origin", cors_allowed_origins.join(",")).header("Content-Type", "application/json").status(exit).body(bytes.into()).unwrap()
}

pub fn text_to_http_response(text: String, exit: u16, cors_allowed_origins: Vec<String>) -> Response<Full<Bytes>> {
    Response::builder().header("Access-Control-Allow-Origin", cors_allowed_origins.join(",")).status(exit).body(text.into()).unwrap()
}

pub fn http_request_validate_cors(req: Request<Incoming>, cors_allowed_origins: Vec<String>) -> Result<Request<Incoming>, Error> {
    if cors_allowed_origins.contains(&"*".to_string()) {
        return Ok(req);
    }

    let origin = req.headers().get("Origin");

    if let None = origin {
        return Ok(req);
    }

    let origin = origin.unwrap().to_str();

    if let Err(_) = origin {
        return Err(
            Error::new(
                ErrorKind::InvalidInput,
                "Invalid origin",
            )
        );
    }

    let origin = origin.unwrap();

    if !cors_allowed_origins.contains(&origin.to_string()) {
        return Err(
            Error::new(
                ErrorKind::InvalidInput,
                "Invalid origin",
            )
        );
    }

    Ok(req)
}

pub fn cors_preflight_http_response(cors_allowed_origins: Vec<String>) -> Response<Full<Bytes>> {
    Response::builder()
        .status(200)
        .header("Access-Control-Allow-Origin", cors_allowed_origins.join(","))
        .header("Access-Control-Allow-Methods", "DELETE, GET, HEAD, OPTIONS, PATCH, POST, PUT")
        .header("Access-Control-Allow-Headers", "Content-Type")
        .body("".into())
        .unwrap()
}