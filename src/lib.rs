//! A smart contract for organizing game leagues
//!
//! The smart contract handles multiple named leagues.
//! Each league can have a specific game type with additional meta information of each game.
//! Game types can still be added in `game_module::game_types`.
//! They also can be quite generic as long as it is a 1v1 type, e.g. soccer.
//! A scoreboard and evaluation of the game meta types is not part of the contract
//!   mostly because it can be done outside, too. (and I postpone it..)
//!
//! Per league there are trusted accounts
//! which can manipulate the league and the actual game matches.
//! The owner (=creator) of the league may also delete the league.

extern crate near_sdk;

pub mod game_module;
pub mod main;

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::Vector;
use near_sdk::collections::{LookupMap, LookupSet};
use near_sdk::{env, near_bindgen, require, AccountId, PanicOnDefault};

use game_module::GameType;
use main::helper::CollectionKeyTuple;
use main::{League, LeagueProperties, UpgradeableLeagueProperties};

/// The smart contract
///
/// The actual smart contract struct
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct LeagueContract {
    /// A map of named leagues. The name is given by the key.
    leagues: LookupMap<String, League>,
}

#[near_bindgen]
impl LeagueContract {
    /// Initialize the contract
    ///
    /// The initialization is quite straight forward without any additional information needed.
    #[init]
    pub fn new() -> Self {
        require!(!env::state_exists(), "Already initialized");

        Self {
            leagues: LookupMap::new(b"0".to_vec()),
        }
    }

    /// The create a league call
    ///
    /// The caller is the owner of the league.
    /// He has to give a `league_name` and a list of trusted `accounts` who may also create call to this league.
    /// With `best_of` and `game_type` all necessary league properties were given.
    /// Finally the a list of `players` in the league were also needed.
    pub fn create_league(
        &mut self,
        league_name: String,
        players: Vec<String>,
        accounts: Vec<AccountId>,
        best_of: u8,
        game_type: GameType,
    ) {
        require!(best_of % 2 == 1, "best_of number should be odd");
        require!(players.len() > 2, "League needs at least 3 participant");
        require!(
            league_name.len() > 2,
            "League name must be at least 3 chars long"
        );
        require!(
            !self.leagues.contains_key(&league_name.to_string()),
            "League with that name already exists"
        );

        // Create unique keys for the collections inside the league
        let keys = CollectionKeyTuple::new(&league_name);

        let prop = UpgradeableLeagueProperties::V1(LeagueProperties { best_of, game_type });

        // Convert the player standard vec to a NEAR collection for the blockchain
        let mut p = Vector::new(keys.get_players_key());
        for player in players {
            p.push(&player);
        }
        // Do the same with the account ids. Also check if the caller does not mention himself.
        // The caller is assumed to be trusted and has as owner even more rights.
        let mut a = LookupSet::new(keys.get_trusted_key());
        let caller = &env::predecessor_account_id();
        for account in accounts {
            if account != *caller {
                a.insert(&account);
            }
        }
        let l = League::new(keys, prop, p, a);
        self.leagues.insert(&league_name, &l);
    }

    /// The delete a league call
    ///
    /// The caller has to be the owner of the league by the name `league_name`.
    /// The league won't be deleted if it is not finished except it is explicitely wished by setting
    /// `force` to true!
    pub fn delete_league(&mut self, league_name: String, force: bool) {
        // Cannot remove yet
        let league = self.leagues.get(&league_name);
        require!(league.is_some(), "League to delete not found");
        // safe to use unwrap now. Could be done in match pattern but I like this more for require!
        let league = league.unwrap();
        require!(league.caller_is_owner(), "You may not delete the league");
        require!(league.is_finished() || force, "League is not finished yet");
        self.leagues.remove(&league_name);
    }

