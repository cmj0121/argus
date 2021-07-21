// Copyright 2021 cmj <cmj@cmj.tw>. All right reserved.
//! Argus: the multi-layer store.
pub use crate::command::Command;

pub mod error;

mod command;

extern crate pest;
#[macro_use]
extern crate pest_derive;
// vim: set tabstop=4 sw=4 expandtab:
