//! Program state processor.


// Imports
// -------------------------------------------------------------------------------------------------

use solana_program::program_pack::Pack;
use spl_token::state::Account;
use {
    crate::{
        instruction::BondInstruction,
        state::*,
        check::Check,
        create::Create,
    },
    borsh::{BorshDeserialize, BorshSerialize},
    solana_program::{
        account_info::{next_account_info, AccountInfo},
        borsh::try_from_slice_unchecked,
        clock::Clock,
        entrypoint::ProgramResult,
        msg,
        program_error::ProgramError,
        pubkey::Pubkey,
        rent::Rent, 
        sysvar::Sysvar, 
        system_program,
    },
};


// Processor
// -------------------------------------------------------------------------------------------------

pub struct Processor;

impl Processor {

    pub fn process(
        program_id: &Pubkey, 
        accounts: &[AccountInfo], 
        instruction_data: &[u8],
    ) -> ProgramResult {
        msg!("Process Instruction...");
        let instruction = BondInstruction::try_from_slice(instruction_data)?;
        msg!("Process Instruction Data...");
        match instruction {
            BondInstruction::Create { 
                config_space, 
                state_bump, 
                state_space, 
                fee_bump, 
                fee_space, 
                exclusion_list_bump, 
                exclusion_list_space, 
                equity_bump, 
                equity_space, 
                treasury_bump, 
                treasury_space, 
                jackpot_bump, 
                jackpot_space, 
                stake_bump, 
                stake_space 
            } => {
                msg!("Instruction: Create accounts");
                Self::process_create(
                    program_id, 
                    accounts, 
                    config_space, 
                    state_bump, 
                    state_space, 
                    fee_bump, 
                    fee_space, 
                    exclusion_list_bump, 
                    exclusion_list_space, 
                    equity_bump, 
                    equity_space, 
                    treasury_bump, 
                    treasury_space, 
                    jackpot_bump, 
                    jackpot_space, 
                    stake_bump, 
                    stake_space, 
                )
            },
            BondInstruction::Initialize { 
                state_bump, 
                fee_bump, 
                exclusion_list_bump, 
                exclusion_list_capacity, 
                exclusion_list_accounts, 
                equity_bump, 
                treasury_bump, 
                jackpot_bump, 
                stake_bump, 
            } => {
                msg!("Instruction: Initialize accounts");
                Self::process_initialize(
                    program_id, 
                    accounts, 
                    state_bump,
                    fee_bump,
                    exclusion_list_bump,
                    exclusion_list_capacity,
                    exclusion_list_accounts,
                    equity_bump,
                    treasury_bump,
                    jackpot_bump,
                    stake_bump,
                )
            },
            BondInstruction::SplitShares {
                amount,
            } => {
                msg!("Instruction: Split Shares");
                Self::process_split_shares(
                    program_id, 
                    accounts, 
                    amount,
                )
            },
            BondInstruction::Draw {
                receiver_seed,
                draw_seed,
            } => {
                msg!("Instruction: Draw");
                Self::process_draw(
                    program_id, 
                    accounts, 
                    receiver_seed,
                    draw_seed,
                )
            },
            BondInstruction::Test => {
                Self::process_test(program_id, accounts)
            }
        }
    }

    fn process_test(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
    ) -> ProgramResult {
        Ok(())
    }

