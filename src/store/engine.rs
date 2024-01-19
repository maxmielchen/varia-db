use std::io::Error;
use std::io::ErrorKind;
use std::sync::Arc;
use std::sync::Mutex;
use log::debug;
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
        if key.len() < 1 {
            return Err(
                Error::new(
                    ErrorKind::InvalidInput,
                    "Key must be at least 1 character long"
                )
            );
        }
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
        
        if let Some(current_value) = self.primary.get(&key).await {

            debug!("Cache hit for key {:?} with value {:?}", key, current_value);

            debug!("Updating secondary storage");
            debug!("Secondary is poisoned: {:?}", self.secondary.is_poisoned());
            self.secondary.lock().unwrap().put(key.clone(), value.clone())?;

            let primary = self.primary.clone();
            let (key_clone, value_clone) = (key.clone(), value.clone());
            tokio::task::spawn(async move {
                debug!("Updating primary storage");
                primary.insert(key_clone, Some(value_clone)).await;
            });

            debug!("Returning old value");
            return Ok(current_value);

        } 
        
        let mut secondary = self.secondary.lock().unwrap();

        if let Some(current_value) = &secondary.get(key.clone())? {

            debug!("Cache miss for key {:?} with value {:?}", key, current_value);

            debug!("Updating secondary storage");
            debug!("Secondary is poisoned: {:?}", self.secondary.is_poisoned());
            secondary.put(key.clone(), value.clone())?;
            
            let primary = self.primary.clone();
            let (key_clone, value_clone) = (key.clone(), value.clone());
            tokio::task::spawn(async move {
                debug!("Updating primary storage");
                primary.insert(key_clone, Some(value_clone)).await;
            });

            debug!("Returning old value");
            return Ok(Some(current_value.clone()));

        } else {
            debug!("Cache miss for key {:?}", key);

            debug!("Updating secondary storage");
            debug!("Secondary is poisoned: {:?}", self.secondary.is_poisoned());
            secondary.put(key.clone(), value.clone())?;
            
            let primary = self.primary.clone();
            let (key_clone, value_clone) = (key.clone(), value.clone());
            tokio::task::spawn(async move {
                debug!("Updating primary storage");
                primary.insert(key_clone, Some(value_clone)).await;
            });

            debug!("Returning old value");
            return Ok(None);

        }
    }

    pub async fn get(&self, key: String) -> Result<Option<Value>, Error> {
        info!("GET {:?}", key);
        
        Self::key_validation(&key)?;

        if let Some(value) = self.primary.get(&key).await {

            debug!("Cache hit for key {:?} with value {:?}", key, value);

            debug!("Returning value");
            return Ok(value);

        } 

        let mut secondary = self.secondary.lock().unwrap(); 
        
        if let Some(value) = &secondary.get(key.clone())? {

            debug!("Cache miss for key {:?} with value {:?}", key, value);

            let primary = self.primary.clone();
            let (key_clone, value_clone) = (key.clone(), value.clone());
            tokio::task::spawn(async move {
                debug!("Updating primary storage");
                primary.insert(key_clone, Some(value_clone)).await;
            });

            debug!("Returning value");
            return Ok(Some(value.clone()));

        } else {

            debug!("Cache miss for key {:?}", key);

            let primary = self.primary.clone();
            let (key_clone, value_clone) = (key.clone(), None);
            tokio::task::spawn(async move {
                debug!("Updating primary storage");
                primary.insert(key_clone, value_clone).await;
            });

            debug!("Returning value");
            return Ok(None);

        }
    }

    pub async fn del(&self, key: String) -> Result<Option<Value>, Error> {
        info!("DEL {:?}", key);
        Self::key_validation(&key)?;

        
        if let Some(value) = self.primary.get(&key).await {

            debug!("Cache hit for key {:?} with value {:?}", key, value);

            debug!("Updating secondary storage");
            debug!("Secondary is poisoned: {:?}", self.secondary.is_poisoned());
            self.secondary.lock().unwrap().del(key.clone())?;
            
            let primary = self.primary.clone();
            let (key_clone, value_clone) = (key.clone(), None);
            tokio::task::spawn(async move {
                debug!("Updating primary storage");
                primary.insert(key_clone, value_clone).await;
            });

            debug!("Returning old value");
            return Ok(value);

        } 

        let mut secondary = self.secondary.lock().unwrap();
        
        if let Some(value) = &secondary.get(key.clone())? {

            debug!("Cache miss for key {:?} with value {:?}", key, value);

            debug!("Updating secondary storage");
            debug!("Secondary is poisoned: {:?}", self.secondary.is_poisoned());
            secondary.del(key.clone())?;
            
            let primary = self.primary.clone();
            let (key_clone, value_clone) = (key.clone(), None);
            tokio::task::spawn(async move {
                debug!("Updating primary storage");
                primary.insert(key_clone, value_clone).await;
            });

            debug!("Returning old value");
            return Ok(Some(value.clone()));

        } else {

            debug!("Cache miss for key {:?}", key);

            let primary = self.primary.clone();
            let (key_clone, value_clone) = (key.clone(), None);
            tokio::task::spawn(async move {
                debug!("Updating primary storage");
                primary.insert(key_clone, value_clone).await;
            });

            debug!("Returning old value");
            return Ok(None);

        }
        
    }

    pub async fn list(&self) -> Result<Vec<String>, Error> {
        info!("LIST");

        debug!("Updating secondary storage");
        debug!("Secondary is poisoned: {:?}", self.secondary.is_poisoned());
        return Ok(self.secondary.lock().unwrap().list()?);
    }

    pub async fn clear(&self) -> Result<(), Error> {
        info!("CLEAR");

        debug!("Updating secondary storage");
        debug!("Secondary is poisoned: {:?}", self.secondary.is_poisoned());
        return Ok(self.secondary.lock().unwrap().clear()?);
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