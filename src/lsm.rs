// Copyright 2021 cmj <cmj@cmj.tw>. All right reserved.
//! The LSM-tree (log-structured merge-tree) implementation.
use crate::layer::{new, Error, Layer};

/// The layer config which generate the layer by the policy.
/// The first arguement is the name of the layer and second is the maximal number
/// of data can be stored in this layer, 0 means unlimited.
pub type LayerConfig = (String, u32);

/// The inner usage layer config that store the exactly layer and threshold.
type InnerLayerConfig = (String, u32, Box<dyn Layer>);

/// The implementation of LSM-tree which may support multi-layers.
pub struct LSMTree {
    /// The layer config and setting the rotate policy.
    config: Vec<InnerLayerConfig>,
}

impl LSMTree {
    /// Create new LSM-tree without any layer.
    pub fn new() -> Self {
        Self { config: vec![] }
    }

    /// Create new LSM-tree with one an only one memory layer.
    pub fn mem() -> Self {
        let cfg: LayerConfig = ("mem".to_string(), 0);
        LSMTree::new().add_layer(cfg).unwrap()
    }

    pub fn add_layer(&self, cfg: LayerConfig) -> Result<Self, Error> {
        let mut layer_cfg: Vec<InnerLayerConfig> = vec![];

        for (name, threshold, _) in self.config.iter() {
            match new(&name) {
                Some(layer) => {
                    let layer_setting: InnerLayerConfig = (name.to_string(), *threshold, layer);
                    layer_cfg.push(layer_setting);
                }
                None => return Err(Error::new(format!("cannot create layer: {}", name))),
            }
        }

        let (name, threshold) = cfg;
        match new(&name) {
            Some(layer) => {
                let layer_setting: InnerLayerConfig = (name, threshold, layer);
                layer_cfg.push(layer_setting);
            }
            None => return Err(Error::new(format!("cannot create layer: {}", name))),
        }

        Ok(Self { config: layer_cfg })
    }

    /// Get the data from the under-layer with specified key.
    pub fn get(&self, key: &Vec<u8>) -> Result<Option<Vec<u8>>, Error> {
        for (_, _, layer) in self.config.iter() {
            match layer.get(key)? {
                Some(v) if !v.deleted => return Ok(Some(v.value.to_vec())),
                _ => {}
            }
        }

        Ok(None)
    }

    /// Set the data to the under-layer with specified key.
    pub fn set(&mut self, key: &Vec<u8>, value: &Vec<u8>) -> Result<(), Error> {
        for (_, _, layer) in self.config.iter_mut() {
            return layer.set(key, value);
        }

        Err(Error::new("no layer supports set".to_string()))
    }

    /// Delete the data from the under-layer with specified key.
    pub fn del(&mut self, key: &Vec<u8>) -> Result<bool, Error> {
        for (_, _, layer) in self.config.iter_mut() {
            return layer.del(key);
        }

        Ok(false)
    }

    /// Count the valid element in the under-layer.
    pub fn count(&self) -> usize {
        let mut count: usize = 0;

        for (_, _, layer) in self.config.iter() {
            count += layer.count();
        }

        count
    }
}

// vim: set ft=rust tabstop=4 sw=4 expandtab:
