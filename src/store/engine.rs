use std::io::Error;
use std::io::ErrorKind;
use log::{error, debug};

use super::Primary;
use super::Secondary;
use super::Value;

pub struct Engine {
    secondary: Secondary,
    #[allow(dead_code)]
    primary: Primary,
}

impl Engine {
    pub fn new(secondary: Secondary, primary: Primary) -> Self {
        Self {
            secondary,
            primary,
        }
    }

    pub fn put(&mut self, key: String, value: Value) -> Result<(), Error> {
        let res = self.secondary.put(key, value);
        if let Err(e) = res {
            error!("Failed to put value in secondary storage: {:?}", e);
            return Err(
                Error::new(
                    ErrorKind::Other,
                    format!("Failed to put value in secondary storage: {:?}", e)
                )
            );
        }
        debug!("Value put in secondary storage");
        Ok(())
    }

    pub fn get(&self, key: &str) -> Result<Value, Error> {
        let res = self.secondary.get(key);
        if let Err(e) = res {
            error!("Failed to get value from secondary storage: {:?}", e);
            return Err(
                Error::new(
                    ErrorKind::Other,
                    format!("Failed to get value from secondary storage: {:?}", e)
                )
            );
        }
        debug!("Value got from secondary storage");
        Ok(res.unwrap())
    }

    pub fn del(&self, key: &str) -> Result<(), Error> {
        let res = self.secondary.del(key);
        if let Err(e) = res {
            error!("Failed to delete value from secondary storage: {:?}", e);
            return Err(
                Error::new(
                    ErrorKind::Other,
                    format!("Failed to delete value from secondary storage: {:?}", e)
                )
            );
        }
        debug!("Value deleted from secondary storage");
        Ok(())
    }

    pub fn list(&self) -> Result<Vec<String>, Error> {
        let res = self.secondary.list();
        if let Err(e) = res {
            error!("Failed to list values from secondary storage: {:?}", e);
            return Err(
                Error::new(
                    ErrorKind::Other,
                    format!("Failed to list values from secondary storage: {:?}", e)
                )
            );
        }
        debug!("Values listed from secondary storage");
        Ok(res.unwrap())
    }

    pub fn clear(&self) -> Result<(), Error> {
        let res = self.secondary.clear();
        if let Err(e) = res {
            error!("Failed to clear secondary storage: {:?}", e);
            return Err(
                Error::new(
                    ErrorKind::Other,
                    format!("Failed to clear secondary storage: {:?}", e)
                )
            );
        }
        debug!("Secondary storage cleared");
        Ok(())
    }
}