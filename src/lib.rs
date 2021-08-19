// Copyright 2021 cmj <cmj@cmj.tw>. All right reserved.
//! argus: the multi-layer store.
pub use crate::error::Error;
pub use crate::traits::{Store, Uid};

mod error;
mod traits;

pub use crate::field::{Field, FieldType};
mod field;
// vim: set tabstop=4 sw=4 expandtab:
