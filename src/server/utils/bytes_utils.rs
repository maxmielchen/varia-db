use serde_json::Error;

use crate::{store::Value, server::protocol::Respond};

pub fn bytes_to_deserialized_value(bytes: Vec<u8>) -> Result<Value, Error> {
    serde_json::from_slice(&bytes)
}

pub fn serialized_respond_to_bytes(res: Respond) -> Vec<u8> {
    let bytes = serde_json::to_vec(&res);
    bytes.expect("Failed to serialize respond")
}