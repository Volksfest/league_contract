//! Creating unique keys for the collections inside a league
//!
//! This assumes a unique string as the seed for the keys.
//! As a league name is unique it will be used to generate its keys.
//! The keys are necessary for the serialized collections.
//!

use near_sdk::env;

/// The three collection keys needed inside a league
pub struct CollectionKeyTuple {
    players_key: Vec<u8>,
    trusted_key: Vec<u8>,
    matches_key: Vec<u8>,
}

impl CollectionKeyTuple {
    /// Get the key for the players collection
    pub fn get_players_key(&self) -> Vec<u8> {
        self.players_key.clone()
    }

    /// Get the key for the trusted account ids collection
    pub fn get_trusted_key(&self) -> Vec<u8> {
        self.trusted_key.clone()
    }

    /// Get the keys for the game matches collection
    pub fn get_matches_key(&self) -> Vec<u8> {
        self.matches_key.clone()
    }

    /// Create a new key collection tuple from a _unique_ string
    pub fn new(seed: &str) -> Self {
        let mut r = env::sha256(seed.as_bytes());

        // Simply add another byte to the usual sha256 to make three unique but similar keys
        r.push(0);
        let last_index = r.len() - 1;

        // probably could be done cleaner, e.g. with macro but this is fine so far
        //r[last_index] = 0; // is already zero but just to have it complete
        let players_key = r.clone();

        r[last_index] = 1;
        let trusted_key = r.clone();

        r[last_index] = 2;
        let matches_key = r; // last one can be moved instead of a clone

        CollectionKeyTuple {
            players_key,
            trusted_key,
            matches_key,
        }
    }
}
