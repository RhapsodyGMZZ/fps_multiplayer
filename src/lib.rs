pub use bevy::prelude::*;
pub use bevy_renet::*;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum ClientMessage {
    Ping,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ServerMessage {
    Pong,
}

pub const PROTOCOL_ID: u64 = 1000;

// For SECURE CONNECTION
pub const PRIVATE_KEY: &[u8; 32] = &[3,2,32,1,35,1,4,2,32,1,35,132,2,32,1,35,132,234,21,23,54,56,55,76,46,147,9,8,57,76,68,97]; 