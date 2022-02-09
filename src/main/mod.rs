//! The main logic of the league
//!
//! Disclaimer: Match would be a wonderful word for a bundle of games between two contestants.
//! Unfortunately, `match` is a bad name in rust due to obvious reasons...
//! I decided for `GameMatch`
//!

pub mod helper;

use helper::CollectionKeyTuple;

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LookupSet;
use near_sdk::collections::UnorderedMap;
use near_sdk::collections::Vector;
use near_sdk::env;
use near_sdk::require;
use near_sdk::AccountId;

// Connection to the games in the other module
use crate::game_module::Game;
use crate::game_module::GameType;

/// The contestants of a `GameMatch`.
#[derive(BorshDeserialize, BorshSerialize)]
pub struct PlayerPair {
    first: u8,
    second: u8,
}

impl PlayerPair {
    /// Create a new unique `PlayerPair`
    ///
    /// Unique in this case means that the two contestants are commutative.
    /// This means that `first` and `second` can be swapped
    ///  but still an equal object would be created
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

    /// Get the first players index
    pub fn first(&self) -> u8 {
        self.first
    }

    /// Get the second players index
    pub fn second(&self) -> u8 {
        self.second
    }

    /// Check if the indices were swapped
    ///
    /// This is an important convenient function as the indices by the caller can be
    /// in different order than this unique struct contains them.
    /// A swapped order may interfere in the interpretation of some data like the winner.
    pub fn is_swapped(&self, should_be_first: u8) -> bool {
        self.first != should_be_first
    }
}

/// The actual league object holding everything together.
///
/// The properties are hold in an additional struct.
/// No idea why I had that great idea, probably due to the upgradeable idea but this could have been
/// applied to the League itself as well...
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
    /// Create a new league
    ///
    /// The `keys` have to be given as the league has no idea how it is named.
    /// The collections `players` and `trusted_Account_ids` are already created
    /// and as such the keys are created, too.
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

    /// Check if the caller of a call on this league is permitted
    ///
    /// Permitted is anybody inside the `trusted_account_ids` set or the `owner`
    pub fn caller_is_allowed(&self) -> bool {
        self.trusted_account_ids
            .contains(&env::predecessor_account_id())
            || self.caller_is_owner()
    }

    /// Check if the caller is the owner of the league
    pub fn caller_is_owner(&self) -> bool {
        env::predecessor_account_id() == self.owner
    }

    /// Check if the league is finished
    ///
    /// This means that every match is finished and no additional game can be added.
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

    /// Add a game to the league
    ///
    /// As such add a game with additional `game_data` for the players given by name.
    /// The indices of these players are searched and then checked if the game can be added to a match.
    /// The game itself needs to be created by additional conversion of the `game_data` json.
    ///
    /// Beware! This method can panic too!
    pub fn add_game(
        &mut self,
        player_names: &(String, String),
        first_in_tuple_won: bool,
        game_data: &String,
    ) {
        // Wonderful iteration through all the names to find the correct indices
        // Maybe it could be done more beautiful but I think this is well enough
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
        ); // Check if both indices exist

        let pair = PlayerPair::new(first.unwrap(), second.unwrap());
        let game_match = self.game_matches.get(&pair);

        let mut game_match = match game_match {
            None => GameMatch::new(),
            Some(m) => m,
        };
        require!(
            !game_match.winner(self.properties.get_best_of()).exist(),
            "Match is already finished"
        ); // Check if the game match is already full (has a winner)

        // Swaps the win flag if the names were swapped in the first place
        let first_has_won = pair.is_swapped(first.unwrap()) ^ first_in_tuple_won;
        let game = Game::new_with_data(first_has_won, self.properties.get_game_type(), game_data);
        require!(
            game.is_some(),
            "Game data cannot be parsed in the game type"
        ); // Check if game is creatable (thus the game data is convertible = the game data conforms the corresponding data struct)
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

impl UpgradeableLeagueProperties {
    /// Convenient implementation to get the `best_of` value independant of the `LeagueProperties` version
    pub fn get_best_of(&self) -> u8 {
        match self {
            UpgradeableLeagueProperties::V1(prop) => prop.best_of,
        }
    }

    /// Convenient implementation to get the `game_type` value independant of the `LeagueProperties` version
    pub fn get_game_type(&self) -> GameType {
        match self {
            UpgradeableLeagueProperties::V1(prop) => prop.game_type.clone(),
        }
    }
}

/// Description who the winner is if he exists
pub enum Winner {
    FirstPlayer,
    SecondPlayer,
    None,
}

impl Winner {
    /// Check if a winner exists
    ///
    /// If not it means that the game match is still ongoing.
    pub fn exist(&self) -> bool {
        !matches!(self, Winner::None)
    }
}

/// The match between two contestants.
///
/// This contains all games where the max is given by the league properties (`best_of`).
/// The GameVariants type must be the same as in the league properties (`game_type`)
/// The pair of the contestants is given by the `PlayerPair` typed key.
#[derive(BorshDeserialize, BorshSerialize, Default)]
pub struct GameMatch {
    /// The vector containing the games
    games: Vec<Game>,
}

impl GameMatch {
    /// Create a new empty game match
    pub fn new() -> Self {
        GameMatch { games: Vec::new() }
    }

    /// Return the winner of a game match
    ///
    /// This checks each game and returns the winner according to the ''best of'' rules.
    /// The winner can also be not determined yet due to missing games
    pub fn winner(&self, best_of: u8) -> Winner {
        let mut a = 0;
        let mut b = 0;
        for i in 0..best_of {
            match self.games.get(i as usize) {
                None => break,
                Some(s) => match s.first_player_won() {
                    true => a += 1,
                    false => b += 1,
                },
            }
        }

        let win_condition = (best_of + 1) / 2;

        if a == win_condition {
            return Winner::FirstPlayer;
        }
        if b == win_condition {
            return Winner::SecondPlayer;
        }
        Winner::None
    }

    /// Add a new game
    ///
    /// Actually insert would be maybe a better terminology
    pub fn add_game(&mut self, game: Game) {
        self.games.push(game);
    }
}
