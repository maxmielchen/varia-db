use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum Value {
    Text(String),
    Number(i128),
    Boolean(bool),
    Array(Vec<Value>),
    Map(Vec<(String, Value)>),
}