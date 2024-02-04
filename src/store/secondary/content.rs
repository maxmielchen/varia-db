use crate::store::Value;

const SIGN_BYTE: u8 = 0x01;
const GAP_BYTE: u8 = 0x02;
const KEY_BYTE: u8 = 0x03;
const VALUE_BYTE: u8 = 0x04;

#[derive(Debug, Clone, PartialEq)]
pub enum ContentType {
    Sign([u8; 5], f32),
    Gap(u64),
    Key(String),
    Value(Value),
}

const SIGN_NAME: [u8; 5] = *b"varia";
const SIGN_VERSION: f32 = 0.2; // Change after each release
pub const SIGN: ContentType = ContentType::Sign(SIGN_NAME, SIGN_VERSION);

pub fn serialize(content_type: &ContentType) -> Vec<u8> {
    match content_type {
        ContentType::Sign(name, version) => {
            let mut bytes = vec![SIGN_BYTE];
            bytes.extend_from_slice(name);
            bytes.extend_from_slice(&version.to_ne_bytes());
            bytes
        }
        ContentType::Gap(gap) => {
            let mut bytes = vec![GAP_BYTE];
            for _ in 0..*gap-1 {
                bytes.push(0);
            }
            bytes
        }
        ContentType::Key(key) => {
            let mut bytes = vec![KEY_BYTE];
            bytes.extend_from_slice(key.as_bytes());
            bytes
        }
        ContentType::Value(value) => {
            let mut bytes = vec![VALUE_BYTE];
            bytes.extend_from_slice(
                &postcard::to_allocvec(value).unwrap()
            );
            bytes
        }
    }
}

pub fn deserialize(bytes: &[u8]) -> ContentType {
    match bytes[0] {
        SIGN_BYTE => {
            let name: [u8; 5] = bytes[1..6].try_into().unwrap();
            let version = f32::from_ne_bytes(bytes[6..10].try_into().unwrap());
            ContentType::Sign(name, version)
        }
        GAP_BYTE => {
            let gap = bytes.len() as u64 - 1;
            ContentType::Gap(gap)
        }
        KEY_BYTE => {
            let key = String::from_utf8(bytes[1..].to_vec()).unwrap();
            ContentType::Key(key)
        }
        VALUE_BYTE => {
            let value = postcard::from_bytes(&bytes[1..]).unwrap();
            ContentType::Value(value)
        }
        _ => panic!("Unknown content type"),
    }
}