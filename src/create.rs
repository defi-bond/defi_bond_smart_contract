//! Create conditions

use solana_program::pubkey::PubkeyError;

use crate::check::Check;


/// Imports
/// ------------------------------------------------------------------------------------------------
use {
    crate::{
        state::LottoSeed,
    },
    borsh::{BorshDeserialize, BorshSerialize},
    solana_program::{
        account_info::{next_account_info, AccountInfo},
        borsh::try_from_slice_unchecked,
        clock::Clock,
        entrypoint::ProgramResult,
        instruction::Instruction,
        msg,
        program::{invoke, invoke_signed}, 
        program_error::ProgramError,
        pubkey::Pubkey,
        rent::Rent, 
        sysvar::Sysvar, 
        system_instruction, 
        system_program,
    },
    spl_associated_token_account::{
        instruction::{
            create_associated_token_account_idempotent
        }
    },
    std::convert::AsRef,
};


/// Create
/// ------------------------------------------------------------------------------------------------

/// Creates accounts.
pub struct Create;

impl Create {
    
    // pub fn seeds<'a: 'b, 'c>(
    //     config_info: &'a AccountInfo<'a>,
    //     seed: &'b LottoSeed,
    //     bump: u8,
    // ) -> [&'c [u8]; 3] {
    //     return [
    //         config_info.key.as_ref(),
    //         seed.as_ref().as_ref(),
    //         &[bump],
    //     ]
    // }

    pub fn seeds<'a: 'b, 'b>(
        config_info: &'b AccountInfo<'a>,
        seed: &'b [u8],
        bump: &'b [u8],
    ) -> [&'b [u8]; 3] {
        return [
            config_info.key.as_ref(),
            seed,
            bump,
        ]
    }

    pub fn account_ix(
        program_id: &Pubkey,
        rent: &Rent,
        payer: &Pubkey,
        address: &Pubkey,
        space: usize,
    ) -> Instruction {
        system_instruction::create_account(
            payer, 
            address, 
            rent.minimum_balance(space), 
            space.try_into().unwrap(), 
            program_id,
        )
    }

    pub fn account<'a: 'b, 'b>(
        program_id: &'b Pubkey,
        payer_info: &'b AccountInfo<'a>,
        account_info: &'b AccountInfo<'a>,
        system_program_info: &'b AccountInfo<'a>,
        rent: &'b Rent,
        space: u32,
    ) -> ProgramResult {
        let ix = Self::account_ix(
            program_id, 
            rent, 
            payer_info.key, 
            account_info.key,
            space.try_into().unwrap(),
        );
        invoke(
            &ix, 
            &[
                payer_info.clone(), 
                account_info.clone(), 
                system_program_info.clone(),
            ],
        )
    }

    pub fn pda(
        program_id: &Pubkey,
        config_info: &AccountInfo,
        seed: LottoSeed,
        bump: u8,
    ) -> Result<Pubkey, PubkeyError> {
        let bump = [bump];
        let seeds = Self::seeds(
            &config_info, 
            &seed.as_ref().as_ref(), 
            &bump,
        );
        Pubkey::create_program_address(
            &seeds,
            program_id,
        )
    }

    pub fn pda_account<'a: 'b, 'b>(
        program_id: &'b Pubkey,
        config_info: &'b AccountInfo<'a>,
        payer_info: &'b AccountInfo<'a>,
        pda_info: &'b AccountInfo<'a>,
        pda_seed: LottoSeed,
        pda_bump: u8,
        system_program_info: &'b AccountInfo<'a>,
        rent: &'b Rent,
        space: u32,
    ) -> ProgramResult {
        let bump = [pda_bump];
        let seeds = Self::seeds(
            &config_info, 
            &pda_seed.as_ref().as_ref(), 
            &bump,
        );
        let pda = Pubkey::create_program_address(
            &seeds,
            program_id,
        )?;
        Check::account(
            pda_info, 
            &pda,
        )?;
        let ix = Self::account_ix(
            program_id, 
            &rent, 
            payer_info.key, 
            &pda_info.key,
            space.try_into().unwrap(),
        );
        invoke_signed(
            &ix, 
            &[
                payer_info.clone(), 
                pda_info.clone(), 
                system_program_info.clone(),
            ],
            &[&seeds],
        )
    }

    pub fn ata_account<'a: 'b, 'b>(
        payer_info: &'b AccountInfo<'a>,
        ata_info: &'b AccountInfo<'a>,
        wallet_info: &'b AccountInfo<'a>,
        token_mint_info: &'b AccountInfo<'a>,
        token_program_info: &'b AccountInfo<'a>,
        associated_token_program_info: &'b AccountInfo<'a>,
        system_program_info: &'b AccountInfo<'a>,
    ) -> ProgramResult {
        let ix = create_associated_token_account_idempotent(
            &payer_info.key, 
            &wallet_info.key, 
            &token_mint_info.key, 
            &token_program_info.key,
        );
        invoke(
            &ix, 
            &[
                payer_info.clone(), 
                ata_info.clone(), 
                wallet_info.clone(),
                token_mint_info.clone(),
                system_program_info.clone(),
                token_program_info.clone(),
                associated_token_program_info.clone(),
            ],
        )
    }

    pub fn pda_and_ata_accounts<'a: 'b, 'b>(
        program_id: &'b Pubkey,
        config_info: &'b AccountInfo<'a>,
        payer_info: &'b AccountInfo<'a>,
        pda_info: &'b AccountInfo<'a>,
        pda_seed: LottoSeed,
        pda_bump: u8,
        ata_info: &'b AccountInfo<'a>,
        token_mint_info: &'b AccountInfo<'a>,
        token_program_info: &'b AccountInfo<'a>,
        associated_token_program_info: &'b AccountInfo<'a>,
        system_program_info: &'b AccountInfo<'a>,
        rent: &'b Rent,
        space: u32,
    ) -> ProgramResult {
        Self::pda_account(
            program_id, 
            config_info, 
            payer_info, 
            pda_info, 
            pda_seed, 
            pda_bump, 
            system_program_info, 
            &rent,
            space,
        )?;
        Self::ata_account(
            payer_info, 
            ata_info, 
            pda_info, 
            token_mint_info, 
            token_program_info, 
            associated_token_program_info, 
            system_program_info, 
        )
    }
    
    pub fn token_transfer_checked<'a, 'b>(
        draw_authotity_info: &'a AccountInfo<'b>,
        config_info: &'a AccountInfo<'b>,
        token_program_info: &'a AccountInfo<'b>,
        token_mint_info: &'a AccountInfo<'b>,
        source_info: &'a AccountInfo<'b>,
        destination_info: &'a AccountInfo<'b>,
        authority_info: &'a AccountInfo<'b>,
        seed: LottoSeed,
        bump: u8,
        amount: u64,
    ) -> ProgramResult {
        let binding = [bump];
        let seeds = Create::seeds(
            &config_info,
            seed.as_ref().as_ref(),
            &binding,
        );
        let ix = spl_token::instruction::transfer_checked(
            &token_program_info.key, 
            &source_info.key, 
            &token_mint_info.key, 
            &destination_info.key, 
            &authority_info.key, 
            &[],
            amount, 
            9,
        )?;
        invoke_signed(
            &ix, 
            &[
                draw_authotity_info.clone(),
                token_program_info.clone(),
                source_info.clone(),
                token_mint_info.clone(),
                destination_info.clone(),
                authority_info.clone(),
            ], 
            &[&seeds],
        )
    }
}