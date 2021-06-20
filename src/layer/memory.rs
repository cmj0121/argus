//! The in-memory layer that support quick but non-persistence I/O
use crate::layer::{Error, Layer, Value};
use itertools::Itertools;
use std::collections::HashMap;

/// The name of the memory layer
pub const NAME: &str = "mem";

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
    fn name(&self) -> &'static str {
        NAME
    }

    /// Insert the key-value pair into the memory-layer.
    fn set(&mut self, key: &Vec<u8>, value: &Vec<u8>) -> Result<(), Error> {
        self.mem.insert(key.to_vec(), Value::new(value));
        Ok(())
    }

    /// Get the element from the memory-layer with specified key.
    fn get(&self, key: &Vec<u8>) -> Result<Option<Value>, Error> {
        match self.mem.get(key) {
            Some(v) => Ok(Some(v.clone())),
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

    fn erase_all(&mut self) -> Result<(), Error> {
        self.mem.clear();
        Ok(())
    }

    /// Count the element in the HashMap.
    fn count(&self) -> u64 {
        let mut count: u64 = 0;

        for (_, value) in self.mem.iter() {
            if !value.deleted {
                count += 1;
            }
        }

        count
    }

    /// Get the all valid keys by the descending order.
    fn keys(&self) -> Box<dyn Iterator<Item = Vec<u8>>> {
        Box::new(
            self.mem
                .iter()
                .filter(|row| !row.1.deleted)
                .map(|row| row.0)
                .cloned()
                .sorted_by(|x, y| y.cmp(x)),
        )
    }

    /// Get the ordered keys by the descending order, event the record mark-as-deletd.
    fn all_keys(&self) -> Box<dyn Iterator<Item = Vec<u8>>> {
        Box::new(self.mem.keys().cloned().sorted_by(|x, y| y.cmp(x)))
    }
}

// vim: set tabstop=4 sw=4 expandtab:
