#[allow(unused_imports)]
use log::{info, error, debug};
use std::error::Error;

extern crate rmp_serde as rmps;

#[derive(Debug)]
pub enum CryptoError {
    NoIvPresent,    
}

impl From<CryptoError>
for std::io::Error {
    fn from(error: CryptoError) -> Self {
        match error {
            CryptoError::NoIvPresent => std::io::Error::new(std::io::ErrorKind::Other, "The metadata does not have IV component present")
        }
    }
}

impl std::fmt::Display
for CryptoError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            CryptoError::NoIvPresent => {
                write!(f, "The event has no initialization vector")
            },
        }
    }
}

impl std::error::Error
for CryptoError
{
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}