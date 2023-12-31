use super::Value;

pub fn weight(key: &String, value: &Value) -> u32 {
    let mut weight = 0;
    weight += std::mem::size_of_val(&key);
    weight += std::mem::size_of_val(&value);
    weight as u32
}