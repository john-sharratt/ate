#[allow(unused_imports)]
use log::{info, error, debug};
use std::error::Error;

extern crate rmp_serde as rmps;

use rmp_serde::decode::Error as RmpDecodeError;
use serde_json::Error as JsonError;
use tokio::task::JoinError;
use std::sync::mpsc as smpsc;
use tokio::sync::mpsc as mpsc;

use super::*;

#[derive(Debug)]
pub enum CommsError
{
    SerializationError(SerializationError),
    SendError(String),
    ReceiveError(String),
    IO(std::io::Error),
    NoReplyChannel,
    NoWireFormat,
    Disconnected,
    ShouldBlock,
    ValidationError(Vec<ValidationError>),
    #[allow(dead_code)]
    JoinError(JoinError),
    LoadError(LoadError),
    RootServerError(String),
    InternalError(String),
}

impl From<SerializationError>
for CommsError
{
    fn from(err: SerializationError) -> CommsError {
        CommsError::SerializationError(err)
    }   
}

impl From<std::io::Error>
for CommsError
{
    fn from(err: std::io::Error) -> CommsError {
        CommsError::IO(err)
    }   
}

impl From<tokio::time::error::Elapsed>
for CommsError
{
    fn from(_err: tokio::time::error::Elapsed) -> CommsError {
        CommsError::IO(std::io::Error::new(std::io::ErrorKind::TimedOut, format!("Timeout while waiting for communication channel").to_string()))
    }   
}

impl<T> From<mpsc::error::SendError<T>>
for CommsError
{
    fn from(err: mpsc::error::SendError<T>) -> CommsError {
        CommsError::SendError(err.to_string())
    }   
}

impl From<mpsc::error::RecvError>
for CommsError
{
    fn from(err: mpsc::error::RecvError) -> CommsError {
        CommsError::ReceiveError(err.to_string())
    }   
}

impl From<smpsc::RecvError>
for CommsError
{
    fn from(err: smpsc::RecvError) -> CommsError {
        CommsError::ReceiveError(err.to_string())
    }   
}

impl<T> From<smpsc::SendError<T>>
for CommsError
{
    fn from(err: smpsc::SendError<T>) -> CommsError {
        CommsError::SendError(err.to_string())
    }   
}

impl<T> From<tokio::sync::broadcast::error::SendError<T>>
for CommsError
{
    fn from(err: tokio::sync::broadcast::error::SendError<T>) -> CommsError {
        CommsError::SendError(err.to_string())
    }   
}

impl From<tokio::sync::broadcast::error::RecvError>
for CommsError
{
    fn from(err: tokio::sync::broadcast::error::RecvError) -> CommsError {
        CommsError::ReceiveError(err.to_string())
    }   
}

impl From<JoinError>
for CommsError
{
    fn from(err: JoinError) -> CommsError {
        CommsError::ReceiveError(err.to_string())
    }   
}

impl From<LoadError>
for CommsError
{
    fn from(err: LoadError) -> CommsError {
        CommsError::LoadError(err)
    }   
}

impl From<ChainCreationError>
for CommsError
{
    fn from(err: ChainCreationError) -> CommsError {
        CommsError::RootServerError(err.to_string())
    }   
}

impl From<CommitError>
for CommsError
{
    fn from(err: CommitError) -> CommsError {
        match err {
            CommitError::ValidationError(errs) => CommsError::ValidationError(errs),
            err => CommsError::InternalError(format!("commit-failed - {}", err.to_string())),
        }
    }   
}

impl From<bincode::Error>
for CommsError
{
    fn from(err: bincode::Error) -> CommsError {
        CommsError::SerializationError(SerializationError::BincodeError(err))
    }   
}

impl From<RmpDecodeError>
for CommsError {
    fn from(err: RmpDecodeError) -> CommsError {
        CommsError::SerializationError(SerializationError::DecodeError(err))
    }
}

impl From<JsonError>
for CommsError {
    fn from(err: JsonError) -> CommsError {
        CommsError::SerializationError(SerializationError::JsonError(err))
    }
}

impl std::fmt::Display
for CommsError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            CommsError::SerializationError(err) => {
                write!(f, "Serialization error while processing communication - {}", err)
            },
            CommsError::IO(err) => {
                write!(f, "IO error while processing communication - {}", err)
            },
            CommsError::ShouldBlock => {
                write!(f, "Operation should have blocked but it didn't")
            }
            CommsError::SendError(err) => {
                write!(f, "Sending error while processing communication - {}", err)
            },
            CommsError::ReceiveError(err) => {
                write!(f, "Receiving error while processing communication - {}", err)
            },
            CommsError::NoReplyChannel => {
                write!(f, "Message has no reply channel attached to it")
            },
            CommsError::NoWireFormat => {
                write!(f, "Server did not send a wire format")
            },
            CommsError::ValidationError(errs) => {
                write!(f, "Message contained event data that failed validation")?;
                for err in errs.iter() {
                    write!(f, " - {}", err.to_string())?;
                }
                Ok(())
            },
            CommsError::Disconnected => {
                write!(f, "Channel has been disconnected")
            },
            CommsError::JoinError(err) => {
                write!(f, "Receiving error while processing communication - {}", err)
            },
            CommsError::LoadError(err) => {
                write!(f, "Load error occured while processing communication - {}", err)
            },
            CommsError::RootServerError(err) => {
                write!(f, "Error at the root server while processing communication - {}", err)
            },
            CommsError::InternalError(err) => {
                write!(f, "Internal comms error - {}", err)
            },
        }
    }
}

impl std::error::Error
for CommsError
{
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}