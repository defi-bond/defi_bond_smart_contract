//! State transition types


/// Imports
/// ------------------------------------------------------------------------------------------------

use {
    borsh::{BorshDeserialize, BorshSchema, BorshSerialize},
    solana_program::pubkey::Pubkey,
    strum_macros::AsRefStr,
};


/// Account Type
/// -------------------------------------------------------------------------------------------------

/// The types of program derived addresses managed by the Bond program.
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize, BorshSchema)]
pub enum BondAccountType {

    /// The type given to a new account that has not been initialized.
    Uninitialized,
    
    /// The game's configurations/settings.
    Config,

    /// The game's current state.
    State,
    
    /// A share in the staking rewards (e.g. jackpot).
    Share,

    /// Stake pool's fee account.
    Fee,

    /// A draw result.
    Draw,

    /// Accounts excluded from winning the draw.
    ExclusionList,
}

impl Default for BondAccountType {
    fn default() -> Self {
        BondAccountType::Uninitialized
    }
}


/// Bond Seed
/// -------------------------------------------------------------------------------------------------

/// The seeds of program derived addresses managed by the Bond program.
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize, BorshSchema, AsRefStr)]
pub enum BondSeed {

    /// The current state.
    #[strum(serialize = "state")]
    State,

    /// The Stake Pool's epoch fee.
    #[strum(serialize = "fee")]
    Fee,

    /// The latest draw result.
    #[strum(serialize = "draw")]
    Draw,
    
    /// Accounts excluded from winning the draw (e.g. program or partner accounts).
    #[strum(serialize = "exclusionList")]
    ExclusionList,

    /// The creator's share.
    #[strum(serialize = "equity")]
    Equity,

    /// The treasury account.
    #[strum(serialize = "treasury")]
    Treasury,

    /// The jackpot.
    #[strum(serialize = "jackpot")]
    Jackpot,

    /// The game's stake (locked).
    #[strum(serialize = "stake")]
    Stake,
}


/// Bond Account
/// ------------------------------------------------------------------------------------------------

/// A Bond account (implemented by all account).
pub trait BondAccount {

    /// True if the account has been initialized.
    fn is_initialized(&self) -> bool;

    /// True if the account has been initialized with the expected [BondAccountType].
    fn is_valid(&self) -> bool;
}


/// Bond Program Account
/// ------------------------------------------------------------------------------------------------

/// A Bond account owned by the program.
pub trait BondProgramAccount: BondAccount {

    /// The account authorized to modify this account.
    fn authority(&self) -> Pubkey;
}


/// Bond Program Derived Account
/// ------------------------------------------------------------------------------------------------

/// A Bond account owned by the program with a derived address.
pub trait BondProgramDerivedAccount: BondProgramAccount {

    /// The derived account's bump seed.
    fn bump(&self) -> u8;
}


/// Bond Config
/// ------------------------------------------------------------------------------------------------

/// The configurations and settings.
#[repr(C)]
#[derive(Clone, Debug, Default, PartialEq, BorshDeserialize, BorshSerialize, BorshSchema)]
pub struct BondConfig {

    /// [BondAccountType::Config].
    pub account_type: BondAccountType,
    
    /// Whether or not the game is active.
    pub is_active: bool,

    /// The minimum number of epochs required between draws.
    pub epochs_per_draw: u8,

    /// The maximum number of consecutive rollovers.
    pub max_rollover: u8,

    /// The maximum odds of a single account expressed by the percentage (0-100)
    /// `(odds_threshold_numerator/odds_threshold_denominator)*100`.
    pub odds_threshold_numerator: u32,

    /// The maximum odds of a single account expressed by the percentage (0-100)
    /// `(odds_threshold_numerator/odds_threshold_denominator)*100`.
    pub odds_threshold_denominator: u32,

    /// The account authorized to run a draw.
    pub draw_authority: Pubkey,

    /// The Stake Pool token's mint address.
    pub token_mint: Pubkey,
}

impl BondAccount for BondConfig {

    /// True if `account_type` is not [BondAccountType::Uninitialized].
    fn is_initialized(&self) -> bool { 
        self.account_type != BondAccountType::Uninitialized 
    }

    /// True if `account_type` is [BondAccountType::Config].
    fn is_valid(&self) -> bool { 
        self.account_type == BondAccountType::Config 
    }
}

impl BondConfig {

    /// Creates a new instance of [BondConfig] with an `account_type` of 
    /// [BondAccountType::Config].
    pub fn new(
        is_active: bool,
        epochs_per_draw: u8,
        max_rollover: u8,
        odds_threshold_numerator: u32,
        odds_threshold_denominator: u32,
        draw_authority: Pubkey,
        token_mint: Pubkey,
    ) -> Self {
        Self { 
            account_type: BondAccountType::Config,
            is_active,
            epochs_per_draw,
            max_rollover,
            odds_threshold_numerator,
            odds_threshold_denominator,
            draw_authority,
            token_mint,
        }
    }
}


