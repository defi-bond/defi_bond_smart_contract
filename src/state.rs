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

/// The types of program derived addresses managed by the Lotto program.
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize, BorshSchema)]
pub enum LottoAccountType {

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

impl Default for LottoAccountType {
    fn default() -> Self {
        LottoAccountType::Uninitialized
    }
}


/// Lotto Seed
/// -------------------------------------------------------------------------------------------------

/// The seeds of program derived addresses managed by the Lotto program.
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize, BorshSchema, AsRefStr)]
pub enum LottoSeed {

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


/// Lotto Account
/// ------------------------------------------------------------------------------------------------

/// A Lotto account (implemented by all account).
pub trait LottoAccount {

    /// True if the account has been initialized.
    fn is_initialized(&self) -> bool;

    /// True if the account has been initialized with the expected [LottoAccountType].
    fn is_valid(&self) -> bool;
}


/// Lotto Program Account
/// ------------------------------------------------------------------------------------------------

/// A Lotto account owned by the program.
pub trait LottoProgramAccount: LottoAccount {

    /// The account authorized to modify this account.
    fn authority(&self) -> Pubkey;
}


/// Lotto Program Derived Account
/// ------------------------------------------------------------------------------------------------

/// A Lotto account owned by the program with a derived address.
pub trait LottoProgramDerivedAccount: LottoProgramAccount {

    /// The derived account's bump seed.
    fn bump(&self) -> u8;
}


/// Lotto Config
/// ------------------------------------------------------------------------------------------------

/// The configurations and settings.
#[repr(C)]
#[derive(Clone, Debug, Default, PartialEq, BorshDeserialize, BorshSerialize, BorshSchema)]
pub struct LottoConfig {

    /// [LottoAccountType::Config].
    pub account_type: LottoAccountType,
    
    /// Whether or not the game is active.
    pub is_active: bool,

    /// The minimum number of epochs required between draws.
    pub epochs_per_draw: u8,

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

impl LottoAccount for LottoConfig {

    /// True if `account_type` is not [LottoAccountType::Uninitialized].
    fn is_initialized(&self) -> bool { 
        self.account_type != LottoAccountType::Uninitialized 
    }

    /// True if `account_type` is [LottoAccountType::Config].
    fn is_valid(&self) -> bool { 
        self.account_type == LottoAccountType::Config 
    }
}

impl LottoConfig {

    /// Creates a new instance of [LottoConfig] with an `account_type` of 
    /// [LottoAccountType::Config].
    pub fn new(
        is_active: bool,
        epochs_per_draw: u8,
        odds_threshold_numerator: u32,
        odds_threshold_denominator: u32,
        draw_authority: Pubkey,
        token_mint: Pubkey,
    ) -> Self {
        Self { 
            account_type: LottoAccountType::Config,
            is_active,
            epochs_per_draw,
            odds_threshold_numerator,
            odds_threshold_denominator,
            draw_authority,
            token_mint,
        }
    }
}


/// Lotto State
/// ------------------------------------------------------------------------------------------------

/// The current state of the game.
#[repr(C)]
#[derive(Clone, Debug, Default, PartialEq, BorshDeserialize, BorshSerialize, BorshSchema)]
pub struct LottoState {

    /// [LottoAccountType::State].
    pub account_type: LottoAccountType,
    
    /// The account authorized to modify this account.
    pub authority: Pubkey,

    /// The derived account's bump seed.
    pub bump: u8,

    /// The latest draw result id.
    pub draw_id: u64,
}

impl LottoAccount for LottoState {

    /// True if `account_type` is not [LottoAccountType::Uninitialized].
    fn is_initialized(&self) -> bool { 
        self.account_type != LottoAccountType::Uninitialized 
    }

    /// True if `account_type` is [LottoAccountType::State].
    fn is_valid(&self) -> bool { 
        self.account_type == LottoAccountType::State 
    }
}

impl LottoProgramAccount for LottoState {
    fn authority(&self) -> Pubkey {
        self.authority
    }
}

impl LottoProgramDerivedAccount for LottoState {
    fn bump(&self) -> u8 {
        self.bump
    }
}

impl LottoState {

    /// Creates a new instance of [LottoState] with an `account_type` of 
    /// [LottoAccountType::State].
    pub fn new(
        authority: Pubkey,
        bump: u8,
        draw_id: u64,
    ) -> Self {
        Self { 
            account_type: LottoAccountType::State,
            authority,
            bump,
            draw_id,
        }
    }
}


/// Lotto Share
/// ------------------------------------------------------------------------------------------------

/// An account that receives a share of the stake pool's rewards ([LottoFee]).
#[repr(C)]
#[derive(Clone, Debug, Default, PartialEq, BorshDeserialize, BorshSerialize, BorshSchema)]
pub struct LottoShare {

    /// [LottoAccountType::Share].
    pub account_type: LottoAccountType,

    /// The account authorized to modify this account.
    pub authority: Pubkey,

    /// The derived account's bump seed.
    pub bump: u8,

    /// The rewards share expressed by the percentage (0-100) `(numerator/denominator)*100`.
    pub numerator: u32,

    /// The rewards share expressed by the percentage (0-100) `(numerator/denominator)*100`.
    pub denominator: u32,
}

impl LottoAccount for LottoShare {

    /// True if `account_type` is not [LottoAccountType::Uninitialized].
    fn is_initialized(&self) -> bool {
        self.account_type != LottoAccountType::Uninitialized
    }

    /// True if `account_type` is [LottoAccountType::Share].
    fn is_valid(&self) -> bool {
        self.account_type == LottoAccountType::Share
    }
}

impl LottoProgramAccount for LottoShare {
    fn authority(&self) -> Pubkey {
        self.authority
    }
}

impl LottoProgramDerivedAccount for LottoShare {
    fn bump(&self) -> u8 {
        self.bump
    }
}

impl LottoShare {

    /// Creates a new instance of [LottoShare] with an `account_type` of [LottoAccountType::Share].
    pub fn new(
        authority: Pubkey, 
        bump: u8, 
        numerator: u32,
        denominator: u32,
    ) -> Self {
        Self { 
            account_type: LottoAccountType::Share, 
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


/// Lotto Draw
/// ------------------------------------------------------------------------------------------------

/// A draw result.
#[repr(C)]
#[derive(Clone, Debug, Default, PartialEq, BorshDeserialize, BorshSerialize, BorshSchema)]
pub struct LottoDraw {

    /// [LottoAccountType::Draw].
    pub account_type: LottoAccountType,

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

    /// The network/bank slot at which the draw took place.
    pub slot: u64,

    /// The timestamp of the first slot in `epoch`.
    pub epoch_start_timestamp: i64,

    /// The bank Epoch at which the draw took place.
    pub epoch: u64,

    /// The timestamp at which the draw took place.
    pub unix_timestamp: i64,
}

impl LottoAccount for LottoDraw {
    
    /// True if `account_type` is not [LottoAccountType::Uninitialized].
    fn is_initialized(&self) -> bool {
        self.account_type != LottoAccountType::Uninitialized
    }

    /// True if `account_type` is [LottoAccountType::Draw].
    fn is_valid(&self) -> bool {
        self.account_type == LottoAccountType::Draw
    }
}

impl LottoProgramAccount for LottoDraw {
    fn authority(&self) -> Pubkey {
        self.authority
    }
}

impl LottoDraw {

    /// Creates a new instance of [LottoDraw] with an `account_type` of [LottoAccountType::Draw].
    pub fn new(
        authority: Pubkey,
        id: u64,
        amount: u64,
        receiver_seed: u64,
        receiver: Pubkey,
        slot: u64,
        epoch_start_timestamp: i64,
        epoch: u64,
        unix_timestamp: i64,
    ) -> Self {
        Self { 
            account_type: LottoAccountType::Draw, 
            authority,
            id,
            amount,
            receiver_seed,
            receiver,
            slot,
            epoch_start_timestamp,
            epoch,
            unix_timestamp,
        }
    }
}


/// Lotto Fee
/// ------------------------------------------------------------------------------------------------

/// The account that collects the Stake Pool's epoch fee.
#[repr(C)]
#[derive(Clone, Debug, Default, PartialEq, BorshDeserialize, BorshSerialize, BorshSchema)]
pub struct LottoFee {

    /// [LottoAccountType::Fee].
    pub account_type: LottoAccountType,

    /// The account authorized to modify this account.
    pub authority: Pubkey,

    /// The derived account's bump seed.
    pub bump: u8,
}

impl LottoAccount for LottoFee {
    
    /// True if `account_type` is not [LottoAccountType::Uninitialized].
    fn is_initialized(&self) -> bool {
        self.account_type != LottoAccountType::Uninitialized
    }

    /// True if `account_type` is [LottoAccountType::Fee].
    fn is_valid(&self) -> bool {
        self.account_type == LottoAccountType::Fee
    }
}

impl LottoProgramAccount for LottoFee {
    fn authority(&self) -> Pubkey {
        self.authority
    }
}

impl LottoProgramDerivedAccount for LottoFee {
    fn bump(&self) -> u8 {
        self.bump
    }
}

impl LottoFee {
    
    /// Creates a new instance of [LottoFee] with an `account_type` of [LottoAccountType::Fee].
    pub fn new(
        authority: Pubkey,
        bump: u8, 
    ) -> Self {
        Self { 
            account_type: LottoAccountType::Fee,
            authority, 
            bump,
        }
    }
}


/// Exclusion List
/// ------------------------------------------------------------------------------------------------

/// A list of accounts that are ineligible to win the Lotto draw.
#[repr(C)]
#[derive(Clone, Debug, Default, PartialEq, BorshDeserialize, BorshSerialize, BorshSchema)]
pub struct LottoExclusionList {

    /// [LottoAccountType::ExclusionList].
    pub account_type: LottoAccountType,

    /// The account authorized to modify this account.
    pub authority: Pubkey,

    /// The derived account's bump seed.
    pub bump: u8,
    
    /// The maximum number of accounts.
    pub capacity: u32,

    /// The excluded accounts.
    pub accounts: Vec<Pubkey>,
}

impl LottoAccount for LottoExclusionList {
    
    /// True if `account_type` is not [LottoAccountType::Uninitialized].
    fn is_initialized(&self) -> bool {
        self.account_type != LottoAccountType::Uninitialized
    }

    /// True if `account_type` is [LottoAccountType::ExclusionList].
    fn is_valid(&self) -> bool {
        self.account_type == LottoAccountType::ExclusionList
    }
}

impl LottoProgramAccount for LottoExclusionList {
    fn authority(&self) -> Pubkey {
        self.authority
    }
}

impl LottoProgramDerivedAccount for LottoExclusionList {
    fn bump(&self) -> u8 {
        self.bump
    }
}

impl LottoExclusionList {

    /// Creates a new instance of [LottoExclusionList] with an `account_type` of 
    /// [LottoAccountType::ExclusionList].
    pub fn new(
        authority: Pubkey,
        bump: u8, 
        capacity: u32,
        accounts: Vec<Pubkey>,
    ) -> Self {
        Self { 
            account_type: LottoAccountType::ExclusionList,
            authority, 
            bump,
            capacity,
            accounts,
        }
    }
}