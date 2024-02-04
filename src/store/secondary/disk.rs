use std::{io::{Error, ErrorKind}, path::Path};
use objstr::{api::ObjStr, file::FileObjStr};

use crate::store::Value;

use super::helpers::{append_entry, find, keys_left, read_entry, remove_entry, sign_creation, sign_validation, start};

pub struct Disk {
    pub fos: FileObjStr
}

impl Disk {

    pub fn new(path: &Path) -> Result<Self, Error> {
        let mut fos = FileObjStr::new(path)?;
        let sign = sign_validation(&mut fos);
        let sign_is_err = sign.is_err();
        let unsupported_version = sign_is_err && (sign.unwrap_err()).kind() == ErrorKind::Unsupported;

        if unsupported_version {
            panic!("Unsupported version");
        }  

        if sign_is_err {
            sign_creation(&mut fos);
        }
        
        Ok(Self {
            fos
        })
    }

    pub fn put(&mut self, key: String, value: Value) -> Result<(), Error> {
        self.del(key.clone())?;

        start(&mut self.fos);
        append_entry(&mut self.fos, key, value);
        Ok(())
    }

    pub fn get(&mut self, key: String) -> Result<Option<Value>, Error> {
        start(&mut self.fos);
        let search_res = find(&mut self.fos, &key);
        if search_res.is_err() {
            return Ok(None);
        }
        let (_, value) = read_entry(&mut self.fos);
        Ok(Some(value))
    }

    pub fn del(&mut self, key: String) -> Result<(), Error> {
        start(&mut self.fos);
        let search_res = find(&mut self.fos, &key);
        if search_res.is_err() {
            return Ok(());
        }  
        remove_entry(&mut self.fos);
        Ok(())
    }

    pub fn list(&mut self) -> Result<Vec<String>, Error> {
        start(&mut self.fos);
        Ok(keys_left(&mut self.fos))
    }

    pub fn clear(&mut self) -> Result<(), Error> {
        start(&mut self.fos);
        let _ = self.fos.cut();
        Ok(())
    }

    pub fn len(&mut self) -> Result<usize, Error> {
        start(&mut self.fos);
        Ok(keys_left(&mut self.fos).len())
    }

    pub fn is_empty(&mut self) -> Result<bool, Error> {
        start(&mut self.fos);
        Ok(keys_left(&mut self.fos).is_empty())
    }

    pub fn defrag(&mut self) -> Result<(), Error> {
        todo!("Defrag")
    }
}