    /// The add a game to a league call
    ///
    /// The caller has to be a trusted account of the league by the name `league_name`.
    /// The game with the given `game_data` and the players given by `player_names` will be added.
    /// The `game_data` has to be deserializable to the type given by the league's `GameType`.
    /// Also the winner has to be given by explicitely saying if the `first_in_tuple_won` or not...
    pub fn add_game(
        &mut self,
        league_name: String,
        player_names: (String, String),
        first_in_tuple_won: bool,
        game_data: String,
    ) {
        require!(player_names.0 != player_names.1, "Need different players");
        let league = self.leagues.get(&league_name);
        require!(league.is_some(), "League does not exist");
        let mut league = league.unwrap();
        league.add_game(&player_names, first_in_tuple_won, &game_data);
        self.leagues.insert(&league_name, &league);
    }
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use crate::game_module::GameType::StandardGameType;
    use crate::LeagueContract;
    use near_sdk::test_utils::{accounts, VMContextBuilder};
    use near_sdk::testing_env;

    fn create_context() -> VMContextBuilder {
        let mut context = VMContextBuilder::new();
        context
            .current_account_id(accounts(0))
            .signer_account_id(accounts(0))
            .predecessor_account_id(accounts(0));
        testing_env!(context.build());
        context
    }

    /// Test that checks if the default implementation panics as expected
    #[test]
    #[should_panic(expected = "The contract is not initialized")]
    fn test_default() {
        let _context = create_context();

        let _contract = LeagueContract::default();
    }

    /// Test that checks that the init function functions fine.
    #[test]
    fn test_new() {
        let _context = create_context();

        let _contract = LeagueContract::new();
    }

    /// Test the creation of a league
    #[test]
    fn test_create_league() {
        let _context = create_context();

        let mut contract = LeagueContract::new();
        let players = vec!["Alice".to_string(), "Bob".to_string(), "Charly".to_string()];
        let accs = vec![accounts(0), accounts(1)];
        contract.create_league("SomeLeague".to_string(), players, accs, 3, StandardGameType);
    }

    /// Test the expected panic of a name collision in leagues
    #[test]
    #[should_panic(expected = "League with that name already exists")]
    fn test_create_league_twice() {
        let _context = create_context();

        let mut contract = LeagueContract::new();
        let players = vec!["Alice".to_string(), "Bob".to_string(), "Charly".to_string()];
        let accs = vec![accounts(0), accounts(1)];
        contract.create_league(
            "SomeLeague".to_string(),
            players.clone(),
            accs.clone(),
            3,
            StandardGameType,
        );
        contract.create_league("SomeLeague".to_string(), players, accs, 3, StandardGameType);
    }

    /// Test a forced deletion of a league
    #[test]
    fn test_force_delete_unfinished_league() {
        let _context = create_context();

        let mut contract = LeagueContract::new();
        let players = vec!["Alice".to_string(), "Bob".to_string(), "Charly".to_string()];
        let accs = vec![accounts(0), accounts(1)];
        let name = "SomeLeague".to_string();
        contract.create_league(name.clone(), players, accs, 3, StandardGameType);
        contract.delete_league(name, true);
    }

    /// Test the panic of a unforced deletion of an unfinished league
    #[test]
    #[should_panic(expected = "League is not finished yet")]
    fn test_delete_unfinished_league() {
        let _context = create_context();

        let mut contract = LeagueContract::new();
        let players = vec!["Alice".to_string(), "Bob".to_string(), "Charly".to_string()];
        let accs = vec![accounts(0), accounts(1)];
        let name = "SomeLeague".to_string();
        contract.create_league(name.clone(), players, accs, 3, StandardGameType);
        contract.delete_league(name, false);
    }

    /// Test rejection of deletion of a league from a not owner
    #[test]
    #[should_panic(expected = "You may not delete the league")]
    fn test_foreigner_force_delete_unfinished_league() {
        let mut context = create_context();

        let mut contract = LeagueContract::new();
        let players = vec!["Alice".to_string(), "Bob".to_string(), "Charly".to_string()];
        let accs = vec![accounts(0), accounts(1)];
        let name = "SomeLeague".to_string();
        contract.create_league(name.clone(), players, accs, 3, StandardGameType);

        context.predecessor_account_id(accounts(1));
        testing_env!(context.build());

        contract.delete_league(name, true);
    }

