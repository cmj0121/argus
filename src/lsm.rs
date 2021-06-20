// Copyright 2021 cmj <cmj@cmj.tw>. All right reserved.
//! The LSM-tree (log-structured merge-tree) implementation.
use crate::layer::memory::NAME as MEM_LAYER_NAME;
use crate::layer::{new, Error, Layer};

/// The layer config which generate the layer by the policy.
/// The first arguement is the layer and second is the maximal number of data
/// can be stored in this layer, 0 means unlimited.
type LayerConfig = (Box<dyn Layer>, u64);

/// The implementation of LSM-tree which may support multi-layers.
pub struct LSM {
    /// The layer config and setting the rotate policy.
    config: Vec<LayerConfig>,
}

impl LSM {
    /// Create new LSM-tree without any layer.
    pub fn new() -> Self {
        Self { config: vec![] }
    }

    /// Create new LSM-tree with one an only one memory layer.
    pub fn mem() -> Self {
        LSM::new().add_layer(MEM_LAYER_NAME, 0).unwrap()
    }

    /// Add the extra layer and create new LSM instance.
    pub fn add_layer(&self, name: &str, threshold: u64) -> Result<Self, Error> {
        let mut layer_cfg: Vec<LayerConfig> = vec![];

        for (old_layer, old_threshold) in self.config.iter() {
            match new(&old_layer.name()) {
                Some(layer) => {
                    let layer_setting: LayerConfig = (layer, *old_threshold);
                    layer_cfg.push(layer_setting);
                }
                None => {
                    return Err(Error::new(format!(
                        "cannot create layer: {}",
                        old_layer.name()
                    )))
                }
            }
        }

        match new(name) {
            Some(layer) => {
                let layer_setting: LayerConfig = (layer, threshold);
                layer_cfg.push(layer_setting);
            }
            None => return Err(Error::new(format!("cannot create layer: {}", name))),
        }

        Ok(Self { config: layer_cfg })
    }

    /// Get the data from the under-layer with specified key.
    pub fn get(&self, key: &Vec<u8>) -> Result<Option<Vec<u8>>, Error> {
        for (layer, _) in self.config.iter() {
            match layer.get(key)? {
                Some(v) => {
                    // found the raw data, return
                    match v.deleted {
                        true => return Ok(None),
                        false => return Ok(Some(v.value.to_vec())),
                    }
                }
                _ => {}
            }
        }

        Ok(None)
    }

    /// Set the data to the under-layer with specified key.
    pub fn set(&mut self, key: &Vec<u8>, value: &Vec<u8>) -> Result<(), Error> {
        for (layer, _) in self.config.iter_mut() {
            let resp = layer.set(key, value);
            self.flush()?;
            return resp;
        }

        Err(Error::new("no layer supports set".to_string()))
    }

    /// Delete the data from the under-layer with specified key.
    pub fn del(&mut self, key: &Vec<u8>) -> Result<bool, Error> {
        for (layer, _) in self.config.iter_mut() {
            // always set the delete in first layer
            return layer.del(key);
        }

        // cannot delete if there is no layer, and treated as fail-deleted.
        Ok(false)
    }

    /// Count the valid element in the under-layer.
    pub fn count(&self) -> u64 {
        let mut count: u64 = 0;

        for (layer, _) in self.config.iter() {
            count += layer.count();
        }

        count
    }

    /// Flush the data to the next layer.
    fn flush(&mut self) -> Result<(), Error> {
        // Check the count of each layer and run flush if need.
        for index in 0..self.config.len() {
            let (layer, threshold) = self.config.get_mut(index).unwrap();

            if *threshold == 0 || layer.count() < *threshold {
                // 1. check the threshold.
                break;
            }
            // 2. flush to next layer.
            // 3. create layer if it is the last layer.
        }
        Ok(())
    }
}

// vim: set ft=rust tabstop=4 sw=4 expandtab:
