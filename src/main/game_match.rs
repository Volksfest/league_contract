use crate::game_types::game::Game;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};

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

pub enum Winner {
    FirstPlayer,
    SecondPlayer,
    None,
}

impl Winner {
    pub fn exist(&self) -> bool {
        matches!(self, Winner::None)
    }
}

impl GameMatch {
    pub fn new() -> Self {
        GameMatch { games: Vec::new() }
    }

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

    pub fn add_game(&mut self, game: Game) {
        self.games.push(game);
    }
}
