use std::io::Error;
use std::io::ErrorKind;
use log::info;
use log::{error, debug};
use moka::sync::Cache;

use super::Disk;
use super::Value;

pub struct Engine {
    secondary: Disk,
    primary: Cache<String, Value>,
}

impl Engine {
    pub fn new(secondary: Disk, primary: Cache<String, Value>) -> Self {
        Self {
            secondary,
            primary,
        }
    }

    fn key_validation(key: &str) -> Result<(), Error> {
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

    pub fn put(&mut self, key: String, value: Value) -> Result<(), Error> {
        info!("PUT {:?} {:?}", key, value);
        Self::key_validation(&key)?;
        let res = self.secondary.put(key.clone(), value.clone());
        if let Err(e) = res {
            error!("Failed to put value in secondary storage: {:?}", e);
            return Err(
                Error::new(
                    ErrorKind::Other,
                    format!("Failed to put value in secondary storage: {:?}", e)
                )
            );
        } else {
            debug!("Value put in secondary storage");

            self.primary.insert(key, value);
            debug!("Value put in primary storage");
        }
        Ok(())
    }

    pub fn get(&self, key: &str) -> Result<Option<Value>, Error> {
        info!("GET {:?}", key);
        Self::key_validation(key)?;

        let res = self.primary.get(key);
        if let Some(value) = res {
            debug!("Value got from primary storage");
            return Ok(Some(value.clone()));
        }

        let res = self.secondary.get(key);
        if let Err(e) = res {
            if e.kind() == ErrorKind::NotFound {
                debug!("Value not found in secondary storage");
                return Ok(None);
            }
            error!("Failed to get value from secondary storage: {:?}", e);
            return Err(
                Error::new(
                    ErrorKind::Other,
                    format!("Failed to get value from secondary storage: {:?}", e)
                )
            );
        } else {
            debug!("Value got from secondary storage")
        }
        Ok(Some(res.unwrap()))
    }

    pub fn del(&self, key: &str) -> Result<(), Error> {
        info!("DEL {:?}", key);
        Self::key_validation(key)?;
        let res = self.secondary.del(key);
        if let Err(e) = res {
            error!("Failed to delete value from secondary storage: {:?}", e);
            return Err(
                Error::new(
                    ErrorKind::Other,
                    format!("Failed to delete value from secondary storage: {:?}", e)
                )
            );
        } else {
            debug!("Value deleted from secondary storage");

            self.primary.remove(key);
            debug!("Value deleted from primary storage");
        }
        Ok(())
    }

    pub fn list(&self) -> Result<Vec<String>, Error> {
        info!("LIST");
        let res = self.secondary.list();
        if let Err(e) = res {
            error!("Failed to list values from secondary storage: {:?}", e);
            return Err(
                Error::new(
                    ErrorKind::Other,
                    format!("Failed to list values from secondary storage: {:?}", e)
                )
            );
        } else {
            debug!("Values listed from secondary storage")
        }
        Ok(res.unwrap())
    }

    pub fn clear(&self) -> Result<(), Error> {
        info!("CLEAR");
        let res = self.secondary.clear();
        if let Err(e) = res {
            error!("Failed to clear secondary storage: {:?}", e);
            return Err(
                Error::new(
                    ErrorKind::Other,
                    format!("Failed to clear secondary storage: {:?}", e)
                )
            );
        } else {
            debug!("Secondary storage cleared");

            self.primary.invalidate_all();
        }
        Ok(())
    }
}