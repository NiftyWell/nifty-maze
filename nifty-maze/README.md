# MultiversX Nifty Maze Smart Contract Documentation

The Nifty Maze smart contract is a game contract based on the Rust programming language for the MultiversX blockchain, formerly known as the Elrond blockchain. It allows players to navigate through a maze and win prizes along the way. The contract provides functionalities for setting up the maze, registering player moves, handling different types of blocks, and distributing rewards to winners.

The smart contract introduces a "clock" made by sending transactions to an auxiliary smart contract on another shard.
The contract checks if a minimal amount of time has passed between call cycles before stopping the clock.
This clock is used to allow users to register moves in the move pool, and automatically select a winner after a certain amount of time.

By setting the duration mapper to 40 seconds, the total clock will follow a cycle of aproximately 72. 
Users have ~52 out of the 72 seconds to register moves before a winner is automatically picked.

## Data Structures

### Position
- `struct Position`: Represents the coordinates of a player in the maze.
- Fields:
  - `row`: Row index of the player's position.
  - `col`: Column index of the player's position.

### PrizeInfo
- `struct PrizeInfo<M: ManagedTypeApi>`: Represents information about a prize in the maze.
- Fields:
  - `prize_type`: Type of the prize (None, Finish, or Normal).
  - `ticker`: Token identifier for the prize.
  - `nonce`: Nonce value for the prize.
  - `quantity`: Quantity of the prize in the form of a BigUint.

### PlayerMove
- `struct PlayerMove<M: ManagedTypeApi>`: Represents information of a move registered by a player.
- Fields:
  - `payment_token`: Payment token used by the player for the move.
  - `payment_nonce`: Nonce value for the payment token.
  - `payment_amount`: Amount of the payment token.
  - `player_move`: Direction of the player's move (Up, Right, Down, Left).
  - `address`: Address of the player.

### PrizeType
- `enum PrizeType`: Represents the type of prize block (None, Finish, or Normal).

### Block
- `enum Block<M: ManagedTypeApi>`: Represents the type of blocks in the maze.
- Possible Variants:
  - `None`: Empty block.
  - `Start`: Start block.
  - `Finish`: Finish block.
  - `Wall`: Wall block (obstacle).
  - `Trap`: Trap block (negative effect).
  - `Prize(PrizeInfo<M>)`: Prize block with associated PrizeInfo.
  - `Random`: Random block (not implemented).
  - `Teleport`: Teleport block (not implemented).
  - `Key(u64)`: Key block with associated key ID.
  - `Door(u64)`: Door block with associated door ID.
  - `MatrixDimensions(usize, usize)`: Special block used at the end of the matrix to provide matrix dimensions.

### Move
- `enum Move`: Represents possible directions to move in the maze (None, Up, Right, Down, Left).

### GameStatus
- `enum GameStatus`: Represents the status of the ongoing game.
- Possible Variants:
  - `None`: Initial status.
  - `Start`: Game started and moves can be registered.
  - `End`: Game ended.

### Status
- `enum Status`: Represents the status of the contract (Frozen or Public).

### PaymentToken
- `struct PaymentToken<M: ManagedTypeApi>`: Represents information of the payment token used to play the game.
- Fields:
  - `ticker`: Payment token identifier.
  - `nonce`: Nonce value for the payment token.
  - `amount`: Amount of the payment token.

### PrizeToWin
- `struct PrizeToWin<M: ManagedTypeApi>`: Represents information of the special prizes to win.
- Fields:
  - `ticker`: Token identifier for the prize.
  - `nonce`: Nonce value for the prize.
  - `amount`: Quantity of the prize in the form of a BigUint.
  - `winner`: Address of the winner for the prize (initially set to ManagedAddress::zero()).

### PlayerPayment
- `struct PlayerPayment<M: ManagedTypeApi>`: Structure used to get the current MVP information through the get_mvp query.
- Fields:
  - `address`: Address of the player.
  - `amount`: Amount spent by the player during the game session.

### ReturnTypes
- `enum ReturnTypes<M: ManagedTypeApi>`: Represents return types used to get all important information from a single query to the contract.

## Modules

### StorageModule
- `trait StorageModule`: Defines storage mappers and views for managing the state of the contract.
- Provides functions for accessing and updating various contract data, such as player positions, matrix blocks, game status, player moves, and MVP rewards.

### SetupModule
- `trait SetupModule`: Defines functions used for setting up the game, registering tokens, initializing the maze matrix, and managing the game status.
- Allows contract owner to register payment tokens, load/unload tokens, set the maze matrix, clear the matrix, set game status, set MVP percentage, initialize player positions, and set the contract status.

### Main
- `trait Main`: Contains the main logic of the MultiversX smart contract.
- Inherits from `StorageModule`, `SetupModule`, and `multiversx_sc_modules::pause::PauseModule`.
- Provides functions for player moves, handling different types of blocks, picking a winning move, and handling the MVP rewards claim.

## Constants

- `MULTIPLIER_TOTAL`: The multiplier used for 18 decimal tokens, like the native token EGLD and the NiftyBit token.

## Functions

### init
- `init(&self)`: Initialization function for the smart contract. No arguments are required.

### ping
- `ping(&self) -> ManagedBuffer`: Function used to start a new round of moves to register. Also, handles the end of the game and triggers the reward distribution.
  - Returns: A `ManagedBuffer` containing the address of the winning player for the current round (if applicable).

### add_move
- `add_move(&self, payment_token: EgldOrEsdtTokenIdentifier, payment_nonce: u64, payment_amount: BigUint, player_move: Move)`: Function used to register a player's move in the maze.
  - `payment_token`: Payment token identifier used by the player for the move.
  - `payment_nonce`: Nonce value for the payment token.
  - `payment_amount`: Amount of the payment token for the move.
  - `player_move`: Direction of the player's move (Up, Right, Down, Left).

### mvp_claim
- `mvp_claim(&self) -> SCResult<()>`: Function used by the MVP to claim the rewards share.
  - Returns: An `SCResult` indicating the success or failure of the MVP rewards claim.

### Other Functions
- The contract also contains several helper functions for checking move validity, handling different types of blocks, and managing player positions and rewards.

## Events
- The contract does not define custom events in the provided code.

## Error Messages

- The contract defines several custom error messages to indicate different scenarios during move registration and rewards claim.

