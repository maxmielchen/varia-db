use std::io::Error;
use std::io::ErrorKind;
use std::sync::Arc;
use std::sync::Mutex;
use log::info;
use log::trace;
use log::{error, debug};
use moka::future::Cache;

use super::Disk;
use super::Value;

pub struct Engine {
    secondary: Arc<Mutex<Disk>>,
    primary: Cache<String, Value>,
}

impl Engine {
    pub fn new(secondary: Disk, primary: Cache<String, Value>) -> Self {
        let secondary = Arc::new(Mutex::new(secondary));
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

    pub async fn put(&self, key: String, value: Value) -> Result<(), Error> {
        info!("PUT {:?} {:?}", key, value);

        Self::key_validation(&key)?;

        let secondary_lock = self.secondary.lock();
        if let Err(e) = secondary_lock {
            error!("Failed to lock secondary storage: {:?}", e);
            return Err(
                Error::new(
                    ErrorKind::Other,
                    format!("Failed to lock secondary storage: {:?}", e)
                )
            );
        }
        let secondary_locked = secondary_lock.unwrap();

        let res = secondary_locked.put(key.clone(), value.clone());
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

        let primary_cloned = self.primary.clone();
        tokio::task::spawn(async move {
            primary_cloned.insert(key, value).await;
            debug!("Value put in primary storage");
            trace!("Primary storage: {:?}", primary_cloned);
        });

        Ok(())
    }

    pub async fn get(&self, key: &str) -> Result<Option<Value>, Error> {
        info!("GET {:?}", key);
        
        Self::key_validation(key)?;

        let res = self.primary.get(key).await;
        if let Some(value) = res {
            debug!("Value got from primary storage");
            return Ok(Some(value.clone()));
        }

        let secondary_lock = self.secondary.lock();
        if let Err(e) = secondary_lock {
            error!("Failed to lock secondary storage: {:?}", e);
            return Err(
                Error::new(
                    ErrorKind::Other,
                    format!("Failed to lock secondary storage: {:?}", e)
                )
            );
        }
        let secondary_locked = secondary_lock.unwrap();

        let res = secondary_locked.get(key);
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
        } 
        debug!("Value got from secondary storage");

        let res = res.unwrap();

        let primary_cloned = self.primary.clone();
        let key_cloned = key.to_string().clone();
        let value_cloned = res.clone();
        tokio::task::spawn(async move {
            primary_cloned.insert(key_cloned, value_cloned).await;
            debug!("Value put in primary storage");
            trace!("Primary storage: {:?}", primary_cloned);
        });

        Ok(Some(res))
    }

    pub async fn del(&self, key: &str) -> Result<(), Error> {
        info!("DEL {:?}", key);
        Self::key_validation(key)?;

        let secondary_lock = self.secondary.lock();
        if let Err(e) = secondary_lock {
            error!("Failed to lock secondary storage: {:?}", e);
            return Err(
                Error::new(
                    ErrorKind::Other,
                    format!("Failed to lock secondary storage: {:?}", e)
                )
            );
        }
        let secondary_locked = secondary_lock.unwrap();

        let res = secondary_locked.del(key);
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

        let primary_cloned = self.primary.clone();
        let key_cloned = key.to_string().clone();
        tokio::task::spawn(async move {
            primary_cloned.invalidate(&key_cloned).await;
            debug!("Value deleted from primary storage");
            trace!("Primary storage: {:?}", primary_cloned);
        });

        Ok(())
    }

    pub async fn list(&self) -> Result<Vec<String>, Error> {
        info!("LIST");

        let secondary_lock = self.secondary.lock();
        if let Err(e) = secondary_lock {
            error!("Failed to lock secondary storage: {:?}", e);
            return Err(
                Error::new(
                    ErrorKind::Other,
                    format!("Failed to lock secondary storage: {:?}", e)
                )
            );
        }
        let secondary_locked = secondary_lock.unwrap();

        let res = secondary_locked.list();
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

    pub async fn clear(&self) -> Result<(), Error> {
        info!("CLEAR");

        let secondary_lock = self.secondary.lock();
        if let Err(e) = secondary_lock {
            error!("Failed to lock secondary storage: {:?}", e);
            return Err(
                Error::new(
                    ErrorKind::Other,
                    format!("Failed to lock secondary storage: {:?}", e)
                )
            );
        }
        let secondary_locked = secondary_lock.unwrap();

        let res = secondary_locked.clear();
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

        self.primary.invalidate_all();
        debug!("Primary storage cleared");
        trace!("Primary storage: {:?}", self.primary);
        
        Ok(())
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