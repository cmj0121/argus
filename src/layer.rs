// Copyright 2021 cmj <cmj@cmj.tw>. All right reserved.
//! The LSM-tree (log-structured merge-tree) implementation.
use std::fmt;

#[derive(Debug)]
pub struct Error {
    msg: String,
}

impl Error {
    pub fn new(msg: String) -> Self {
        Self { msg: msg }
    }
}

/// Show the format `{}` for implemente the fmt::Display trait.
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}

#[derive(Debug, Clone)]
pub struct Value {
    pub(super) value: Vec<u8>,
    pub(super) deleted: bool,
}

impl Value {
    /// Create new value by pass the Vec<u8>
    fn new(value: &Vec<u8>) -> Self {
        Self {
            value: value.to_vec(),
            deleted: false,
        }
    }

    /// Expose the deletd status for the value
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
    fn name(&self) -> &'static str;

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

    /// Erase all data in the current layer.
    fn erase(&mut self) -> Result<(), Error> {
        Err(Error::new(format!(
            "[{}] not implemented `erase_all`",
            self.name()
        )))
    }

    /// Flush data to another layer.
    fn flush(&self, _: &Box<dyn Layer>) -> Result<(), Error> {
        Err(Error::new(format!(
            "[{}] not implemented `flush`",
            self.name()
        )))
    }

    /// Count the valid element in the layer.
    fn count(&self) -> u64;

    /// Count the total element in the layer, includes mark-as-deleted
    fn capacity(&self) -> u64;

    /// Get the all valid keys by the descending order.
    fn keys(&self, include_deleted: bool) -> Box<dyn Iterator<Item = Vec<u8>>>;

    /// Get the order key-value pair by the descending order, event the record mark-as-deleted.
    fn pairs(&self) -> Box<dyn Iterator<Item = (&Vec<u8>, &Value)> + '_>;
}

pub mod memory;

/// Open the layer by the specified name.
///
/// This function will return the Box<dyn Layer> if name pre-define in layer, or
/// return None.
pub fn new(name: &str) -> Result<Box<dyn Layer>, Error> {
    match name {
        memory::NAME => return Ok(Box::new(memory::MemoryLayer::new())),
        _ => Err(Error::new(format!("cannot create layer: {}", name))),
    }
}

// vim: set tabstop=4 sw=4 expandtab:
