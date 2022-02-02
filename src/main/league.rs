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
use crate::game_types::game::GameType;

use near_sdk::collections::LookupSet;
use near_sdk::collections::UnorderedMap;
use near_sdk::collections::Vector;
use near_sdk::require;
use near_sdk::AccountId;

use near_sdk::env;

use crate::game_types::game::Game;
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

    pub fn first(&self) -> u8 {
        self.first
    }

    pub fn second(&self) -> u8 {
        self.second
    }

    pub fn is_swapped(&self, should_be_first: u8) -> bool {
        self.first != should_be_first
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
    /// The owner of the league (in this context the same as the creator)
    owner: AccountId,
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
            owner: env::predecessor_account_id(),
        }
    }

    pub fn caller_is_allowed(&self) -> bool {
        self.trusted_account_ids
            .contains(&env::predecessor_account_id())
            || self.caller_is_owner()
    }

    pub fn caller_is_owner(&self) -> bool {
        env::predecessor_account_id() == self.owner
    }

    pub fn is_finished(&self) -> bool {
        let p = self.players.len();
        // Gaussian sum formula.
        // It yields to the number of matches where each player played with everybody.
        // Or at least started...
        if self.game_matches.len() != p * (p - 1) / 2 {
            return false;
        } else {
            // So in case everyone started to play against each other
            // it still needs to be confirmed that they also finished
            for (_pair, game_match) in self.game_matches.iter() {
                if !game_match.winner(self.properties.get_best_of()).exist() {
                    return false;
                }
            }
        }
        true
    }

    pub fn add_game(
        &mut self,
        player_names: &(String, String),
        first_in_tuple_won: bool,
        game_data: &String,
    ) {
        let mut first: Option<u8> = None;
        let mut second: Option<u8> = None;
        for (idx, i) in self.players.iter().enumerate() {
            if i == player_names.0 {
                first = Some(idx as u8);
            } else if i == player_names.1 {
                second = Some(idx as u8);
            }
        }
        require!(
            first.is_some() && second.is_some(),
            "At least one player not found in the league"
        );
        let pair = PlayerPair::new(first.unwrap(), second.unwrap());
        let game_match = self.game_matches.get(&pair);

        let mut game_match = match game_match {
            None => GameMatch::new(),
            Some(m) => m,
        };
        require!(
            !game_match.winner(self.properties.get_best_of()).exist(),
            "Match is already finished"
        );
        // Swaps the win flag if the names were swapped in the first place
        let first_has_won = pair.is_swapped(first.unwrap()) ^ first_in_tuple_won;
        let game = Game::new_with_data(first_has_won, self.properties.get_game_type(), game_data);
        require!(
            game.is_some(),
            "Game data cannot be parsed in the game type"
        );
        game_match.add_game(game.unwrap());
        self.game_matches.insert(&pair, &game_match);
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

/// Convenient implementation
impl UpgradeableLeagueProperties {
    pub fn get_best_of(&self) -> u8 {
        match self {
            UpgradeableLeagueProperties::V1(prop) => prop.best_of,
        }
    }

    pub fn get_game_type(&self) -> GameType {
        match self {
            UpgradeableLeagueProperties::V1(prop) => prop.game_type.clone(),
        }
    }
}