    /// Create accounts.
    fn process_create(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
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
    ) -> ProgramResult {

        // Unpack accounts.
        let account_info_iter = &mut accounts.iter();
        let payer_info = next_account_info(account_info_iter)?;
        let config_info = next_account_info(account_info_iter)?; 
        let state_info = next_account_info(account_info_iter)?;
        let fee_info = next_account_info(account_info_iter)?; 
        let fee_ata_info = next_account_info(account_info_iter)?; 
        let exclusion_list_info = next_account_info(account_info_iter)?; 
        let equity_info = next_account_info(account_info_iter)?; 
        let equity_ata_info = next_account_info(account_info_iter)?; 
        let treasury_info = next_account_info(account_info_iter)?; 
        let treasury_ata_info = next_account_info(account_info_iter)?; 
        let jackpot_info = next_account_info(account_info_iter)?; 
        let jackpot_ata_info = next_account_info(account_info_iter)?; 
        let stake_info = next_account_info(account_info_iter)?; 
        let stake_ata_info = next_account_info(account_info_iter)?; 
        let token_mint_info = next_account_info(account_info_iter)?;
        let token_program_info = next_account_info(account_info_iter)?;
        let associated_token_program_info = next_account_info(account_info_iter)?;
        let system_program_info = next_account_info(account_info_iter)?;

        // Validate accounts.
        Check::signer(payer_info)?;
        Check::signer_and_writable(config_info)?;
        Check::writable(state_info)?;
        Check::writable(fee_info)?;
        Check::writable(fee_ata_info)?;
        Check::writable(exclusion_list_info)?;
        Check::writable(equity_info)?;
        Check::writable(equity_ata_info)?;
        Check::writable(treasury_info)?;
        Check::writable(treasury_ata_info)?;
        Check::writable(jackpot_info)?;
        Check::writable(jackpot_ata_info)?;
        Check::writable(stake_info)?;
        Check::writable(stake_ata_info)?;
        Check::owner(token_mint_info, &spl_token::id())?;
        Check::account(token_program_info, &spl_token::id())?;
        Check::account(associated_token_program_info, &spl_associated_token_account::id())?;
        Check::account(system_program_info, &system_program::id())?;

        // Rent.
        let rent = Rent::get()?;

        // Config Account.
        Create::account(
            program_id, 
            payer_info, 
            config_info, 
            system_program_info, 
            &rent,
            config_space,
        )?;

        // State PDA Account.
        Create::pda_account(
            program_id, 
            config_info, 
            payer_info, 
            &state_info,
            BondSeed::State,
            state_bump,
            system_program_info,
            &rent,
            state_space,
        )?;

        // Fee PDA + ATA Accounts.
        Create::pda_and_ata_accounts(
            program_id, 
            config_info, 
            payer_info, 
            fee_info, 
            BondSeed::Fee, 
            fee_bump, 
            fee_ata_info, 
            token_mint_info, 
            token_program_info, 
            associated_token_program_info, 
            system_program_info, 
            &rent,
            fee_space,
        )?;

        // Exclusion List PDA Account.
        Create::pda_account(
            program_id, 
            config_info, 
            payer_info, 
            exclusion_list_info, 
            BondSeed::ExclusionList, 
            exclusion_list_bump, 
            system_program_info,
            &rent,
            exclusion_list_space
        )?;

        // Equity PDA + ATA Accounts.
        Create::pda_and_ata_accounts(
            program_id, 
            config_info, 
            payer_info, 
            equity_info, 
            BondSeed::Equity, 
            equity_bump, 
            equity_ata_info, 
            token_mint_info, 
            token_program_info, 
            associated_token_program_info, 
            system_program_info, 
            &rent,
            equity_space,
        )?;

        // Treasury PDA + ATA Accounts.
        Create::pda_and_ata_accounts(
            program_id, 
            config_info, 
            payer_info, 
            treasury_info, 
            BondSeed::Treasury, 
            treasury_bump, 
            treasury_ata_info, 
            token_mint_info, 
            token_program_info, 
            associated_token_program_info, 
            system_program_info, 
            &rent,
            treasury_space,
        )?;

        // Jackpot PDA + ATA Accounts.
        Create::pda_and_ata_accounts(
            program_id, 
            config_info, 
            payer_info, 
            jackpot_info, 
            BondSeed::Jackpot, 
            jackpot_bump, 
            jackpot_ata_info, 
            token_mint_info, 
            token_program_info, 
            associated_token_program_info, 
            system_program_info, 
            &rent,
            jackpot_space,
        )?;

        // Stake PDA + ATA Accounts.
        Create::pda_and_ata_accounts(
            program_id, 
            config_info, 
            payer_info, 
            stake_info, 
            BondSeed::Stake, 
            stake_bump, 
            stake_ata_info, 
            token_mint_info, 
            token_program_info, 
            associated_token_program_info, 
            system_program_info, 
            &rent,
            stake_space,
        )
    }

