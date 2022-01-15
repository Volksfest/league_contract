use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::Vector;

use crate::game_types::GameVariant;

/// The match between two contestants.
///
/// This contains all games where the max is given by the league properties (`best_of`).
/// The GameVariants type must be the same as in the league properties (`game_type`)
/// The pair of the contestants is given by the `PlayerPair` typed key.
#[derive(BorshDeserialize, BorshSerialize)]
pub struct GameMatch {
    /// The vector containing the games
    games: Vector<GameVariant>,
}
