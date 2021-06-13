//! The in-memory layer that support quick but non-persistence I/O
use crate::layer::{Error, Layer};
use std::collections::HashMap;

struct Value {
    pub(super) value: Vec<u8>,
    pub(super) deleted: bool,
}

impl Value {
    fn new(value: &Vec<u8>) -> Self {
        Self {
            value: value.to_vec(),
            deleted: false,
        }
    }
}

/// The in-memory layer which store the key-value pair in memory via HashMap
pub struct MemoryLayer {
    mem: HashMap<Vec<u8>, Value>,
}

impl Layer for MemoryLayer {
    /// Create a dummy memory layer.
    fn new() -> Self {
        Self {
            mem: HashMap::new(),
        }
    }

    /// Show the layer name: memory
    fn name(&self) -> String {
        "memory".to_string()
    }

    /// Open the new memory-layer by pass the HashMap. Note the URI will not
    /// used in the memory-layer.
    fn save(&mut self, _: &str, rows: &HashMap<Vec<u8>, Vec<u8>>) -> Result<(), Error> {
        self.mem.clear();

        for (key, value) in rows.iter() {
            self.mem.insert(key.to_vec(), Value::new(value));
        }

        Ok(())
    }

    /// Insert the key-value pair into the memory-layer.
    fn set(&mut self, key: &Vec<u8>, value: &Vec<u8>) -> Result<(), Error> {
        self.mem.insert(key.to_vec(), Value::new(value));
        Ok(())
    }

    /// Get the element from the memory-layer with specified key.
    fn get(&self, key: &Vec<u8>) -> Result<Option<Vec<u8>>, Error> {
        match self.mem.get(key) {
            Some(v) => match v.deleted {
                true => Ok(None),
                false => Ok(Some(v.value.to_vec())),
            },
            _ => Ok(None),
        }
    }

    /// Set mark-as-deleted in the HashMap
    fn del(&mut self, key: &Vec<u8>) -> Result<bool, Error> {
        match self.mem.get_mut(key) {
            Some(mut v) => match v.deleted {
                true => Ok(false),
                false => {
                    v.deleted = true;
                    Ok(true)
                }
            },
            _ => Ok(false),
        }
    }

    /// Count the element in the HashMap.
    fn count(&self) -> usize {
        let mut count: usize = 0;

        for (_, value) in self.mem.iter() {
            if !value.deleted {
                count += 1;
            }
        }

        count
    }
}

// vim: set tabstop=4 sw=4 expandtab:
