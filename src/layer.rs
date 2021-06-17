// Copyright 2021 cmj <cmj@cmj.tw>. All right reserved.
//! The LSM-tree (log-structured merge-tree) implementation.
use std::collections::HashMap;

#[derive(Debug)]
pub struct Error {
    msg: String,
}

impl Error {
    pub fn new(msg: String) -> Self {
        Self { msg: msg }
    }
}

#[derive(Debug, Clone)]
pub struct Value {
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

    pub fn is_deleted(&self) -> bool {
        self.deleted
    }
}

/// The layer define several method used in the LSM-tree which can used in
/// difference scenraio, like in-memory or persistence storage.
pub trait Layer {
    /// Create new layer with its own initialization.
    fn new() -> Self
    where
        Self: Sized;

    /// Show the layer's name
    fn name(&self) -> String;

    /// Create new layer with specified URI and pass the known data.
    fn save(&mut self, _uri: &str, _rows: &HashMap<Vec<u8>, Vec<u8>>) -> Result<(), Error> {
        Err(Error::new(format!("[{}] not support save", self.name())))
    }

    /// Open the exist layer via the specified URI
    fn open(&self, _uri: &str) -> Result<Self, Error>
    where
        Self: Sized,
    {
        Err(Error::new(format!("[{}] not support open", self.name())))
    }

    /// Set the key to the LSM-tree layer.
    fn set(&mut self, key: &Vec<u8>, value: &Vec<u8>) -> Result<(), Error>;

    /// Get the value from the LSM-tree layer with specified key.
    fn get(&self, key: &Vec<u8>) -> Result<Option<Value>, Error>;

    /// Delete the record from the LSM-tree with specified key.
    fn del(&mut self, key: &Vec<u8>) -> Result<bool, Error>;

    /// Count the valid element in the layer.
    fn count(&self) -> usize;
}

pub mod memory;

/// Open the layer by the specified name.
///
/// This function will return the Box<dyn Layer> if name pre-define in layer, or
/// return None.
pub fn new(name: &str) -> Option<Box<dyn Layer>> {
    match name {
        "mem" | "memory" => return Some(Box::new(memory::MemoryLayer::new())),
        _ => None,
    }
}

// vim: set tabstop=4 sw=4 expandtab:
