use serde::Serialize;

use crate::store::Value;

#[derive(Serialize)]
pub enum Respond {
    Ok,
    Err(String),
    Value(Option<Value>),
    Keys(Vec<String>),
}