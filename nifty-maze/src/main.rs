#![no_std]
multiversx_sc::imports!();
multiversx_sc::derive_imports!();

// Here the multiplier is hardcoded for 18 decimal tokens, like the native token EGLD and the NiftyBit token.
pub const MULTIPLIER_TOTAL: u64 = 1000000000000000000;

pub mod storage;
pub mod errors;
pub mod setup;

use errors::{
    ERR_CONTRACT_PAUSED,
    ERR_GAME_ENDED,
    ERR_PAYMENT_IDENTIFIER,
    ERR_PAYMENT_NONCE,
    ERR_PAYMENT_AMOUNT,

    ERR_GO_THROUGH_WALL,
    ERR_KEY_NEEDED,
    ERR_JUMP_OVER_TRAP,

    ERR_GAME_ONGOING,
    ERR_MVP_CLAIMED,
    ERR_NOT_MVP,
};

use storage::{
    Position,
    PrizeType,
    PrizeInfo,
    PrizeToWin,
    Block,
    Move,
    GameStatus,
    Status,
    PlayerMove,
};

#[multiversx_sc::contract]
pub trait Main: 
    storage::StorageModule
    + setup::SetupModule
    + multiversx_sc_modules::pause::PauseModule
{
    #[proxy]
    fn ping_proxy(&self, sc_address: ManagedAddress) -> ping_proxy::Proxy<Self::Api>;

    #[init]
    fn init(&self) {
    }

    #[endpoint(ping)]
    fn ping(&self) -> ManagedBuffer {
        let now = self.blockchain().get_block_timestamp();

        // New round of moves to register
        self.timer().set(now);
        self.can_make_move().set(true);

        // The duration lower bound has been exceeded.
        if now - self.timer().get() >= self.duration().get() {
            self.timer().clear();
            self.can_make_move().set(false);

            let winning_address = self.pick_move();
            self.addresses_with_move().clear();
            self.moves().clear();

            let mut response = ManagedBuffer::new();
            response.append(&winning_address.as_managed_buffer());
            return response;
        }

        self.ping_proxy(self.pong_address().get())
            .pong()
            .async_call()
            .call_and_exit();
    }

    #[payable("*")]
    #[endpoint(addMove)]
    #[allow(clippy::too_many_arguments)]
    fn add_move(
        &self,
        #[payment_token] payment_token  : EgldOrEsdtTokenIdentifier,
        #[payment_nonce] payment_nonce  : u64,
        #[payment_amount] payment_amount: BigUint,
        player_move: Move,
    ) {
        let caller = self.blockchain().get_caller();

        // Check contract status.
        require!(
            self.status().get() == Status::Public,
            ERR_CONTRACT_PAUSED
        );
        // Check game session status.
        require!(
            self.game_status().get() != GameStatus::End, 
            ERR_GAME_ENDED
        );

        // Check payment token.
        require!(
            payment_token == self.payment_token().get(), 
            ERR_PAYMENT_IDENTIFIER
        );
        require!(
            payment_nonce == 0,
            ERR_PAYMENT_NONCE
        );

        // Amount can vary depending on number of jumps in the move.
        // Amount needs to be a multiple of the payment amount set in the contract.
        require!(
            payment_amount.clone() % self.payment_amount().get() == 0 && payment_amount.clone() != BigUint::zero(),
            ERR_PAYMENT_AMOUNT
        );

        // payment_amount is a multiple of the one in memory, so jump size will always be a positive integer.
        let jump_size = (payment_amount.clone()/self.payment_amount().get()).to_u64().unwrap() as usize;
        
        // Check move validity
        self.check_move(jump_size, player_move.clone());

        // If needed, remove the old move registered.
        if self.addresses_with_move().contains(&caller) {
            for p_move in self.moves().iter() {
                if p_move.address == caller.clone() {
                    self.moves().swap_remove(&p_move);
                }
            }
        }

        // Insert the new player move in this game's round registered moves.
        self.moves().insert(
            PlayerMove {
                payment_token: payment_token,
                payment_nonce: payment_nonce,
                payment_amount: payment_amount.clone(),
                player_move: player_move,
                address: caller.clone(),
            }
        );

        // Add address to addresses with move for this game round.
        if !self.addresses_with_move().contains(&caller) {
            self.addresses_with_move().insert(caller.clone());
        }

        // Add player for this entire game session.
        if !self.players().contains(&caller) {
            self.players().insert(caller.clone());
        }

        // Add payment amount the the caller's total payments for this game session.
        self.player_payments(&caller).update(|val| *val += payment_amount.clone());
        // Add to the total amount collected for this game session.
        self.collected_tokens().update(|val| *val += payment_amount);

        // If this is the first move registered in this game round.
        if !self.can_make_move().get() {
            self.ping();
        }
    }

    fn check_move(&self, jump_size: usize, player_move: Move) {
        // Check for obstacles on the way
        for jump in 1..=jump_size {
            // Get the position of the player if the player move is applied, block by block until the player jumps "jump_size" blocks.
            let new_move = self.get_new_position(player_move.clone(), jump);

            // Get each block on the way.
            let block = self.matrix(new_move.row).get(new_move.col);
            
            match block {
                Block::Wall => {
                    require!(
                        block != Block::Wall, 
                        ERR_GO_THROUGH_WALL
                    );
                    break; // Stop checking further moves if there's a wall
                },
                Block::Door(door_id) => {
                    require!(
                        self.collected_keys_id().contains(&door_id), 
                        ERR_KEY_NEEDED
                    );
                    break; // Stop checking further moves if there's a door
                },
                Block::Trap => {
                    require!(
                        jump_size <= 1, 
                        ERR_JUMP_OVER_TRAP
                    );
                    break; // Stop checking further moves if there's a trap
                },
                _ => (), // Normal block or prize, just advance.
            }
        }
    }

    // Function used to get the new position in the matrix if "player_move" and "jump" are applied.
    fn get_new_position(
        &self,
        player_move: Move,
        jump: usize,
    ) -> Position {
        let current_position = self.player_position().get();

        if player_move == Move::Up {
            return Position{row: current_position.row-jump, col: current_position.col};
        }
        else if player_move == Move::Right {
            return Position{row: current_position.row, col: current_position.col+jump};
        }
        else if player_move == Move::Down {
            return Position{row: current_position.row+jump, col: current_position.col};
        }
        else {
            return Position{row: current_position.row, col: current_position.col-jump};
        }
    }

    // Function used to pick a random winning move through all registered moves in the moves() mapper.
    fn pick_move(&self) -> ManagedAddress {
        let mut rand_source = RandomnessSource::new();
        let rand_index = rand_source.next_usize_in_range(1, self.moves().len()+1);
        let winning_move = self.moves().get_by_index(rand_index);
        self.set_new_position(winning_move.clone());
        self.check_block(winning_move.address.clone());
        return winning_move.address;
    }

    // Function used to set the new player position.
    fn set_new_position(
        &self,
        player_move: PlayerMove<Self::Api>,
    ) 
    {
        // Set the player coordinates to the new correct coordinates.
        let movement = player_move.player_move;
        let current_position = self.player_position().get();
        let jump = (player_move.payment_amount/self.payment_amount().get()).to_u64().unwrap() as usize;
        if movement == Move::Up {
            self.player_position().set(Position{row: current_position.row-jump, col: current_position.col});
        }
        else if movement == Move::Right {
            self.player_position().set(Position{row: current_position.row, col: current_position.col+jump});
        }
        else if movement == Move::Down {
            self.player_position().set(Position{row: current_position.row+jump, col: current_position.col});
        }
        else if movement == Move::Left {
            self.player_position().set(Position{row: current_position.row, col: current_position.col-jump});
        }
    }

    // Function used to check and handle the current block.
    fn check_block(
        &self,
        caller: ManagedAddress
    ) {
        let current_position = self.player_position().get();
        let block = self.matrix(current_position.row).get(current_position.col);
        // At this point player position should never be a Wall.
        require!(
            block != Block::Wall, 
            ERR_GO_THROUGH_WALL
        );
        match block {
            Block::Prize(prize_info) => self.handle_reward(prize_info, caller),
            Block::Trap => self.handle_trap(),
            Block::Door(door_id) => self.handle_door(door_id),
            Block::Key(key_id) => self.handle_key(key_id),
            //Block::Random => self.handle_random(),
            //Block::Teleport => self.handle_teleport(),
            _=> (), // Normal block, just advance.
        }
    }

    // Function used to handle move on a door block.
    fn handle_door(
        &self,
        door_id: u64
    ) {
        require!(
            self.collected_keys_id().contains(&door_id), 
            ERR_KEY_NEEDED
        );
    }

    // Function used to handle move on a key block.
    fn handle_key(
        &self,
        key_id: u64
    ) {
        let current_position = self.player_position().get();
        self.matrix(current_position.row).set(current_position.col, &Block::None);
        if !self.collected_keys_id().contains(&key_id) {
            self.collected_keys_id().insert(key_id);
        }
    }

    // Function used to handle move on a trap block.
    fn handle_trap(
        &self
    ) {
        // Player position is reset to the start position.
        self.player_position().set(self.start_position().get());
    }

    // Function used to handle move on a prize block.
    fn handle_reward(
        &self,
        prize: PrizeInfo<Self::Api>,
        caller: ManagedAddress,
    ) {
        // Give reward then clear reward case.
        if prize.prize_type == PrizeType::Finish {
            self.game_status().set(GameStatus::End);
        } 
        let position = self.player_position().get();
        self.matrix(position.row).set(position.col, &Block::None);

        // Remove old prize entry.
        self.prizes_to_win().swap_remove(&PrizeToWin{
            ticker: prize.ticker.clone(),
            nonce: prize.nonce,
            amount: prize.quantity.clone(),
            winner: ManagedAddress::zero()  // Assuming winner is zero address initially.
        });

        // Enter new prize entry with winner address.
        self.prizes_to_win().insert(PrizeToWin{
            ticker: prize.ticker.clone(),
            nonce: prize.nonce,
            amount: prize.quantity.clone(),
            winner: caller.clone(),
        });

        // Send reward to winner.
        self.send()
                .direct(&caller, &prize.ticker, prize.nonce, &prize.quantity);
    }

    // Function used by the MVP to claim the rewards share.
    #[endpoint(mvpClaim)]
    fn mvp_claim(&self) -> SCResult<()>{

        // MVP rewards must not be already claimed.
        require!(
            self.mvp_claimed().get() == false, 
            ERR_MVP_CLAIMED
        );

        // The whole game session must have ended.
        require!(
            self.game_status().get() == GameStatus::End, 
            ERR_GAME_ONGOING
        );

        let caller = self.blockchain().get_caller();
        let mvp_info = self.get_mvp();

        // Caller must be the MVP.
        require!(
            caller == mvp_info.address, 
            ERR_NOT_MVP
        );
        // Send the share to the MVP.
        let amount = self.collected_tokens().get() * self.mvp_percent().get() / MULTIPLIER_TOTAL;
        self.send()
                .direct(&caller, &self.payment_token().get(), 0, &amount);
        self.mvp_claimed().set(true);
        Ok(())
    }
}
