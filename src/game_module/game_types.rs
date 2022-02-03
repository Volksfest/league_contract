//! Contains the definition of different game types

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use serde::{Deserialize, Serialize};
use serde_json;

/// A type with no additional data
///
/// This represents the additional data of a standard game.
/// These ones do not contain any additional data...
#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct StandardGameData {}

impl StandardGameData {
    /// Converts the json data into a borsh serialization
    ///
    /// As a TODO: This shall be reimplemented as a derive macro
    pub fn convert(data: &String) -> Option<Vec<u8>> {
        match serde_json::from_str::<StandardGameData>(data) {
            Ok(obj) => match borsh::to_vec(&obj) {
                Ok(serialization) => Some(serialization),
                Err(_) => None,
            },
            Err(_) => None,
        }
    }
}
