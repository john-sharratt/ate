#[allow(unused_imports)]
use log::{info, warn, debug, error};
use serde::*;

use super::*;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Sudo {
    pub email: String,
    pub uid: u32,
    pub google_auth: String,
    pub secret: String,
    pub qr_code: String,
    pub access: Vec<Authorization>,
    pub groups: Vec<String>,
}