    #[test]
    fn test_add_games() {
        let mut context = create_context();

        let mut contract = LeagueContract::new();
        let name = "SomeLeague".to_string();
        let players = vec!["Alice".to_string(), "Bob".to_string(), "Charly".to_string()];
        let accs = vec![accounts(1)];
        contract.create_league(name.clone(), players.clone(), accs, 3, StandardGameType);

        context.predecessor_account_id(accounts(1));
        testing_env!(context.build());

        contract.add_game(
            name.clone(),
            (players[0].clone(), players[1].clone()),
            true,
            "{}".to_string(),
        );
        contract.add_game(
            name.clone(),
            (players[0].clone(), players[1].clone()),
            false,
            "{}".to_string(),
        );
        contract.add_game(
            name,
            (players[0].clone(), players[1].clone()),
            true,
            "{}".to_string(),
        );
        // TODO add a view later to verify finished game
    }

    #[test]
    #[should_panic(expected = "Game data cannot be parsed in the game type")]
    fn test_add_wrong_game_data() {
        let mut context = create_context();

        let mut contract = LeagueContract::new();
        let name = "SomeLeague".to_string();
        let players = vec!["Alice".to_string(), "Bob".to_string(), "Charly".to_string()];
        let accs = vec![accounts(1)];
        contract.create_league(name.clone(), players.clone(), accs, 3, StandardGameType);

        context.predecessor_account_id(accounts(1));
        testing_env!(context.build());

        contract.add_game(
            name,
            (players[0].clone(), players[1].clone()),
            true,
            "{house: true}".to_string(),
        );
    }

    #[test]
    #[should_panic(expected = "Match is already finished")]
    fn test_add_game_to_finished_match() {
        let mut context = create_context();

        let mut contract = LeagueContract::new();
        let name = "SomeLeague".to_string();
        let players = vec!["Alice".to_string(), "Bob".to_string(), "Charly".to_string()];
        let accs = vec![accounts(1)];
        contract.create_league(name.clone(), players.clone(), accs, 3, StandardGameType);

        context.predecessor_account_id(accounts(1));
        testing_env!(context.build());

        contract.add_game(
            name.clone(),
            (players[0].clone(), players[1].clone()),
            true,
            "{}".to_string(),
        );
        contract.add_game(
            name.clone(),
            (players[0].clone(), players[1].clone()),
            true,
            "{}".to_string(),
        );
        contract.add_game(
            name,
            (players[0].clone(), players[1].clone()),
            true,
            "{}".to_string(),
        );
    }

    #[test]
    #[should_panic(expected = "At least one player not found in the league")]
    fn test_add_game_with_wrong_player() {
        let mut context = create_context();

        let mut contract = LeagueContract::new();
        let name = "SomeLeague".to_string();
        let players = vec!["Alice".to_string(), "Bob".to_string(), "Charly".to_string()];
        let accs = vec![accounts(1)];
        contract.create_league(name.clone(), players.clone(), accs, 3, StandardGameType);

        context.predecessor_account_id(accounts(1));
        testing_env!(context.build());

        contract.add_game(
            name,
            ("Malory".to_string(), players[1].clone()),
            true,
            "{}".to_string(),
        );
    }

    #[test]
    #[should_panic(expected = "Need different players")]
    fn test_add_game_with_player_twice() {
        let mut context = create_context();

        let mut contract = LeagueContract::new();
        let name = "SomeLeague".to_string();
        let players = vec!["Alice".to_string(), "Bob".to_string(), "Charly".to_string()];
        let accs = vec![accounts(1)];
        contract.create_league(name.clone(), players.clone(), accs, 3, StandardGameType);

        context.predecessor_account_id(accounts(1));
        testing_env!(context.build());

        contract.add_game(
            name,
            (players[1].clone(), players[1].clone()),
            true,
            "{}".to_string(),
        );
    }

