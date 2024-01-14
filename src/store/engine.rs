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
        
        if let Some(current_value) = self.primary.get(&key).await {

            self.secondary.lock().unwrap().put(key.clone(), value.clone())?;

            let primary = self.primary.clone();
            let (key_clone, value_clone) = (key.clone(), value.clone());
            tokio::spawn(async move {
                primary.insert(key_clone, Some(value_clone)).await;
            });

            return Ok(current_value);

        } else if let Some(current_value) = self.secondary.lock().unwrap().get(key.clone())? {

            self.secondary.lock().unwrap().put(key.clone(), value.clone())?;
            
            let primary = self.primary.clone();
            let (key_clone, value_clone) = (key.clone(), value.clone());
            tokio::spawn(async move {
                primary.insert(key_clone, Some(value_clone)).await;
            });

            return Ok(Some(current_value));

        } else {
            self.secondary.lock().unwrap().put(key.clone(), value.clone())?;
            
            let primary = self.primary.clone();
            let (key_clone, value_clone) = (key.clone(), value.clone());
            tokio::spawn(async move {
                primary.insert(key_clone, Some(value_clone)).await;
            });

            return Ok(None);

        }
    }

    pub async fn get(&self, key: String) -> Result<Option<Value>, Error> {
        info!("GET {:?}", key);
        
        Self::key_validation(&key)?;

        if let Some(value) = self.primary.get(&key).await {

            return Ok(value);

        } else if let Some(value) = self.secondary.lock().unwrap().get(key.clone())? {

            let primary = self.primary.clone();
            let (key_clone, value_clone) = (key.clone(), value.clone());
            tokio::spawn(async move {
                primary.insert(key_clone, Some(value_clone)).await;
            });

            return Ok(Some(value));

        } else {

            let primary = self.primary.clone();
            let (key_clone, value_clone) = (key.clone(), None);
            tokio::spawn(async move {
                primary.insert(key_clone, value_clone).await;
            });

            return Ok(None);

        }
    }

    pub async fn del(&self, key: String) -> Result<Option<Value>, Error> {
        info!("DEL {:?}", key);
        Self::key_validation(&key)?;

        
        if let Some(value) = self.primary.get(&key).await {

            self.secondary.lock().unwrap().del(key.clone())?;
            
            let primary = self.primary.clone();
            let (key_clone, value_clone) = (key.clone(), None);
            tokio::spawn(async move {
                primary.insert(key_clone, value_clone).await;
            });

            return Ok(value);

        } else if let Some(value) = self.secondary.lock().unwrap().get(key.clone())? {

            self.secondary.lock().unwrap().del(key.clone())?;
            
            let primary = self.primary.clone();
            let (key_clone, value_clone) = (key.clone(), None);
            tokio::spawn(async move {
                primary.insert(key_clone, value_clone).await;
            });

            return Ok(Some(value));

        } else {

            let primary = self.primary.clone();
            let (key_clone, value_clone) = (key.clone(), None);
            tokio::spawn(async move {
                primary.insert(key_clone, value_clone).await;
            });

            return Ok(None);

        }
        
    }

    pub async fn list(&self) -> Result<Vec<String>, Error> {
        info!("LIST");

        return Ok(self.secondary.lock().unwrap().list()?);
    }

    pub async fn clear(&self) -> Result<(), Error> {
        info!("CLEAR");

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