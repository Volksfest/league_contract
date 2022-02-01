use crate::game_types::standard::StandardGameData;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use serde::{Serialize, Deserialize};

/// An enum without an additional value.
///
/// It is used to decide which game type shall be generated
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
pub enum GameType {
    StandardGameType,
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Game {
    first_player_is_winner : bool,
    game_data : Vec<u8>,
}

impl Game {
    pub fn new_with_data(first_player_is_winner: bool, game_type: GameType, data: &String) -> Option<Self> {
        let game_data =
        match game_type {
            GameType::StandardGameType => StandardGameData::convert(data),
        }?;
        Some(Game{first_player_is_winner, game_data})
    }

    pub fn first_player_won(&self) -> bool {
        self.first_player_is_winner
    }
}