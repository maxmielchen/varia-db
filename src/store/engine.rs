use std::io::Error;
use std::io::ErrorKind;
use std::sync::Arc;
use std::sync::Mutex;
use log::info;
use moka::future::Cache;

use super::Disk;
use super::Value;

pub struct Engine {
    secondary: Arc<Mutex<Disk>>,
    primary: Cache<String, Option<Value>>,
}

impl Engine {
    pub fn new(secondary: Disk, primary: Cache<String, Option<Value>>) -> Self {
        let secondary = Arc::new(Mutex::new(secondary));
        Self {
            secondary,
            primary,
        }
    }

    fn key_validation(key: &String) -> Result<(), Error> {
        for c in key.chars() {
            if !c.is_alphanumeric() {
                return Err(
                    Error::new(
                        ErrorKind::InvalidInput,
                        format!("Invalid character '{}' in key", c)
                    )
                );
            }
        }
        Ok(())
    }

    pub async fn put(&self, key: String, value: Value) -> Result<Option<Value>, Error> {
        info!("PUT {:?} {:?}", key, value);

        Self::key_validation(&key)?;

        unimplemented!()
    }

    pub async fn get(&self, key: String) -> Result<Option<Value>, Error> {
        info!("GET {:?}", key);
        
        Self::key_validation(&key)?;

        unimplemented!()
    }

    pub async fn del(&self, key: String) -> Result<Option<Value>, Error> {
        info!("DEL {:?}", key);
        Self::key_validation(&key)?;

        unimplemented!()
    }

    pub async fn list(&self) -> Result<Vec<String>, Error> {
        info!("LIST");

        unimplemented!()
    }

    pub async fn clear(&self) -> Result<(), Error> {
        info!("CLEAR");

        unimplemented!()
    }
}

impl Clone for Engine {
    fn clone(&self) -> Self {
        Self {
            secondary: self.secondary.clone(),
            primary: self.primary.clone(),
        }
    }
}