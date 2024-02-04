use std::io::{ErrorKind, SeekFrom};
use std::io::Error;

use objstr::file::FileObjStr;
use objstr::api::ObjStr as _;

use crate::store::Value;

use super::content::{deserialize, serialize, ContentType, SIGN};

pub fn sign_creation(fos: &mut FileObjStr) {
    let sign = serialize(&SIGN);
    {
        let _ = fos.seek(SeekFrom::Start(0));
        let _ = fos.cut();
    }
    fos.append(sign).unwrap();
}

pub fn sign_validation(fos: &mut FileObjStr) -> Result<(), Error> {
    fos.seek(SeekFrom::Start(0)).unwrap();
    let bytes = fos.read();
    if bytes.is_err() {
        return Err(Error::new(
            ErrorKind::InvalidData,
            "Invalid sign"
        ));
    }
    let bytes = bytes.unwrap();
    let sign = deserialize(&bytes);
    
    match sign {
        ContentType::Sign(name, version) => {
            if let ContentType::Sign(name_r, version_r) = SIGN {
                if name_r == name && version_r == version {
                    return Ok(());
                }
            }
            return Err(Error::new(
                ErrorKind::Unsupported,
                "Unsupported version"
            ));
        }
        _ => {
            return Err(Error::new(
                ErrorKind::InvalidData,
                "Invalid sign"
            ));
        }
    }
}

pub fn start(fos: &mut FileObjStr) {
    let res = fos.seek(SeekFrom::Start(1));
    if let Err(e) = res {
        panic!("Weird file conditions: {}", e);
    }
}

pub fn find(fos: &mut FileObjStr, key: &str) -> Result<(), Error> {
    loop {
        let bytes = fos.read();
        if bytes.is_err() {
            return Err(Error::new(
                ErrorKind::NotFound,
                "Key not found"
            ));
        }
        let bytes = bytes.unwrap();
        let content_type = deserialize(&bytes);
        match content_type {
            ContentType::Key(k) => {
                if k == key {
                    fos.seek(SeekFrom::Current(-1))?;
                    return Ok(());
                }
            }
            _ => {continue;}
        }
    }
}

pub fn read_entry(fos: &mut FileObjStr) -> (String, Value) {
    let key_buf = fos.read().unwrap();
    let value_buf = fos.read().unwrap();

    let key = match deserialize(&key_buf) {
        ContentType::Key(k) => k,
        _ => panic!("Invalid key")
    };

    let value = match deserialize(&value_buf) {
        ContentType::Value(v) => v,
        _ => panic!("Invalid value")
    };

    (key, value)
}

pub fn append_entry(fos: &mut FileObjStr, key: String, value: Value) {
    let key_buf = serialize(&ContentType::Key(key));
    let value_buf = serialize(&ContentType::Value(value));

    fos.append(key_buf).unwrap();
    fos.append(value_buf).unwrap();
}

pub fn remove_entry(fos: &mut FileObjStr) {
    let key_buf = fos.read().unwrap();
    let value_buf = fos.read().unwrap();

    match deserialize(&key_buf) {
        ContentType::Key(_) => {},
        _ => panic!("Invalid key")
    };

    match deserialize(&value_buf) {
        ContentType::Value(_) => {},
        _ => panic!("Invalid value")
    };

    fos.seek(SeekFrom::Current(-2)).unwrap();

    let len = fos.len(1, 2).unwrap();

    fos.overwrite(
        vec![serialize(&ContentType::Gap(len))], 
        2
    ).unwrap();
}

pub fn write_entry(fos: &mut FileObjStr, key: String, value: Value) -> Result<(), Error> {
    let key_buf = serialize(&ContentType::Key(key));
    let value_buf = serialize(&ContentType::Value(value));

    let len = fos.len(2, 1);
    if len.is_ok_and(|len| len == (key_buf.len() + value_buf.len()) as u64) {
        fos.overwrite(vec![key_buf, value_buf], 1)?;
        return Ok(())
    } 

    let len = fos.len(3, 1);
    
    if let Err(e) = len {
        return Err(Error::new(
            ErrorKind::InvalidData,
            "Not enough space"
        ));
    }

    let len = len.unwrap();
    if len > (key_buf.len() + value_buf.len()) as u64 {
        let gab = serialize(&ContentType::Gap(len - (key_buf.len() + value_buf.len()) as u64));
        fos.overwrite(vec![key_buf, value_buf, gab], 1)?;
        return Ok(())
    }
    
    return Err(Error::new(
        ErrorKind::InvalidData,
        "Not enough space"
    ));
}

pub fn keys_left(fos: &mut FileObjStr) -> Vec<String> {
    let mut keys = Vec::new();
    loop {
        let bytes = fos.read();
        if bytes.is_err() {
            break;
        }
        let bytes = bytes.unwrap();
        let content_type = deserialize(&bytes);
        match content_type {
            ContentType::Key(k) => {
                keys.push(k);
            }
            _ => {continue;}
        }
    }
    keys
}