/// Bond State
/// ------------------------------------------------------------------------------------------------

/// The current state of the game.
#[repr(C)]
#[derive(Clone, Debug, Default, PartialEq, BorshDeserialize, BorshSerialize, BorshSchema)]
pub struct BondState {

    /// [BondAccountType::State].
    pub account_type: BondAccountType,
    
    /// The account authorized to modify this account.
    pub authority: Pubkey,

    /// The derived account's bump seed.
    pub bump: u8,

    /// The latest draw result id.
    pub draw_id: u64,

    /// The number of consecutive rollovers.
    pub rollover: u8,
}

impl BondAccount for BondState {

    /// True if `account_type` is not [BondAccountType::Uninitialized].
    fn is_initialized(&self) -> bool { 
        self.account_type != BondAccountType::Uninitialized 
    }

    /// True if `account_type` is [BondAccountType::State].
    fn is_valid(&self) -> bool { 
        self.account_type == BondAccountType::State 
    }
}

impl BondProgramAccount for BondState {
    fn authority(&self) -> Pubkey {
        self.authority
    }
}

impl BondProgramDerivedAccount for BondState {
    fn bump(&self) -> u8 {
        self.bump
    }
}

impl BondState {

    /// Creates a new instance of [BondState] with an `account_type` of 
    /// [BondAccountType::State].
    pub fn new(
        authority: Pubkey,
        bump: u8,
        draw_id: u64,
        rollover: u8,
    ) -> Self {
        Self { 
            account_type: BondAccountType::State,
            authority,
            bump,
            draw_id,
            rollover,
        }
    }
}


/// Bond Share
/// ------------------------------------------------------------------------------------------------

/// An account that receives a share of the stake pool's rewards ([BondFee]).
#[repr(C)]
#[derive(Clone, Debug, Default, PartialEq, BorshDeserialize, BorshSerialize, BorshSchema)]
pub struct BondShare {

    /// [BondAccountType::Share].
    pub account_type: BondAccountType,

    /// The account authorized to modify this account.
    pub authority: Pubkey,

    /// The derived account's bump seed.
    pub bump: u8,

    /// The rewards share expressed by the percentage (0-100) `(numerator/denominator)*100`.
    pub numerator: u32,

    /// The rewards share expressed by the percentage (0-100) `(numerator/denominator)*100`.
    pub denominator: u32,
}

impl BondAccount for BondShare {

    /// True if `account_type` is not [BondAccountType::Uninitialized].
    fn is_initialized(&self) -> bool {
        self.account_type != BondAccountType::Uninitialized
    }

    /// True if `account_type` is [BondAccountType::Share].
    fn is_valid(&self) -> bool {
        self.account_type == BondAccountType::Share
    }
}

impl BondProgramAccount for BondShare {
    fn authority(&self) -> Pubkey {
        self.authority
    }
}

impl BondProgramDerivedAccount for BondShare {
    fn bump(&self) -> u8 {
        self.bump
    }
}

impl BondShare {

    /// Creates a new instance of [BondShare] with an `account_type` of [BondAccountType::Share].
    pub fn new(
        authority: Pubkey, 
        bump: u8, 
        numerator: u32,
        denominator: u32,
    ) -> Self {
        Self { 
            account_type: BondAccountType::Share, 
            authority, 
            bump,
            numerator,
            denominator, 
        }
    }

    pub fn share(&self, amount: u64) -> u64 {
        let denominator = u64::from(self.denominator);
        if denominator == 0 { 0 } else { (amount * u64::from(self.numerator)) / denominator }
    }
}


/// Bond Draw
/// ------------------------------------------------------------------------------------------------

/// A draw result.
#[repr(C)]
#[derive(Clone, Debug, Default, PartialEq, BorshDeserialize, BorshSerialize, BorshSchema)]
pub struct BondDraw {

    /// [BondAccountType::Draw].
    pub account_type: BondAccountType,

    /// The account authorized to modify this account.
    pub authority: Pubkey,

    /// Unique id / sequence number.
    pub id: u64,

    /// The amount in lamports.
    pub amount: u64,

    /// The randomly generated value used to select the winner.
    pub receiver_seed: u64,

    /// The winning account.
    pub receiver: Pubkey,

    /// The rollover count at the time of this draw.
    pub rollover: u8,

    /// The network/bank slot at which the draw took place.
    pub slot: u64,

