use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub enum Value {
    Text(String),
}