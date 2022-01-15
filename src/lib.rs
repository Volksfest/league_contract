//! A smart contract for organizing game leagues
//!
//! The smart contract handles multiple named leagues.
//! Each league can have a specific game type with additional meta information of each game.
//! Game types can still be added.
//! They also can be quite generic as long there is a 1v1 type, e.g. soccer.
//! A scoreboard and evaluation of the game meta types is not part of the contract.
//!
//! Per league there are trusted accounts
//! which can manipulate the league and the actual game scores.
//! The creation of a league will have a fee for the compensation of the memory usage
//! and will need the list of the trusted accounts.
//! Maybe the money shall be refunded?

pub mod game_types;
pub mod main;

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LookupMap;
use near_sdk::{near_bindgen, PanicOnDefault};

use main::league::League;

/// The smart contract
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct LeagueContract {
    /// A map of named leagues. The name is given by the key.
    leagues: LookupMap<String, League>,
}
