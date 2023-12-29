use serde::Deserialize;

use crate::store::Value;

#[derive(Deserialize)]
pub enum Request {
    Put(String, Value),
    Get(String),
    Del(String),
    List,
}