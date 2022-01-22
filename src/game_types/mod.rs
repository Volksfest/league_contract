//! This modules contains all different game types currently supported
//!
//! Right now only the `StandardGame` type will be supported
//! which is a game without additional information.
//! As such it is the bare minimum implementing the `Game` trait.
//!
//! As proof of concept a game type for Starcraft is planned.

pub mod game;
pub mod standard;

use crate::game_types::standard::StandardGame;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use serde::Serialize;

/// An enum without an additional value.
///
/// It is used to decide which game type shall be generated
#[derive(BorshDeserialize, BorshSerialize, Serialize)]
pub enum GameType {
    StandardGameType,
}

/// An enum with the actual game data
///
/// The game data shall all implement the `Game` trait
/// This enum is needed to be able to serialize concrete data types
#[derive(BorshDeserialize, BorshSerialize)]
pub enum GameVariant {
    StandardGameType(StandardGame),
}
