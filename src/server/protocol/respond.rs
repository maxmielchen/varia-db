use serde::{Serialize, Deserialize};

use crate::store::Value;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum Respond {
    Value(Option<Value>),
    Array(Vec<String>),
}