    fn check_initialize_account(
        program_id: &Pubkey,
        account: &impl BondAccount,
        account_info: &AccountInfo,
        rent: &Rent,
    ) -> Result<(), ProgramError> {
        // Check::uninitialized(account, account_info)?;
        Check::writable(account_info)?; 
        Check::rent_exempt(account_info, rent)?;
        Check::owner(account_info, program_id)?;
        Ok(())
    }

    fn check_initialize_pda_account(
        program_id: &Pubkey,
        config_info: &AccountInfo,
        pda: &impl BondAccount,
        pda_info: &AccountInfo,
        seed: BondSeed,
        bump: u8,
        rent: &Rent,
    ) -> Result<(), ProgramError> {
        Check::pda(program_id, config_info, pda_info, seed, bump)?;
        Self::check_initialize_account(program_id, pda, pda_info, rent)
    }

    fn initialize_share(
        program_id: &Pubkey,
        config_info: &AccountInfo,
        account_info: &AccountInfo,
        authority: Pubkey,
        numerator: u32,
        denominator: u32,
        seed: BondSeed,
        bump: u8,
        rent: &Rent,
    ) -> ProgramResult {
        let account = try_from_slice_unchecked::<BondShare>(
            &account_info.data.borrow(),
        )?;
        Self::check_initialize_pda_account(
            program_id, 
            config_info, 
            &account,
            account_info, 
            seed, 
            bump, 
            &rent,
        )?;
        BondShare::new(
            authority, 
            bump, 
            numerator,
            denominator,
        ).serialize(
            &mut &mut account_info.data.borrow_mut()[..],
        )?;
        Ok(())
    }

    /// Initialize accounts.
    fn process_initialize(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        state_bump: u8,
        fee_bump: u8,
        exclusion_list_bump: u8,
        exclusion_list_capacity: u32,
        exclusion_list_accounts: Vec<Pubkey>,
        equity_bump: u8,
        treasury_bump: u8,
        jackpot_bump: u8,
        stake_bump: u8,
    ) -> ProgramResult {

        // Unpack accounts.
        let account_info_iter = &mut accounts.iter();
        let payer_info = next_account_info(account_info_iter)?;
        let config_info = next_account_info(account_info_iter)?;
        let draw_authority_info = next_account_info(account_info_iter)?;
        let token_mint_info = next_account_info(account_info_iter)?;
        let state_info = next_account_info(account_info_iter)?;
        let fee_info = next_account_info(account_info_iter)?;
        let exclusion_list_info = next_account_info(account_info_iter)?;
        let equity_info = next_account_info(account_info_iter)?;
        let treasury_info = next_account_info(account_info_iter)?;
        let jackpot_info = next_account_info(account_info_iter)?;
        let stake_info = next_account_info(account_info_iter)?;

        // Initial validation.
        Check::signer(payer_info)?;
        Check::signer_and_writable(config_info)?;
        Check::signer(draw_authority_info)?;
        Check::owner(token_mint_info, &spl_token::id())?;

        // The default PDA account authority.
        let authority = *config_info.key;

        // Rent.
        let rent = Rent::get()?;

        // Config Account.
        msg!("Initialize Config");
        let config = try_from_slice_unchecked::<BondConfig>(
            &config_info.data.borrow(),
        )?;
        Self::check_initialize_account(
            program_id, 
            &config, 
            config_info,
            &rent,
        )?;
        BondConfig::new(
            true,
            0, 
            3,
            10, 
            100, 
            *draw_authority_info.key, 
            *token_mint_info.key,
        ).serialize(
            &mut &mut config_info.data.borrow_mut()[..],
        )?;

        // State PDA Account.
        msg!("Initialize State");
        let state = try_from_slice_unchecked::<BondState>(
            &state_info.data.borrow(),
        )?;
        Self::check_initialize_pda_account(
            program_id, 
            config_info, 
            &state,
            state_info, 
            BondSeed::State, 
            state_bump, 
            &rent,
        )?;
        BondState::new(
            authority,
            state_bump,
            0,
            0,
        ).serialize(
            &mut &mut state_info.data.borrow_mut()[..],
        )?;

        // Fee PDA Account.
        msg!("Initialize Fee");
        let fee = try_from_slice_unchecked::<BondFee>(
            &fee_info.data.borrow(),
        )?;
        Self::check_initialize_pda_account(
            program_id, 
            config_info, 
            &fee,
            fee_info, 
            BondSeed::Fee, 
            fee_bump, 
            &rent,
        )?;
        BondFee::new(
            authority,
            fee_bump,
        ).serialize(
            &mut &mut fee_info.data.borrow_mut()[..],
        )?;

        // Exclusion List PDA Account.
        msg!("Initialize Exclusion List");
        let exclusion_list = try_from_slice_unchecked::<BondExclusionList>(
            &exclusion_list_info.data.borrow(),
        )?;
        Self::check_initialize_pda_account(
            program_id, 
            config_info, 
            &exclusion_list,
            exclusion_list_info, 
            BondSeed::ExclusionList, 
            exclusion_list_bump, 
            &rent,
        )?;
        BondExclusionList::new(
            authority,
            exclusion_list_bump,
            exclusion_list_capacity,
            exclusion_list_accounts,
        ).serialize(
            &mut &mut exclusion_list_info.data.borrow_mut()[..],
        )?;

        // Equity PDA Account.
        msg!("Initialize Equity");
        Self::initialize_share(
            program_id, 
            config_info, 
            equity_info, 
            authority, 
            10,
            100, 
            BondSeed::Equity, 
            equity_bump, 
            &rent,
        )?;

        // Treasury PDA Account.
        msg!("Initialize Treasury");
        Self::initialize_share(
            program_id, 
            config_info, 
            treasury_info, 
            authority, 
            0, 
            0,
            BondSeed::Treasury, 
            treasury_bump, 
            &rent,
        )?;

        // Jackpot PDA Account.
        msg!("Initialize Jackpot");
        Self::initialize_share(
            program_id, 
            config_info, 
            jackpot_info, 
            authority, 
            80, 
            100,
            BondSeed::Jackpot, 
            jackpot_bump, 
            &rent,
        )?;

        // Stake PDA Account.
        msg!("Initialize Stake");
        Self::initialize_share(
            program_id, 
            config_info, 
            stake_info, 
            authority, 
            10, 
            100, 
            BondSeed::Stake, 
            stake_bump, 
            &rent,
        )?;

        msg!("Initialize Bond Complete!");
        Ok(())
    }

    fn check_draw_account(
        program_id: &Pubkey,
        config_info: &AccountInfo,
        draw_authority_info: &AccountInfo,
        config: &BondConfig,
    ) -> Result<(), ProgramError> {
        Check::owner(config_info, program_id)?;
        Check::signer(draw_authority_info)?;
        Check::account(draw_authority_info, &config.draw_authority)?;
        Check::valid(config, config_info)
    }

    fn check_draw_pda_account(
        program_id: &Pubkey,
        config_info: &AccountInfo,
        share_info: &AccountInfo,
        share: &impl BondProgramAccount,
        share_ata_info: &AccountInfo,
        share_ata: &Account,
    ) -> Result<(), ProgramError> {
        Check::account(config_info, &share.authority())?;
        Check::owner(&share_info, program_id)?;
        Check::valid(share, share_info)?;
        Check::pubkey(&share_ata.owner, &share_info.key)
    }

    fn process_split_shares(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        amount: Option<u64>,
    ) -> ProgramResult {

        let account_info_iter = &mut accounts.iter();
        let draw_authority_info = next_account_info(account_info_iter)?;
        let config_info = next_account_info(account_info_iter)?;
        let config = try_from_slice_unchecked::<BondConfig>(&config_info.data.borrow())?;
        Self::check_draw_account(program_id, config_info, draw_authority_info, &config)?;

        let fee_info = next_account_info(account_info_iter)?;
        let fee = try_from_slice_unchecked::<BondFee>(&fee_info.data.borrow())?;
        let fee_ata_info = next_account_info(account_info_iter)?;
        let fee_ata = Account::unpack_from_slice(&fee_ata_info.data.borrow())?;
        Self::check_draw_pda_account(
            program_id, 
            config_info, 
            fee_info, 
            &fee, 
            fee_ata_info,
            &fee_ata,
        )?;
        
        let equity_info = next_account_info(account_info_iter)?;
        let equity = try_from_slice_unchecked::<BondShare>(&equity_info.data.borrow())?;
        let equity_ata_info = next_account_info(account_info_iter)?;
        let equity_ata = Account::unpack_from_slice(&equity_ata_info.data.borrow())?;
        Self::check_draw_pda_account(
            program_id, 
            config_info, 
            equity_info, 
            &equity, 
            equity_ata_info,
            &equity_ata,
        )?;

        let treasury_info = next_account_info(account_info_iter)?;
        let treasury = try_from_slice_unchecked::<BondShare>(&treasury_info.data.borrow())?;
        let treasury_ata_info = next_account_info(account_info_iter)?;
        let treasury_ata = Account::unpack_from_slice(&treasury_ata_info.data.borrow())?;
        Self::check_draw_pda_account(
            program_id, 
            config_info, 
            treasury_info, 
            &treasury, 
            treasury_ata_info,
            &treasury_ata,
        )?;

        let jackpot_info = next_account_info(account_info_iter)?;
        let jackpot = try_from_slice_unchecked::<BondShare>(&jackpot_info.data.borrow())?;
        let jackpot_ata_info = next_account_info(account_info_iter)?;
        let jackpot_ata = Account::unpack_from_slice(&jackpot_ata_info.data.borrow())?;
        Self::check_draw_pda_account(
            program_id, 
            config_info, 
            jackpot_info, 
            &jackpot, 
            jackpot_ata_info,
            &jackpot_ata,
        )?;

        let stake_info = next_account_info(account_info_iter)?;       
        let stake = try_from_slice_unchecked::<BondShare>(&stake_info.data.borrow())?; 
        let stake_ata_info = next_account_info(account_info_iter)?;
        let stake_ata = Account::unpack_from_slice(&stake_ata_info.data.borrow())?;
        Self::check_draw_pda_account(
            program_id, 
            config_info, 
            stake_info, 
            &stake, 
            stake_ata_info,
            &stake_ata,
        )?;

        let token_mint_info = next_account_info(account_info_iter)?;
        let token_program_info = next_account_info(account_info_iter)?;  

        let amount: u64 = amount.unwrap_or(fee_ata.amount);

        // Fee -> Equity
        Create::token_transfer_checked(
            draw_authority_info,
            config_info,
            token_program_info,
            token_mint_info,
            fee_ata_info,
            equity_ata_info,
            fee_info,
            BondSeed::Fee,
            fee.bump,
            equity.share(amount),
        )?;

        // Fee -> Treasury
        Create::token_transfer_checked(
            draw_authority_info,
            config_info,
            token_program_info,
            token_mint_info,
            fee_ata_info,
            treasury_ata_info,
            fee_info,
            BondSeed::Fee,
            fee.bump,
            treasury.share(amount),
        )?;

        // Fee -> Jackpot
        Create::token_transfer_checked(
            draw_authority_info,
            config_info,
            token_program_info,
            token_mint_info,
            fee_ata_info,
            jackpot_ata_info,
            fee_info,
            BondSeed::Fee,
            fee.bump,
            jackpot.share(amount),
        )?; 

        // Fee -> Stake
        Create::token_transfer_checked(
            draw_authority_info,
            config_info,
            token_program_info,
            token_mint_info,
            fee_ata_info,
            stake_ata_info,
            fee_info,
            BondSeed::Fee,
            fee.bump,
            stake.share(amount),
        )
    }

    fn process_draw(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        receiver_seed: u64,
        draw_seed: u64,
    ) -> ProgramResult {

        // Unpack accounts...
        let account_info_iter = &mut accounts.iter();
        let draw_authority_info = next_account_info(account_info_iter)?;
        let config_info = next_account_info(account_info_iter)?;
        let config = try_from_slice_unchecked::<BondConfig>(&config_info.data.borrow())?;
        Self::check_draw_account(program_id, config_info, draw_authority_info, &config)?;

        let state_info = next_account_info(account_info_iter)?;
        let mut state = try_from_slice_unchecked::<BondState>(&state_info.data.borrow_mut())?;
        Check::owner(state_info, program_id)?;
        Check::valid(&state, state_info)?;
        Check::account(config_info, &state.authority)?;
        
        let jackpot_info = next_account_info(account_info_iter)?;
        let jackpot = try_from_slice_unchecked::<BondShare>(&jackpot_info.data.borrow())?;
        let jackpot_ata_info = next_account_info(account_info_iter)?;
        let jackpot_ata = Account::unpack_from_slice(&jackpot_ata_info.data.borrow())?;
        Self::check_draw_pda_account(
            program_id, 
            config_info, 
            jackpot_info, 
            &jackpot, 
            jackpot_ata_info,
            &jackpot_ata,
        )?;

        let receiver_info = next_account_info(account_info_iter)?;
        let receiver_ata_info = next_account_info(account_info_iter)?;
        let receiver_ata = Account::unpack_from_slice(&receiver_ata_info.data.borrow())?;
        Check::pubkey(&receiver_ata.owner, receiver_info.key)?;

        let draw_info = next_account_info(account_info_iter)?;
        let draw = try_from_slice_unchecked::<BondDraw>(&draw_info.data.borrow())?;
        Check::uninitialized(&draw, draw_info)?;
        Check::owner(draw_info, program_id)?;

        let token_mint_info = next_account_info(account_info_iter)?;
        let token_program_info = next_account_info(account_info_iter)?; 

        let amount = jackpot_ata.amount;
        if amount == 0 {
            return Ok(())
        }

        let epoch = Clock::get()?;
        let id = state.draw_id + 1;

        if draw_seed != id {
            return Err(ProgramError::InvalidSeeds);
        }

        let is_rollover = receiver_info.key.eq(jackpot_info.key);
        let rollover = if is_rollover { state.rollover + 1 } else { 0 };

        BondDraw::new(
            draw_authority_info.key.clone(),
            id,
            amount,
            receiver_seed,
            receiver_info.key.clone(),
            rollover,
            epoch.slot,
            epoch.epoch_start_timestamp,
            epoch.epoch,
            epoch.unix_timestamp,
        ).serialize(
            &mut &mut draw_info.data.borrow_mut()[..],
        )?;

        state.draw_id = id;
        state.rollover = rollover;
        state.serialize(&mut &mut state_info.data.borrow_mut()[..])?;

        if !is_rollover {
            // Jackpot -> Winner!
            Create::token_transfer_checked(
                draw_authority_info,
                config_info,
                token_program_info,
                token_mint_info,
                jackpot_ata_info,
                receiver_ata_info,
                jackpot_info,
                BondSeed::Jackpot,
                jackpot.bump,
                amount,
            )?;
        }

        Ok(())
    }

}