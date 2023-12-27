use std::{path::{PathBuf, Path}, fs::{create_dir_all, File, remove_file}, io::{Error, Write, BufReader, ErrorKind}};

use log::error;

use super::Value;

pub struct Secondary {
    path: PathBuf,
}

impl Secondary {
    pub fn new(path: &Path) -> Result<Self, Error> {
        if path.exists() {
            if !path.is_dir() {
                error!("Path {:?} is not a directory", path);
                return Err(Error::new(
                    ErrorKind::InvalidInput,
                    format!("Path {:?} is not a directory", path)
                ));
            }
        } else {
            create_dir_all(path)?;
        }
        Ok(Self {
            path: path.to_path_buf(),
        })
    }

    pub fn put(&mut self, key: String, value: Value) -> Result<(), Error> {
        let content = serde_json::to_string(&value)?;
        let mut file = File::create(self.path.join(format!("{}.json", key)))?;
        file.write_all(content.as_bytes())?;
        Ok(())
    }

    pub fn get(&self, key: &str) -> Result<Value, Error> {
        let file = File::open(self.path.join(format!("{}.json", key)))?;
        let reader = BufReader::new(file);
        let value = serde_json::from_reader(reader)?;
        Ok(value)
    }

    pub fn del(&self, key: &str) -> Result<(), Error> {
        remove_file(self.path.join(format!("{}.json", key)))?;
        Ok(())
    }

    pub fn list(&self) -> Result<Vec<String>, Error> {
        let mut keys = Vec::new();
        for entry in self.path.read_dir()? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() && path.extension().unwrap() == "json" {
                let key = path.file_stem().unwrap().to_str().unwrap().to_string();
                keys.push(key);
            }
        }
        Ok(keys)
    }

    pub fn clear(&self) -> Result<(), Error> {
        for entry in self.path.read_dir()? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() && path.extension().unwrap() == "json" {
                remove_file(path)?;
            }
        }
        Ok(())
    }

    pub fn len(&self) -> Result<usize, Error> {
        let mut count = 0;
        for entry in self.path.read_dir()? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() && path.extension().unwrap() == "json" {
                count += 1;
            }
        }
        Ok(count)
    }
}