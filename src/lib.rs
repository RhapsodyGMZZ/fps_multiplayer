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
pub const PRIVATE_KEY: &[u8; 32] = &[
    3, 2, 32, 1, 35, 1, 4, 2, 32, 1, 35, 132, 2, 32, 1, 35, 132, 234, 21, 23, 54, 56, 55, 76, 46,
    147, 9, 8, 57, 76, 68, 97,
];


/////////////////////////////////////SOME INFOS ABOUT DIFFERENTS CHANNELS /////////////////////////////

// Configuration for a reliable and ordered channel,
// messages will be received in the order that they've been sent
// If a message is lost, it'll be resent
// EXAMPLE: Pings, chat messages...
// 
// pub struct ReliableChannelConfig {
//     pub channel_id: u8,
// }

// Configuration for a unreliable and unordered channel,
// messages sent in this channel can be lost and arrive in a diff order than they've been sent <br>
// EXAMPLE: Player's current position...
// 
// pub struct UnreliableChannelConfig {
//     pub channel_id: u8,
// }

// Configuration for a block channel, used for sending big and reliable messages,
// that are not so frequent <br>
// EXAMPLE: level initialization, player's inventory...
// pub struct BlockChannelConfig {
//     pub channel_id: u8,
// }

///////////////////////////////////////////////////////////////////////////////////////////////////////
