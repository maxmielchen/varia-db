use serde::Serialize;

use crate::store::Value;

#[derive(Serialize, Clone)]
pub enum Respond {
    Ok,
    Value(Option<Value>),
    Keys(Vec<String>),
}