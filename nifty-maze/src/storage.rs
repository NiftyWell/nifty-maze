multiversx_sc::imports!();
multiversx_sc::derive_imports!();

// Coordinates of the player.
#[derive(TypeAbi, TopEncode, TopDecode, NestedEncode, PartialEq, NestedDecode, Clone)]
pub struct Position
{
    pub row: usize,
    pub col: usize,
}

// Information of a prize in the maze.
#[derive(TypeAbi, TopEncode, TopDecode, NestedEncode, PartialEq, NestedDecode, Clone)]
pub struct PrizeInfo<M: ManagedTypeApi>
{
    pub prize_type: PrizeType,
    pub ticker: EgldOrEsdtTokenIdentifier<M>,
    pub nonce: u64,
    pub quantity: BigUint<M>
}

// Information of a move registered by a player.
#[derive(TypeAbi, TopEncode, TopDecode, NestedEncode, PartialEq, NestedDecode, Clone)]
pub struct PlayerMove<M: ManagedTypeApi>
{
    pub payment_token: EgldOrEsdtTokenIdentifier<M>,
    pub payment_nonce: u64,
    pub payment_amount: BigUint<M>,
    pub player_move: Move,
    pub address: ManagedAddress<M>,
}

// Type of prize.
#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, TypeAbi, PartialEq, Clone)]
pub enum PrizeType {
    None,
    Finish,
    Normal,
}

// Type of blocks in the maze.
#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, TypeAbi, PartialEq, Clone)]
pub enum Block<M: ManagedTypeApi>{
    None,
    Start,
    Finish,
    Wall,
    Trap,
    Prize(PrizeInfo<M>),
    Random,
    Teleport,
    Key(u64),
    Door(u64),
    MatrixDimensions(usize, usize), // Put at the end of the matrix
}

// Possible directions to move.
#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, TypeAbi, PartialEq, Clone)]
pub enum Move {
    None,
    Up,
    Right,
    Down,
    Left
}

// Status of the ongoing game.
#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, TypeAbi, PartialEq, Clone)]
pub enum GameStatus {
    None,
    Start,
    End,
}

// Information of the payment token used to play by users.
#[derive(TypeAbi, TopEncode, TopDecode, NestedEncode, PartialEq, NestedDecode, Clone)]
pub struct PaymentToken<M: ManagedTypeApi>
{
    pub ticker: EgldOrEsdtTokenIdentifier<M>,
    pub nonce: u64,
    pub amount: BigUint<M>
}

// Information of the special prizes to win.
// This information is stored in a storage mapper and is used to track all winners without altering the maze.
// Winner is set to ManagedAddress::zero() until a winner for the prize is selected.
#[derive(TypeAbi, TopEncode, TopDecode, NestedEncode, PartialEq, NestedDecode, Clone)]
pub struct PrizeToWin<M: ManagedTypeApi>
{
    pub ticker: EgldOrEsdtTokenIdentifier<M>,
    pub nonce: u64,
    pub amount: BigUint<M>,
    pub winner: ManagedAddress<M>
}

// Structure used only to get the current mvp information through the get_mvp query
#[derive(TypeAbi, TopEncode, TopDecode, NestedEncode, PartialEq, NestedDecode, Clone)]
pub struct PlayerPayment<M: ManagedTypeApi>
{
    pub address: ManagedAddress<M>,
    pub amount: BigUint<M>
}

// Return types used to get all important information froma single query to the contract.
#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, TypeAbi, PartialEq, Clone)]
pub enum ReturnTypes<M: ManagedTypeApi> {
    None,
    TypePosition(Position),
    TypeGameStatus(GameStatus),
    TypeCooldown(u64),
    TypeTimerStart(u64),
    TypeCollectedTokens(BigUint<M>),
    TypeBigUint(BigUint<M>),
    TypeManagedAddress(ManagedAddress<M>),
    TypePlayerPayment(PlayerPayment<M>),
    TypeMvpClaimed(bool),
    TypeKey(u64),
    TypePrize(PrizeToWin<M>),
    TypePlayerMove(PlayerMove<M>),
}

// Status of the contract.
#[derive(TopEncode, TopDecode, TypeAbi, PartialEq, Clone, Copy, Debug)]
pub enum Status
{
    Frozen,
    Public,
}

#[multiversx_sc::module]
pub trait StorageModule {

    // Reward Tokens
    #[view(getRewardTokens)]
    #[storage_mapper("rewardTokens")]
    fn reward_tokens(&self, prize_type: Block<Self::Api>) -> SingleValueMapper<PrizeInfo<Self::Api>>;

    // Payment Token Ticker
    #[view(getPaymentToken)]
    #[storage_mapper("paymentToken")]
    fn payment_token(&self) -> SingleValueMapper<EgldOrEsdtTokenIdentifier>;

    // Payment Token Quantity
    #[view(getPaymentAmount)]
    #[storage_mapper("paymentAmount")]
    fn payment_amount(&self) -> SingleValueMapper<BigUint>;

    // Player position
    #[view(getPlayerPosition)]
    #[storage_mapper("playerPosition")]
    fn player_position(&self) -> SingleValueMapper<Position>;

    // Start position
    #[view(getStartPosition)]
    #[storage_mapper("startPosition")]
    fn start_position(&self) -> SingleValueMapper<Position>;

    // Matrix: Row starts at 1
    #[view(getMatrix)]
    #[storage_mapper("matrix")]
    fn matrix(&self, row: usize) -> VecMapper<Block<Self::Api>>;

