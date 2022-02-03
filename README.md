# League Contract

A smart contract to save the scores of a league.

The matches in the league are played according to a best-of rule.
So multiple games need to be won to win a whole match.

The game type can be variable. 
Per default the winner will be noted.
Different game types can be included to allow more data to be saved.

## Structure

The contract just contains all the leagues which are uniquely named.
The league consist of a player list and all the matches between two players.
The matches contain multiple games according to the league's ``best_of`` property.

The games can have more information than just the winner depending on the type of the game.
The main logic of the league does not care about the game type 
  and as such the additional data is just forwarded as json to be serialized as borsh.

If the json's type does not match the given league's property, the insertion of the game will fail.
The conversion between json and borsh will be done 
  by serialization/deserialization of an implemented structure.
This guarantees a valid json and also creates a way to use the data in the future.
Maybe an additional view for statistics and data aggregation?

These views would need to be created per game type.

### Calls

The league has only three calls

- Create a league with ``LeagueContract::create_league``
  - Needs a list of trusted account ids who may alter the league
  - Player names in the league
  - League properties 
- Add a game to a league with ``LeagueContract::add_game``
  - Contestant's names
  - The winner
  - Additional game data if the game type wishes it  
- Delete a league with ``LeagueContract::delete_league``
  - Only the owner can delete the league
  - Flag to force deletion if not finished yet  

Additionally, of course, all calls need the league's name.

The idea behind the deleting is mostly to get back resources.
As a todo the creation of a league should be payable 
  which gets refunded when deleted (as a pledge).

### Views

To be implemented

# Todos

## Important

- [x] Write a README.md (maybe a bit earlier the next time)
- [ ] Add more tests
- [x] Add more documentation

## Further ideas

- [ ] Implement Views
- [ ] Add Starcraft as an additional game type
- [ ] Add Views for Starcraft
- [ ] Add a pledge

## More nice to have

- [ ] Deploy the compiled and tested wasm in the CI directly to a testnet account 

## Avoid

- [x] Ruin the git history and be lazy to rebase
- [ ] Ruin it even more
