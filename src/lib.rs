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

extern crate near_sdk;

pub mod game_types;
pub mod main;

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::Vector;
use near_sdk::collections::{LookupMap, LookupSet};
use near_sdk::{env, near_bindgen, require, AccountId, PanicOnDefault};

use game_types::GameType;
use main::keys::CollectionKeyTuple;
use main::league::{League, LeagueProperties, UpgradeableLeagueProperties};

/// The smart contract
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct LeagueContract {
    /// A map of named leagues. The name is given by the key.
    leagues: LookupMap<String, League>,
}

#[near_bindgen]
impl LeagueContract {
    /// Initialize the contract
    #[init]
    pub fn new() -> Self {
        require!(!env::state_exists(), "Already initialized");

        Self {
            leagues: LookupMap::new(b"0".to_vec()),
        }
    }

    pub fn create_league(
        &mut self,
        league_name: &str,
        players: &[String],
        accounts: &[AccountId],
        best_of: u8,
        game_type: GameType,
    ) {
        require!(best_of % 2 == 1, "best_of number should be odd");
        require!(!accounts.is_empty(), "Need at least one trusted account");
        require!(players.len() > 2, "League needs at least 3 participant");
        require!(
            league_name.len() > 2,
            "League name must be at least 3 chars long"
        );
        require!(
            !self.leagues.contains_key(&league_name.to_string()),
            "League with that name already exists"
        );

        let keys = CollectionKeyTuple::new(league_name);

        let prop = UpgradeableLeagueProperties::V1(LeagueProperties { best_of, game_type });

        let mut p = Vector::new(keys.get_players_key());
        for player in players {
            p.push(player);
        }
        let mut a = LookupSet::new(keys.get_trusted_key());
        for account in accounts {
            a.insert(account);
        }
        let l = League::new(keys, prop, p, a);
        self.leagues.insert(&league_name.to_string(), &l);
    }
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use crate::game_types::GameType::StandardGameType;
    use crate::LeagueContract;
    use near_sdk::test_utils::{accounts, VMContextBuilder};
    use near_sdk::testing_env;

    #[test]
    #[should_panic(expected = "The contract is not initialized")]
    fn test_default() {
        let mut context = VMContextBuilder::new();
        context
            .current_account_id(accounts(0))
            .signer_account_id(accounts(0))
            .predecessor_account_id(accounts(0));
        testing_env!(context.build());

        let _contract = LeagueContract::default();
    }

    #[test]
    fn test_new() {
        let mut context = VMContextBuilder::new();
        context
            .current_account_id(accounts(0))
            .signer_account_id(accounts(0))
            .predecessor_account_id(accounts(0));
        testing_env!(context.build());

        let _contract = LeagueContract::new();
    }

    #[test]
    fn test_create_league() {
        let mut context = VMContextBuilder::new();
        context
            .current_account_id(accounts(0))
            .signer_account_id(accounts(0))
            .predecessor_account_id(accounts(0));
        testing_env!(context.build());

        let mut contract = LeagueContract::new();
        let players = vec!["Alice".to_string(), "Bob".to_string(), "Charly".to_string()];
        let accs = vec![accounts(0), accounts(1)];
        contract.create_league(
            &"SomeLeague".to_string(),
            &players,
            &accs,
            3,
            StandardGameType,
        );
    }

    #[test]
    #[should_panic(expected = "League with that name already exists")]
    fn test_create_league_twice() {
        let mut context = VMContextBuilder::new();
        context
            .current_account_id(accounts(0))
            .signer_account_id(accounts(0))
            .predecessor_account_id(accounts(0));
        testing_env!(context.build());

        let mut contract = LeagueContract::new();
        let players = vec!["Alice".to_string(), "Bob".to_string(), "Charly".to_string()];
        let accs = vec![accounts(0), accounts(1)];
        contract.create_league(
            &"SomeLeague".to_string(),
            &players,
            &accs,
            3,
            StandardGameType,
        );
        contract.create_league(
            &"SomeLeague".to_string(),
            &players,
            &accs,
            3,
            StandardGameType,
        );
    }
}
