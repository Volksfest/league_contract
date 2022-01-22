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

use near_sdk::collections::LookupSet;
use near_sdk::collections::UnorderedMap;
use near_sdk::collections::Vector;
use near_sdk::AccountId;

use near_sdk::env;

use crate::main::keys::CollectionKeyTuple;

/// The contestants of a `GameMatch`.
#[derive(BorshDeserialize, BorshSerialize)]
pub struct PlayerPair {
    first: u8,
    second: u8,
}

impl PlayerPair {
    pub fn new(first: u8, second: u8) -> Self {
        if first <= second {
            PlayerPair { first, second }
        } else {
            PlayerPair {
                first: second,
                second: first,
            }
        }
    }
}

/// The actual league object holding everything together.
#[derive(BorshDeserialize, BorshSerialize)]
pub struct League {
    /// The properties of the league.
    properties: UpgradeableLeagueProperties,
    /// The (constant) list of all participants of the league
    players: Vector<String>,
    /// The actual games between all contestants.
    game_matches: UnorderedMap<PlayerPair, GameMatch>,
    /// The set of accounts being allowed to manipulate the league. Can be seen as moderators.
    trusted_account_ids: LookupSet<AccountId>,
}

impl League {
    pub fn new(
        keys: CollectionKeyTuple,
        properties: UpgradeableLeagueProperties,
        players: Vector<String>,
        trusted_account_ids: LookupSet<AccountId>,
    ) -> Self {
        League {
            properties,
            players,
            trusted_account_ids,
            game_matches: UnorderedMap::new(keys.get_matches_key()),
        }
    }

    pub fn is_allowed(&self) -> bool {
        self.trusted_account_ids
            .contains(&env::predecessor_account_id())
    }

    pub fn is_finished(&self) -> bool {
        let p = self.players.len();
        // Gaussian sum formula.
        // It yields to the number of matches where each player played with everybody.
        self.game_matches.len() == p * (p - 1) / 2
    }
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
    pub best_of: u8,
    /// The actual type of the game which is played.
    pub game_type: GameType,
}