    // Matrix: Row starts at 1
    #[view(getFullMatrix)]
    fn get_full_matrix(&self) -> MultiValueEncoded<Block<Self::Api>> {
        let mut matrix: MultiValueEncoded<Block<Self::Api>> = MultiValueEncoded::new(); 
        let mut row = 1;
        while !self.matrix(row).is_empty() {
            for block in self.matrix(row).iter() {
                matrix.push(block);
            }
            row += 1;
        }
        // We add the matrix dimensions at the end to give more information about the shape of the maze.
        matrix.push(Block::MatrixDimensions(row-1, self.matrix(row-1).len()));
        return matrix;
    }

    // Matrix: Row starts at 1
    #[view(getGameStatus)]
    #[storage_mapper("gameStatus")]
    fn game_status(&self) -> SingleValueMapper<GameStatus>;

    // Collected keys
    #[view(getKeys)]
    #[storage_mapper("collectedKeysId")]
    fn collected_keys_id(&self) -> UnorderedSetMapper<u64>;
    
    // Amount of payment tokens collected.
    #[view(getCollectedTokens)]
    #[storage_mapper("collectedTokens")]
    fn collected_tokens(&self) -> SingleValueMapper<BigUint>;

    // Total amount paid by a player during this game session.
    #[view(getPlayerPayments)]
    #[storage_mapper("playerPayments")]
    fn player_payments(&self, address: &ManagedAddress) -> SingleValueMapper<BigUint>;

    // Percent applied to the MVP rewards.
    #[view(getMvpPercent)]
    #[storage_mapper("mvpPercent")]
    fn mvp_percent(&self) -> SingleValueMapper<BigUint>;

    // Players mapper is used to get all players that have played at least once during the whole game session.
    #[view(getPlayers)]
    #[storage_mapper("players")]
    fn players(&self) -> UnorderedSetMapper<ManagedAddress>;

    // Get information about the payment token used to play the game.
    #[view(getPaymentInfo)]
    fn get_payment_info(&self) -> PaymentToken<Self::Api> {
        PaymentToken{
            ticker: self.payment_token().get(),
            nonce: 0,
            amount: self.payment_amount().get() 
        }
    }

    // Flag to know if the MVP claimed the rewards.
    #[view(getMvpClaimed)]
    #[storage_mapper("mvpClaimed")]
    fn mvp_claimed(&self) -> SingleValueMapper<bool>;

    // Information of the special prizes to win.
    // This information is stored in a storage mapper and is used to track all winners without altering the maze.
    // Winner is set to ManagedAddress::zero() until a winner for the prize is selected.
    #[view(getPrizesToWin)]
    #[storage_mapper("prizesToWin")]
    fn prizes_to_win(&self) -> UnorderedSetMapper<PrizeToWin<Self::Api>>;

    // Contract Status
    #[view(getStatus)]
    #[storage_mapper("status")]
    fn status(&self) -> SingleValueMapper<Status>;

    // Get the address that has spent the most payment ESDT this game session.
    #[view(getMvp)]
    fn get_mvp(&self) -> PlayerPayment<Self::Api> {
        let mut address = ManagedAddress::zero();
        let mut amount = BigUint::zero();
        for player in self.players().iter() {
            let payment = self.player_payments(&player).get();
            if payment > amount {
                amount = payment;
                address = player
            }
        }
        PlayerPayment {
            address: address,
            amount: amount,
        }
    }

    // View returning general data about the contract in a MultiValueEncoded structure.
    #[view(getGeneralData)]
    fn get_general_data(&self) -> MultiValueEncoded<ReturnTypes<Self::Api>> {
        let mut my_vec: MultiValueEncoded<ReturnTypes<Self::Api>> = MultiValueEncoded::new();
        my_vec.push(ReturnTypes::TypePosition(self.player_position().get()));
        my_vec.push(ReturnTypes::TypeGameStatus(self.game_status().get()));

        my_vec.push(ReturnTypes::TypeCooldown(self.duration().get()));
        my_vec.push(ReturnTypes::TypeTimerStart(self.timer().get())); // timestamp when clock starts.
        my_vec.push(ReturnTypes::TypeCollectedTokens(self.collected_tokens().get()));
        for key in self.collected_keys_id().iter() {
            my_vec.push(ReturnTypes::TypeKey(key));
        }
        for prize in self.prizes_to_win().iter() {
            my_vec.push(ReturnTypes::TypePrize(prize));
        }
        for single_move in self.moves().iter() {
            my_vec.push(ReturnTypes::TypePlayerMove(single_move));
        }
        my_vec.push(ReturnTypes::TypeMvpClaimed(self.mvp_claimed().get()));
        return my_vec;
    }    


    // Timer used to see how much time passed with our clock.
    #[view(getTimer)]
    #[storage_mapper("timer")]
    fn timer(&self) -> SingleValueMapper<u64>;

    // Duration of the clock (lower bound)
    #[view(getDuration)]
    #[storage_mapper("duration")]
    fn duration(&self) -> SingleValueMapper<u64>;

    // All player registered moves during a round.
    #[view(getMoves)]
    #[storage_mapper("moves")]
    fn moves(&self) -> UnorderedSetMapper<PlayerMove<Self::Api>>;

    // Can make move flag (true when the clock starts).
    #[view(getCanMakeMove)]
    #[storage_mapper("canmakemove")]
    fn can_make_move(&self) -> SingleValueMapper<bool>;

    // Address of the pong contract used to create the clock.
    #[view(getPongAddress)]
    #[storage_mapper("pongaddress")]
    fn pong_address(&self) -> SingleValueMapper<ManagedAddress>;

    // List of all addresses with moves during this game round.
    #[view(getAddressesWithMove)]
    #[storage_mapper("addressesWithMove")]
    fn addresses_with_move(&self) -> UnorderedSetMapper<ManagedAddress>;
}