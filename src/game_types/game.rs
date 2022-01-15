/// The `Game` trait for the contract to be independent of actual game types.
pub trait Game {
    /// Check who won in the game.
    ///
    /// The concrete winner is given by the `GameMatch`.
    /// Here only the winner of the two contestants is needed which is given as the returned bool.
    fn is_first_player_winner(&self) -> bool;

    /// Return the json containing all meta data.
    ///
    /// As the contract shall be independent of the game type it cannot be able to parse the game.
    /// The type shall create its interpretation itself and return it in this method.
    fn get_description_json(&self) -> String;
}
