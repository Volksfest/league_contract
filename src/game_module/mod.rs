//! This modules contains all different game types with their data
//!
//! The `Game` struct contains one of the structs inside `game_types` depending on the
//! league properties `GameType`.

pub mod game_types;

use crate::game_module::game_types::StandardGameData;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use serde::{Deserialize, Serialize};
use strum_macros::EnumVariantNames;

/// An enum to describe the game type
///
/// It is used to decide to which game the data shall be deserialized
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone, EnumVariantNames)]
pub enum GameType {
    StandardGameType,
}

/// The game type
///
/// The contestants are given by the containing `GameMatch`
/// Here only the winner of the single game is given and the additional serialized data
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Game {
    first_player_is_winner: bool,
    game_data: Vec<u8>,
}

impl Game {
    /// Create a new game
    ///
    /// `first_player_is_winner` does exactly what its name is.
    /// The 'game_type' is the type to decide in which the JSON `data` shall be deserialized
    pub fn new_with_data(
        first_player_is_winner: bool,
        game_type: GameType,
        data: &String,
    ) -> Option<Self> {
        let game_data = match game_type {
            GameType::StandardGameType => StandardGameData::convert(data),
        }?;
        Some(Game {
            first_player_is_winner,
            game_data,
        })
    }

    /// Retrieve if the first player is the winner
    pub fn first_player_won(&self) -> bool {
        self.first_player_is_winner
    }

    /// Retrieve the game content as JSON
    ///
    /// TODO this will be nested into another json. this looks ugly as string
    pub fn game_content(&self, game_type: &GameType) -> String {
        match game_type {
            GameType::StandardGameType => StandardGameData::convert_back(&self.game_data),
        }
    }
}
