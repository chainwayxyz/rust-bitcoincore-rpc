// Rust JSON-RPC Library
// Written in 2015 by
//     Andrew Poelstra <apoelstra@wpsoftware.net>
//
// To the extent possible under law, the author(s) have dedicated all
// copyright and related and neighboring rights to this software to
// the public domain worldwide. This software is distributed without
// any warranty.
//
// You should have received a copy of the CC0 Public Domain Dedication
// along with this software.
// If not, see <http://creativecommons.org/publicdomain/zero/1.0/>.
//

//! # Error handling
//!
//! Some useful methods for creating Error objects
//!

use std::{error, fmt};

use serde::{Deserialize, Serialize};
use serde_json;

/// A library error
#[derive(Debug)]
pub enum Error {
    /// A transport error
    Transport(Box<dyn error::Error + Send + Sync>),
    /// Json error
    Json(serde_json::Error),
    /// Error response
    Rpc(RpcError),
    /// Response to a request did not have the expected nonce
    NonceMismatch,
    /// Response to a request had a jsonrpc field other than "2.0"
    VersionMismatch,
    /// Batches can't be empty
    EmptyBatch,
    /// Too many responses returned in batch
    WrongBatchResponseSize,
    /// Batch response contained a duplicate ID
    BatchDuplicateResponseId(serde_json::Value),
    /// Batch response contained an ID that didn't correspond to any request ID
    WrongBatchResponseId(serde_json::Value),
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Error {
        Error::Json(e)
    }
}

impl From<RpcError> for Error {
    fn from(e: RpcError) -> Error {
        Error::Rpc(e)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Transport(ref e) => write!(f, "transport error: {}", e),
            Error::Json(ref e) => write!(f, "JSON decode error: {}", e),
            Error::Rpc(ref r) => write!(f, "RPC error response: {:?}", r),
            Error::BatchDuplicateResponseId(ref v) => {
                write!(f, "duplicate RPC batch response ID: {}", v)
            }
            Error::WrongBatchResponseId(ref v) => write!(f, "wrong RPC batch response ID: {}", v),
            Error::NonceMismatch => write!(f, "Nonce of response did not match nonce of request"),
            Error::VersionMismatch => write!(f, "`jsonrpc` field set to non-\"2.0\""),
            Error::EmptyBatch => write!(f, "batches can't be empty"),
            Error::WrongBatchResponseSize => write!(f, "too many responses returned in batch"),
        }
    }
}

impl error::Error for Error {
    fn cause(&self) -> Option<&dyn error::Error> {
        match *self {
            Error::Transport(ref e) => Some(&**e),
            Error::Json(ref e) => Some(e),
            _ => None,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
/// A JSONRPC error object
pub struct RpcError {
    /// The integer identifier of the error
    pub code: i32,
    /// A string describing the error
    pub message: String,
    /// Additional data specific to the error
    pub data: Option<Box<serde_json::value::RawValue>>,
}
