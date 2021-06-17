// Copyright 2021 cmj <cmj@cmj.tw>. All right reserved.
//! The LSM-tree (log-structured merge-tree) implementation.
use crate::layer::{new, Error, Layer};

/// The implementation of LSM-tree which may support multi-layers.
pub struct LSMTree {
    layers: Box<dyn Layer>,
}

impl LSMTree {
    /// Create new LSM-tree with default layers.
    pub fn new() -> Self {
        Self {
            layers: new("mem").unwrap(),
        }
    }

    /// Get the data from the under-layer with specified key.
    pub fn get(&self, key: &Vec<u8>) -> Result<Option<Vec<u8>>, Error> {
        match self.layers.get(key)? {
            Some(v) if !v.deleted => Ok(Some(v.value.to_vec())),
            _ => Ok(None),
        }
    }

    /// Set the data to the under-layer with specified key.
    pub fn set(&mut self, key: &Vec<u8>, value: &Vec<u8>) -> Result<(), Error> {
        self.layers.set(key, value)
    }

    /// Delete the data from the under-layer with specified key.
    pub fn del(&mut self, key: &Vec<u8>) -> Result<bool, Error> {
        self.layers.del(key)
    }

    /// Count the valid element in the under-layer.
    pub fn count(&self) -> usize {
        self.layers.count()
    }
}

// vim: set ft=rust tabstop=4 sw=4 expandtab:
