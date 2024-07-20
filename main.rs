use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufWriter, Read, Write};
use thiserror::Error;

#[derive(Error, Debug)]
enum StoreError {
    #[error("Key not found: {0}")]
    KeyNotFound(String),
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

struct KeyValueStore {
    data: HashMap<String, String>,
    file_path: String,
}

impl KeyValueStore {
    fn new(file_path: String) -> Result<Self, StoreError> {
        let mut store = KeyValueStore {
            data: HashMap::new(),
            file_path,
        };
        store.load()?;
        Ok(store)
    }

    fn set(&mut self, key: String, value: String) -> Result<(), StoreError> {
        self.data.insert(key, value);
        self.save()
    }

    fn get(&self, key: &str) -> Result<&String, StoreError> {
        self.data.get(key).ok_or_else(|| StoreError::KeyNotFound(key.to_string()))
    }

    fn delete(&mut self, key: &str) -> Result<String, StoreError> {
        let value = self.data.remove(key).ok_or_else(|| StoreError::KeyNotFound(key.to_string()))?;
        self.save()?;
        Ok(value)
    }

    fn load(&mut self) -> Result<(), StoreError> {
        let file = File::open(&self.file_path)?;
        let mut reader = BufReader::new(file);
        let mut contents = String::new();
        reader.read_to_string(&mut contents)?;

        for line in contents.lines() {
            let parts: Vec<&str> = line.splitn(2, ':').collect();
            if parts.len() == 2 {
                self.data.insert(parts[0].to_string(), parts[1].to_string());
            }
        }
        Ok(())
    }

    fn save(&self) -> Result<(), StoreError> {
        let file = OpenOptions::new().write(true).create(true).truncate(true).open(&self.file_path)?;
        let mut writer = BufWriter::new(file);

        for (key, value) in &self.data {
            writeln!(writer, "{}:{}", key, value)?;
        }

        Ok(())
    }
}

fn main() -> Result<(), StoreError> {
    let mut store = KeyValueStore::new("store.db".to_string())?;

    store.set("hello".to_string(), "world".to_string())?;
    println!("Value for 'hello': {}", store.get("hello")?);
    
    store.delete("hello")?;
    println!("'hello' key deleted");
    
    match store.get("hello") {
        Ok(value) => println!("Value for 'hello': {}", value),
        Err(e) => println!("Error: {}", e),
    }   
    Ok(())
}