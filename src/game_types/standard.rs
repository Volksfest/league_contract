use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use serde::{Serialize, Deserialize};
use serde_json;

/// s
#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct StandardGameData {

}

impl StandardGameData {
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
