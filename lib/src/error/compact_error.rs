#[allow(unused_imports)]
use log::{info, error, debug};
use std::error::Error;
use tokio::sync::watch::error::*;

extern crate rmp_serde as rmps;

use super::*;

#[derive(Debug)]
pub enum CompactError {
    SinkError(SinkError),
    IO(tokio::io::Error),
    LoadError(LoadError),
    WatchError(String),
    SerializationError(SerializationError),
}

impl From<tokio::io::Error>
for CompactError {
    fn from(err: tokio::io::Error) -> CompactError {
        CompactError::IO(err)
    }
}

impl From<SinkError>
for CompactError {
    fn from(err: SinkError) -> CompactError {
        CompactError::SinkError(err)
    }
}

impl From<LoadError>
for CompactError {
    fn from(err: LoadError) -> CompactError {
        CompactError::LoadError(err)
    }
}

impl From<SerializationError>
for CompactError {
    fn from(err: SerializationError) -> CompactError {
        CompactError::SerializationError(err)
    }
}

impl From<RecvError>
for CompactError {
    fn from(err: RecvError) -> CompactError {
        CompactError::WatchError(err.to_string())
    }
}

impl<T> From<SendError<T>>
for CompactError
where T: std::fmt::Debug
{
    fn from(err: SendError<T>) -> CompactError {
        CompactError::WatchError(err.to_string())
    }
}

impl std::fmt::Display
for CompactError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            CompactError::IO(err) => {
                write!(f, "Failed to compact the chain due to an IO error - {}", err)
            },
            CompactError::SerializationError(err) => {
                write!(f, "Failed to compact the chain due to a serialization error - {}", err)
            },
            CompactError::SinkError(err) => {
                write!(f, "Failed to compact the chain due to an error in the sink - {}", err)
            },
            CompactError::WatchError(err) => {
                write!(f, "Failed to compact the chain due to an error in watch notification - {}", err)
            },
            CompactError::LoadError(err) => {
                write!(f, "Failed to compact the chain due to an error loaded on event - {}", err)
            },
        }
    }
}

impl std::error::Error
for CompactError
{
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}