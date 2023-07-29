multiversx_sc::imports!();
multiversx_sc::derive_imports!();

use crate::storage::{
    Block,
    Position,
    GameStatus,
    PrizeToWin,
    Status,
};

#[multiversx_sc::module]
pub trait SetupModule:
    crate::storage::StorageModule 
{
    // Function used to register the ESDT used as payment by users to take part in the game.
    #[only_owner]
    #[endpoint(registerPaymentToken)]
    #[allow(clippy::too_many_arguments)]
    fn register_payment_token(
        &self,
        token: EgldOrEsdtTokenIdentifier,
        quantity: BigUint,
    ) -> SCResult<()> {
        self.payment_token().set(&token);
        self.payment_amount().set(&quantity);
        Ok(())
    }

    // Used to load the contract with NFTs & ESDTs.
    #[only_owner]
    #[payable("*")]
    #[endpoint(loadTokens)]
    #[allow(clippy::too_many_arguments)]
    fn load_tokens(
        &self,
    ) -> SCResult<()> {
        Ok(())
    }

    // Used to unload NFTs & ESDTs from the contract.
    #[only_owner]
    #[endpoint(unloadTokens)]
    #[allow(clippy::too_many_arguments)]
    fn unload_tokens(
        &self,
        ticker: EgldOrEsdtTokenIdentifier,
        nonce: u64,
        amount: BigUint,
    ) -> SCResult<()> {
        let caller = self.blockchain().get_caller();
        self.send()
                .direct(&caller, &ticker, nonce, &amount);
        Ok(())
    }

    // Function used to register the maze matrix.
    #[only_owner]
    #[endpoint(setMatrix)]
    #[allow(clippy::too_many_arguments)]
    fn set_matrix(
        &self,
        args: MultiValueEncoded<MultiValue3<usize, usize, Block<Self::Api>>>
    ) -> SCResult<()> {
        self.game_status().set(GameStatus::Start);
        self.set_can_make_move();
        let _ = self.init_position(2, 2);
        for triple in args.into_iter(){
            let tuple = triple.into_tuple();
            let row = tuple.0;
            let block = tuple.2;
            self.matrix(row).push(&block);
            match block {
                // Register prize block in memory.
                Block::Prize(prize_info) => {
                    let prize_to_win = PrizeToWin {
                        ticker: prize_info.ticker,
                        nonce: prize_info.nonce,
                        amount: prize_info.quantity,
                        winner: ManagedAddress::zero()  // Assuming winner is zero address initially.
                    };
                    self.prizes_to_win().insert(prize_to_win);
                },
                _ => {} // Default case to handle all other block types
            }        
        }
        Ok(())
    }

    // Clear maze matrix and reset the game.
    #[only_owner]
    #[endpoint(clearMatrix)]
    #[allow(clippy::too_many_arguments)]
    fn clear_matrix(
        &self,
    ) -> SCResult<()> {
        self.moves().clear();
        self.timer().set(0);
        self.mvp_claimed().set(false);
        self.collected_tokens().clear();
        self.prizes_to_win().clear();
        for player in self.players().iter() {
            self.player_payments(&player).clear();
        }
        self.collected_keys_id().clear();
        let mut row = 1;
        while !self.matrix(row).is_empty() {
            self.matrix(row).clear();
            row += 1;
        }
        Ok(())
    }

    #[only_owner]
    #[endpoint(setGameStatus)]
    #[allow(clippy::too_many_arguments)]
    fn set_game_status(
        &self,
        status: GameStatus,
    ) -> SCResult<()> {
        self.game_status().set(status);
        Ok(())
    }

    #[only_owner]
    #[endpoint(setMvpPercent)]
    #[allow(clippy::too_many_arguments)]
    fn set_mvp_percent(
        &self,
        percent: BigUint,
    ) -> SCResult<()> {
        self.mvp_percent().set(percent);
        Ok(())
    }

    // Function used to initialize the starting position of the player and the actual position of the player.
    #[only_owner]
    #[endpoint(setInitPosition)]
    #[allow(clippy::too_many_arguments)]
    fn init_position(
        &self,
        row: usize,
        col: usize,
    ) -> SCResult<()> {
        self.player_position().set(Position{row: row, col: col});
        self.start_position().set(Position{row: row, col: col});
        Ok(())
    }

    // Function used to set the pong contract address used for the "clock".
    #[only_owner]
    #[endpoint(setPongAddress)]
    #[allow(clippy::too_many_arguments)]
    fn set_pong_address(
        &self,
        pong_address: ManagedAddress,
    ) -> SCResult<()> {
        self.pong_address().set(pong_address);
        Ok(())
    }

    // Function used to only set the starting position.
    // This is the position to which the player gets reset when walking into a trap.
    #[only_owner]
    #[endpoint(setStartPosition)]
    #[allow(clippy::too_many_arguments)]
    fn set_start_position(
        &self,
        row: usize,
        col: usize,
    ) -> SCResult<()> {
        self.start_position().set(Position{row: row, col: col});
        Ok(())
    }

    // Function used to set the player's active position.
    #[only_owner]
    #[endpoint(setPlayerPosition)]
    #[allow(clippy::too_many_arguments)]
    fn set_player_position(
        &self,
        row: usize,
        col: usize,
    ) -> SCResult<()> {
        self.player_position().set(Position{row: row, col: col});
        Ok(())
    }

    // Function used to initialize the duration of the clock (lower bound).
    #[only_owner]
    #[endpoint(setDuration)]
    #[allow(clippy::too_many_arguments)]
    fn set_duration(
        &self,
        duration: u64,
    ) -> SCResult<()> {
        self.duration().set(duration);
        Ok(())
    }
    
    #[only_owner]
    #[endpoint(setFirstMoveTimestamp)]
    #[allow(clippy::too_many_arguments)]
    fn set_timer(
        &self,
        start_timer: u64,
    ) -> SCResult<()> {
        self.timer().set(start_timer);
        Ok(())
    }

    // Set contract status.
    // This status only influences the addMove function.
    #[only_owner]
    #[endpoint(setStatus)]
    fn set_status(&self, status: Status)
    {
        self.status().set(status);
    }

    // Set the can make move mapper.
    // This mapper is used to know if the clock is active and users can register moves.
    #[only_owner]
    #[endpoint(setCanMakeMove)]
    fn set_can_make_move(&self)
    {
        self.can_make_move().set(false);
    }
}