    #[test]
    fn test_delete_finished_match() {
        // To lazy to define a new predecessor here
        let _context = create_context();

        let mut contract = LeagueContract::new();
        let name = "SomeLeague".to_string();
        let players = vec!["Alice".to_string(), "Bob".to_string(), "Charly".to_string()];
        contract.create_league(
            name.clone(),
            players.clone(),
            Vec::new(),
            3,
            StandardGameType,
        );

        contract.add_game(
            name.clone(),
            (players[0].clone(), players[1].clone()),
            true,
            "{}".to_string(),
        );
        contract.add_game(
            name.clone(),
            (players[0].clone(), players[1].clone()),
            true,
            "{}".to_string(),
        );

        contract.add_game(
            name.clone(),
            (players[0].clone(), players[2].clone()),
            false,
            "{}".to_string(),
        );
        contract.add_game(
            name.clone(),
            (players[0].clone(), players[2].clone()),
            false,
            "{}".to_string(),
        );

        contract.add_game(
            name.clone(),
            (players[1].clone(), players[2].clone()),
            true,
            "{}".to_string(),
        );
        contract.add_game(
            name.clone(),
            (players[1].clone(), players[2].clone()),
            false,
            "{}".to_string(),
        );
        contract.add_game(
            name.clone(),
            (players[1].clone(), players[2].clone()),
            true,
            "{}".to_string(),
        );

        // None forced of course
        contract.delete_league(name, false);
    }

    #[test]
    fn test_simpler_league() {
        // To lazy to define a new predecessor here
        let _context = create_context();

        let mut contract = LeagueContract::new();
        let name = "SomeLeague".to_string();
        let players = vec!["Alice".to_string(), "Bob".to_string(), "Charly".to_string()];
        contract.create_league(
            name.clone(),
            players.clone(),
            Vec::new(),
            1,
            StandardGameType,
        );

        contract.add_game(
            name.clone(),
            (players[0].clone(), players[1].clone()),
            true,
            "{}".to_string(),
        );
        contract.add_game(
            name.clone(),
            (players[0].clone(), players[2].clone()),
            false,
            "{}".to_string(),
        );
        contract.add_game(
            name.clone(),
            (players[1].clone(), players[2].clone()),
            true,
            "{}".to_string(),
        );

        // None forced of course
        contract.delete_league(name, false);
    }

    #[test]
    fn test_complex_league() {
        // To lazy to define a new predecessor here
        let _context = create_context();

        let mut contract = LeagueContract::new();
        let name = "SomeLeague".to_string();
        let players = vec!["Alice".to_string(), "Bob".to_string(), "Charly".to_string()];
        contract.create_league(
            name.clone(),
            players.clone(),
            Vec::new(),
            5,
            StandardGameType,
        );

        contract.add_game(
            name.clone(),
            (players[0].clone(), players[1].clone()),
            true,
            "{}".to_string(),
        );
        contract.add_game(
            name.clone(),
            (players[0].clone(), players[1].clone()),
            false,
            "{}".to_string(),
        );
        contract.add_game(
            name.clone(),
            (players[0].clone(), players[1].clone()),
            false,
            "{}".to_string(),
        );
        contract.add_game(
            name.clone(),
            (players[0].clone(), players[1].clone()),
            true,
            "{}".to_string(),
        );
        contract.add_game(
            name,
            (players[0].clone(), players[1].clone()),
            true,
            "{}".to_string(),
        );

        // TODO add a view later to verify finished game
    }

    #[test]
    #[should_panic(expected = "Match is already finished")]
    fn test_complex_league_failing() {
        // To lazy to define a new predecessor here
        let _context = create_context();

        let mut contract = LeagueContract::new();
        let name = "SomeLeague".to_string();
        let players = vec!["Alice".to_string(), "Bob".to_string(), "Charly".to_string()];
        contract.create_league(
            name.clone(),
            players.clone(),
            Vec::new(),
            5,
            StandardGameType,
        );

        contract.add_game(
            name.clone(),
            (players[0].clone(), players[1].clone()),
            true,
            "{}".to_string(),
        );
        contract.add_game(
            name.clone(),
            (players[0].clone(), players[1].clone()),
            false,
            "{}".to_string(),
        );
        contract.add_game(
            name.clone(),
            (players[0].clone(), players[1].clone()),
            false,
            "{}".to_string(),
        );
        contract.add_game(
            name.clone(),
            (players[0].clone(), players[1].clone()),
            true,
            "{}".to_string(),
        );
        contract.add_game(
            name.clone(),
            (players[0].clone(), players[1].clone()),
            true,
            "{}".to_string(),
        );
        contract.add_game(
            name,
            (players[0].clone(), players[1].clone()),
            false,
            "{}".to_string(),
        );
    }
}
