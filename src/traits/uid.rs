// Copyright 2021 cmj <cmj@cmj.tw>. All right reserved.
//! The UID (unique identifier) definination.
use crate::Error;

/// The pseudo trait of the UID, define the necessay operation. It should be
/// pre-defined when store open and fixed-length in the store, which contains
/// 1) timestamp 2) cluster and 3) machine information.
pub trait Uid {
    /// generate the UID from bytes
    fn from_bytes(bytes: &[u8]) -> Result<Self, Error>
    where
        Self: Sized;

    /// generate the bytes from UID
    fn as_bytes(&self) -> &'static [u8];

    /// show as the human-readable format
    fn as_str(&self) -> &str;

    /// the total bits used in the UID
    fn len(&self) -> usize;

    /// the timestamp of ms, based on the epoch time (UTC+0)
    fn timestamp_ms(&self) -> u64;
    /// get the cluster id
    fn cluster_id(&self) -> u16;
    /// get the machine id
    fn machine_id(&self) -> u16;
}

// vim: set tabstop=4 sw=4 expandtab:
