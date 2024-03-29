//! Instruction types.


// Imports
// -------------------------------------------------------------------------------------------------

use {
    borsh::{BorshDeserialize, BorshSerialize},
    solana_program::pubkey::Pubkey,
};


// Intructions
// -------------------------------------------------------------------------------------------------

/// The instructions supported by the Bond program.
#[repr(C)]
#[derive(Clone, Debug, PartialEq, BorshSerialize, BorshDeserialize)]
pub enum BondInstruction {
    
    /// Creates accounts for the PDAs and their corresponding ATA accounts where applicable.
    /// 
    /// The instruction requires each account's bump seed and allocation size so that these may be 
    /// computed off chain.
    /// 
    /// ## Accounts
    /// - `[s]` `[payer]` - The fee payer.
    /// - `[s, w]` `[config]` - The game's settings and main authority. All PDAs are generated by 
    ///     the seeds [`config`, [BondSeed]].
    /// - `[w]` `[state]` - The game's current state (PDA of [`config`, [BondSeed::State]]).
    /// - `[w]` `[fee]` The Stake Pool's epoch fee receiver (PDA of [`config`, [BondSeed::Fee]]).
    /// - `[w]` `[fee_ata]` The associated token address of `fee` for `token_mint`.
    /// - `[w]` `[exclusion_list]` The accounts excluded from all draws, such as personal or 
    ///     business accounts (PDA of [`config`, [BondSeed::ExclusionList]]).
    /// - `[w]` `[equity]` The shareholders account (PDA of [`config`, [BondSeed::Equity]]).
    /// - `[w]` `[equity_ata]` The associated token address of `equity` for `token_mint`.
    /// - `[w]` `[treasury]` The treasury account (PDA of [`config`, [BondSeed::Treasury]]).
    /// - `[w]` `[treasury_ata]` The associated token address of `treasury` for `token_mint`.
    /// - `[w]` `[jackpot]` The jackpot account (PDA of [`config`, [BondSeed::Jackpot]]).
    /// - `[w]` `[jackpot_ata]` The associated token address of `jackpot` for `token_mint`.
    /// - `[w]` `[stake]` The locked stake  (PDA of [`config`, [BondSeed::Stake]]).
    /// - `[w]` `[stake_ata]` The associated token address of `stake` for `token_mint`.
    /// - `[]` `[token_mint]`- The Stake Pool's token mint.
    /// - `[]` `[token_program]`- The Token Program's id.
    /// - `[]` `[associated_token_program]`- The Associated Token Program's id.
    /// - `[]` `[system_program]`- The System Program's id.
    /// 
    /// ## Data
    /// - `[config_space]` - The allocation size of a [BondConfig] account.
    /// - `[state_bump]` - [BondState]'s PDA bump seed.
    /// - `[state_space]` - The allocation size of a [BondState] account.
    /// - `[fee_bump]` - [BondFee]'s PDA bump seed.
    /// - `[fee_space]` - The allocation size of a [BondFee] account.
    /// - `[exclusion_list_bump]` - [BondExclusionList]'s PDA bump seed.
    /// - `[exclusion_list_space]` - The allocation size of a [BondExclusionList] account.
    /// - `[equity_bump]` - Equity ([BondShare]) PDA bump seed.
    /// - `[equity_space]` - The allocation size of a [BondShare] account.
    /// - `[treasury_bump]` - Treasury ([BondShare]) PDA bump seed.
    /// - `[treasury_space]` - The allocation size of a [BondShare] account.
    /// - `[jackpot_bump]` - Jackpot ([BondShare]) PDA bump seed.
    /// - `[jackpot_space]` - The allocation size of a [BondShare] account.
    /// - `[stake_bump]` - Stake ([BondShare]) PDA bump seed.
    /// - `[stake_space]` - The allocation size of a [BondShare] account.
    Create {
        config_space: u32,
        state_bump: u8,
        state_space: u32,
        fee_bump: u8,
        fee_space: u32,
        exclusion_list_bump: u8,
        exclusion_list_space: u32,
        equity_bump: u8,
        equity_space: u32,
        treasury_bump: u8,
        treasury_space: u32,
        jackpot_bump: u8,
        jackpot_space: u32,
        stake_bump: u8,
        stake_space: u32,
    },
    
    /// Initializes PDAs (and ATAs if applicable) for the Bond program.
    /// 
    /// `The accounts must be created before calling Initialize`.
    /// 
    /// ## Accounts
    /// - `[s]` `[payer]` - The fee payer.
    /// - `[s, w]` `[config]` - The game's settings and main authority.
    /// - `[s]` `[draw_authority]` - The authority designated to run draws.
    /// - `[]` `[token_mint]` - The Stake Pool's token mint address.
    /// - `[w]` `[state]` - The game's current state ([BondState]).
    /// - `[w]` `[fee]` - The Stake Pool's epoch fee receiver ([BondFee]).
    /// - `[w]` `[exclusion_list]` - The accounts excluded from all draws ([BondExclusionList]).
    /// - `[w]` `[equity]` - The shareholders account ([BondShare]).
    /// - `[w]` `[treasury]` - The treasury account ([BondShare]).
    /// - `[w]` `[jackpot]` - The jackpot account ([BondShare]).
    /// - `[w]` `[stake]` - The locked stake (min balance) ([BondShare]).
    /// - `[]` `[token_program]` - The Token Program's id.
    /// - `[]` `[associated_token_program]` - The Associated Token Program's id.
    /// - `[]` `[system_program]` - The System Program's id.
    /// 
    /// ## Data
    /// - `[state_bump]` - [BondState]'s PDA bump seed.
    /// - `[fee_bump]` - [BondFee]'s PDA bump seed.
    /// - `[exclusion_list_bump]` - [BondExclusionList]'s PDA bump seed.
    /// - `[exclusion_list_capacity]` - The max length of the accounts list.
    /// - `[exclusion_list_accounts]` - The list of accounts to exclude from all draws.
    /// - `[equity_bump]` - Equity ([BondShare]) PDA bump seed.
    /// - `[treasury_bump]` - Treasury ([BondShare]) PDA bump seed.
    /// - `[jackpot_bump]` - Jackpot ([BondShare]) PDA bump seed.
    /// - `[stake_bump]` - Stake ([BondShare]) PDA bump seed.
    Initialize {
        state_bump: u8,
        fee_bump: u8,
        exclusion_list_bump: u8,
        exclusion_list_capacity: u32,
        exclusion_list_accounts: Vec<Pubkey>,
        equity_bump: u8,
        treasury_bump: u8,
        jackpot_bump: u8,
        stake_bump: u8,

        // rollover_bump: u8,
        // treasury_bump: u8,
        // stake_bump: u8,
        // equity_bump: u8,
        // draw_bump: u8,
        // fee_bump: u8,
        // exclusion_list_bump: u8,
        // exclusion_list_capacity: u32,
        // exclusion_list: Vec<Pubkey>,
    },

    SplitShares {
        amount: Option<u64>,
    },

    /// Runs a Bond draw.
    Draw {
        receiver_seed: u64,
        draw_seed: u64,
    },

    Test,
}