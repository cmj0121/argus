// Copyright 2021 cmj <cmj@cmj.tw>. All right reserved.
//! The trait of the store, provide the general data operations.
use crate::{Error, Field, Uid};
use std::boxed::Box;

/// The pseudo trait of the data, provide several necessay operations
pub trait Store {
    /// close all of the used resouce and clean-up temporary files.
    fn close(&self) -> Result<(), Error>;

    /// save the data into the store
    fn save(&mut self, uid: &dyn Uid, field: &dyn Field) -> Result<(), Error>;
    /// load the data from the store
    fn load(&mut self, uid: &dyn Uid) -> Result<Option<Box<dyn Field>>, Error>;
    /// mark the data as deleted in the store
    fn delete(&mut self, uid: &dyn Uid) -> Result<bool, Error>;
    /// flush the store and make data compact
    fn flush(&mut self) -> Result<(), Error>;
    /// erase all data from the store as initization
    fn erase(&mut self) -> Result<(), Error>;

    /// total number of valid data in the store
    fn len(&self) -> usize;

    // iterate the valid keys or all keys
    // iterate the valid key-value pair or all pairs

    // save the metadata
    // load the metadata
}

// vim: set tabstop=4 sw=4 expandtab:
