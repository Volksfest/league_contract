use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};

/// Bare minimum type for the `Game` trait
#[derive(BorshDeserialize, BorshSerialize)]
pub struct StandardGame {
    /// The only given information is the winner
    is_first_player_winner: bool,
}