    /// The timestamp of the first slot in `epoch`.
    pub epoch_start_timestamp: i64,

    /// The bank Epoch at which the draw took place.
    pub epoch: u64,

    /// The timestamp at which the draw took place.
    pub unix_timestamp: i64,
}

impl BondAccount for BondDraw {
    
    /// True if `account_type` is not [BondAccountType::Uninitialized].
    fn is_initialized(&self) -> bool {
        self.account_type != BondAccountType::Uninitialized
    }

    /// True if `account_type` is [BondAccountType::Draw].
    fn is_valid(&self) -> bool {
        self.account_type == BondAccountType::Draw
    }
}

impl BondProgramAccount for BondDraw {
    fn authority(&self) -> Pubkey {
        self.authority
    }
}

impl BondDraw {

    /// Creates a new instance of [BondDraw] with an `account_type` of [BondAccountType::Draw].
    pub fn new(
        authority: Pubkey,
        id: u64,
        amount: u64,
        receiver_seed: u64,
        receiver: Pubkey,
        rollover: u8,
        slot: u64,
        epoch_start_timestamp: i64,
        epoch: u64,
        unix_timestamp: i64,
    ) -> Self {
        Self { 
            account_type: BondAccountType::Draw, 
            authority,
            id,
            amount,
            receiver_seed,
            receiver,
            rollover,
            slot,
            epoch_start_timestamp,
            epoch,
            unix_timestamp,
        }
    }
}


/// Bond Fee
/// ------------------------------------------------------------------------------------------------

/// The account that collects the Stake Pool's epoch fee.
#[repr(C)]
#[derive(Clone, Debug, Default, PartialEq, BorshDeserialize, BorshSerialize, BorshSchema)]
pub struct BondFee {

    /// [BondAccountType::Fee].
    pub account_type: BondAccountType,

    /// The account authorized to modify this account.
    pub authority: Pubkey,

    /// The derived account's bump seed.
    pub bump: u8,
}

impl BondAccount for BondFee {
    
    /// True if `account_type` is not [BondAccountType::Uninitialized].
    fn is_initialized(&self) -> bool {
        self.account_type != BondAccountType::Uninitialized
    }

    /// True if `account_type` is [BondAccountType::Fee].
    fn is_valid(&self) -> bool {
        self.account_type == BondAccountType::Fee
    }
}

impl BondProgramAccount for BondFee {
    fn authority(&self) -> Pubkey {
        self.authority
    }
}

impl BondProgramDerivedAccount for BondFee {
    fn bump(&self) -> u8 {
        self.bump
    }
}

impl BondFee {
    
    /// Creates a new instance of [BondFee] with an `account_type` of [BondAccountType::Fee].
    pub fn new(
        authority: Pubkey,
        bump: u8, 
    ) -> Self {
        Self { 
            account_type: BondAccountType::Fee,
            authority, 
            bump,
        }
    }
}


/// Exclusion List
/// ------------------------------------------------------------------------------------------------

/// A list of accounts that are ineligible to win the Bond draw.
#[repr(C)]
#[derive(Clone, Debug, Default, PartialEq, BorshDeserialize, BorshSerialize, BorshSchema)]
pub struct BondExclusionList {

    /// [BondAccountType::ExclusionList].
    pub account_type: BondAccountType,

    /// The account authorized to modify this account.
    pub authority: Pubkey,

    /// The derived account's bump seed.
    pub bump: u8,
    
    /// The maximum number of accounts.
    pub capacity: u32,

    /// The excluded accounts.
    pub accounts: Vec<Pubkey>,
}

impl BondAccount for BondExclusionList {
    
    /// True if `account_type` is not [BondAccountType::Uninitialized].
    fn is_initialized(&self) -> bool {
        self.account_type != BondAccountType::Uninitialized
    }

    /// True if `account_type` is [BondAccountType::ExclusionList].
    fn is_valid(&self) -> bool {
        self.account_type == BondAccountType::ExclusionList
    }
}

impl BondProgramAccount for BondExclusionList {
    fn authority(&self) -> Pubkey {
        self.authority
    }
}

impl BondProgramDerivedAccount for BondExclusionList {
    fn bump(&self) -> u8 {
        self.bump
    }
}

impl BondExclusionList {

    /// Creates a new instance of [BondExclusionList] with an `account_type` of 
    /// [BondAccountType::ExclusionList].
    pub fn new(
        authority: Pubkey,
        bump: u8, 
        capacity: u32,
        accounts: Vec<Pubkey>,
    ) -> Self {
        Self { 
            account_type: BondAccountType::ExclusionList,
            authority, 
            bump,
            capacity,
            accounts,
        }
    }
}