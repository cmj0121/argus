// Copyright 2021 cmj <cmj@cmj.tw>. All right reserved.
//! The LSM-tree (log-structured merge-tree) implementation.
use std::collections::HashMap;

struct Value {
    pub(super) raw: Vec<u8>,
    pub(super) deleted: bool,
}

impl Value {
    fn from(value: &Vec<u8>) -> Self {
        Self {
            raw: value.to_vec(),
            deleted: false,
        }
    }
}

pub struct LSMTree {
    mem: HashMap<Vec<u8>, Value>,
}

impl LSMTree {
    pub fn new() -> Self {
        Self {
            mem: HashMap::new(),
        }
    }

    pub fn open(_: &str) -> Result<Self, String>
    where
        Self: Sized,
    {
        Err("not implemented".to_string())
    }

    /// Set the record to LSM-key by specified key, return the success
    /// or failure
    pub fn set(&mut self, key: &Vec<u8>, value: &Vec<u8>) -> bool {
        self.mem.insert(key.to_vec(), Value::from(value));
        true
    }

    /// Get the record form the LSM-tree by specified key, return
    /// value or None if not exist
    pub fn get(&self, key: &Vec<u8>) -> Option<&Vec<u8>> {
        match self.mem.get(&*key) {
            Some(v) => match v.deleted {
                true => return None,
                false => Some(&v.raw),
            },
            _ => return None,
        }
    }

    /// Set record as deleted, in-memory set mark-as-deleted and flush
    /// the record to next layer (may removed)
    pub fn del(&mut self, key: &Vec<u8>) -> bool {
        let mut val: Value = Value::from(&vec![]);

        val.deleted = true;
        self.mem.insert(key.to_vec(), val);
        true
    }
}

// vim: set tabstop=4 sw=4 expandtab:
