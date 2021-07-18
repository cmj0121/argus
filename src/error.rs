// Copyright 2021 cmj <cmj@cmj.tw>. All right reserved.
//! The kind of the error message.
use std::fmt;

/// The pre-define and customized error message
#[derive(Debug, Clone)]
pub enum Error {
    /// The default error messsage
    NotImplemented,
    /// The customized error message
    Message(String),
}

/// Format trait for an empty format for Error.
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Message(s) => write!(f, "{}", s),
            _ => write!(f, "{:?}", self),
        }
    }
}

// vim: set tabstop=4 sw=4 expandtab:
