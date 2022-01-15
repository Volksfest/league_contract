//! This module contains the main internal logic.
//!
//! Disclaimer: Match would be a wonderful word for a bundle of games between two contestants.
//! Unfortunately, `match` is a bad name in rust due to obvious reasons...
//! I decided for `GameMatch`
//!
//! The player vector `players` shall be immutable.
//! Indexes to that vector are used Instead of references to the player.
//! Mostly easier to handle and actually `PlayerPair` can be made unambiguously by rules.

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};

use super::game_match::GameMatch;
use crate::game_types::GameType;

use near_sdk::collections::LookupMap;
use near_sdk::collections::LookupSet;
use near_sdk::collections::Vector;
use near_sdk::AccountId;

/// The contestants of a `GameMatch`.
#[derive(BorshDeserialize, BorshSerialize)]
pub struct PlayerPair {
    first: usize,
    second: usize,
}

/// The actual league object holding everything together.
#[derive(BorshDeserialize, BorshSerialize)]
pub struct League {
    /// The properties of the league.
    properties: UpgradeableLeagueProperties,
    /// The (constant) list of all participants of the league
    players: Vector<String>,
    /// The actual games between all contestants.
    game_matches: LookupMap<PlayerPair, GameMatch>,
    /// The set of accounts being allowed to manipulate the league. Can be seen as moderators.
    trusted_accounts: LookupSet<AccountId>,
}

/// The upgradeable enum for the properties to be able to easily upgrade the league
#[derive(BorshDeserialize, BorshSerialize)]
pub enum UpgradeableLeagueProperties {
    V1(LeagueProperties),
}

/// Current version of the properties
#[derive(BorshDeserialize, BorshSerialize)]
pub struct LeagueProperties {
    /// The maximum amount of games each `GameMatch` may have
    best_of: usize,
    /// The actual type of the game which is played.
    game_type: GameType,
}
