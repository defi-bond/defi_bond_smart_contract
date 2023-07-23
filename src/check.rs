//! Check conditions


/// Imports
/// ------------------------------------------------------------------------------------------------

use {
    crate::state::{
        BondSeed, 
        BondAccount,
    },
    solana_program::{
        account_info::AccountInfo, 
        program_error::ProgramError, 
        rent::Rent, 
        pubkey::Pubkey, 
        msg,
    },
};


/// Check
/// ------------------------------------------------------------------------------------------------

/// Asserts a condition or throws a [ProgramError].
pub struct Check;

impl Check {

    /// Check that `account_info` is rent exempt.
    pub fn rent_exempt(
        account_info: &AccountInfo,
        rent: &Rent,
    ) -> Result<(), ProgramError> {
        if !rent.is_exempt(account_info.lamports(), account_info.data_len()) {
            msg!(
                "Rent exempt account required for account {} with balance {}", 
                account_info.key,
                account_info.lamports(),
            );
            Err(ProgramError::AccountNotRentExempt)
        } else {
            Ok(())
        }
    }

    /// Check that `account_info` is a signer account.
    pub fn signer(
        account_info: &AccountInfo,
    ) -> Result<(), ProgramError> {
        if !account_info.is_signer {
            msg!("Missing signature for account {}", account_info.key);
            Err(ProgramError::MissingRequiredSignature)
        } else {
            Ok(())
        }
    }

    /// Check that `account_info` is a writable account.
    pub fn writable(
        account_info: &AccountInfo,
    ) -> Result<(), ProgramError> {
        if !account_info.is_writable {
            msg!("Writable account required for {}", account_info.key);
            Err(ProgramError::InvalidAccountData)
        } else {
            Ok(())
        }
    }

    /// Check that `account_info` is a readonly account.
    pub fn readonly(
        account_info: &AccountInfo,
    ) -> Result<(), ProgramError> {
        if !account_info.is_writable {
            msg!("Readonly account required for {}", account_info.key);
            Err(ProgramError::InvalidAccountData)
        } else {
            Ok(())
        }
    }

    /// Check that `account_info` is a signer and writable account.
    pub fn signer_and_writable(
        account_info: &AccountInfo,
    ) -> Result<(), ProgramError> {
        Self::signer(account_info)?;
        Self::writable(account_info)
    }

    /// Check that `account_info` is a signer and readonly account.
    pub fn signer_and_readonly(
        account_info: &AccountInfo,
    ) -> Result<(), ProgramError> {
        Self::signer(account_info)?;
        Self::readonly(account_info)
    }

    /// Check that `account_info` is owned by `owner_id`.
    pub fn owner(
        account_info: &AccountInfo,
        owner_id: &Pubkey,
    ) -> Result<(), ProgramError> {
        if account_info.owner.ne(owner_id) {
            msg!("Invalid Owner: expected {}, received {}", owner_id, account_info.owner);
            Err(ProgramError::IncorrectProgramId)
        } else {
            Ok(())
        }
    }

    /// Check that `pubkey` is owned by `account_key`.
    pub fn pubkey(
        pubkey: &Pubkey,
        account_key: &Pubkey,
    ) -> Result<(), ProgramError> {
        if pubkey.ne(account_key) {
            msg!("Invalid Pubkey: expected {}, received {}", pubkey, account_key);
            Err(ProgramError::IncorrectProgramId)
        } else {
            Ok(())
        }
    }

    /// Check that `account_info` is `account_key`.
    pub fn account(
        account_info: &AccountInfo,
        account_key: &Pubkey
    ) -> Result<(), ProgramError> {
        if account_info.key.ne(account_key) {
            msg!("Invalid Account: expected {}, received {}", account_info.key, account_key);
            Err(ProgramError::InvalidAccountData)
        } else {
            Ok(())
        }
    }

    /// Check that `pda_info` is an account derived from `config_info`, `seed` and `bump`, owned by 
    /// `program_id`.
    pub fn pda(
        program_id: &Pubkey,
        config_info: &AccountInfo,
        pda_info: &AccountInfo,
        seed: BondSeed,
        bump: u8,
    ) -> Result<(), ProgramError> {
        let pda = Pubkey::create_program_address(
            &[
                config_info.key.as_ref(),
                seed.as_ref().as_ref(), 
                &[bump],
            ],
            program_id,
        )?;
        Self::account(
            pda_info, 
            &pda,
        )
    }

    /// Check that `ata_info` is an associated token account derived from `pda_info` and 
    /// `token_mint`.
    pub fn ata(
        pda_info: &AccountInfo,
        token_mint: &Pubkey,
        ata_info: &AccountInfo,
    ) -> Result<(), ProgramError> {
        let ata = spl_associated_token_account::get_associated_token_address(
            &pda_info.key, 
            token_mint,
        );
        Self::account(
            ata_info, 
            &ata,
        )
    }

    /// Check that `account` has been initialized.
    pub fn initialized(
        account: &impl BondAccount,
        account_info: &AccountInfo,
    ) -> Result<(), ProgramError> {
        if !account.is_initialized() {
            msg!("Uninitialized account {}", account_info.key);
            Err(ProgramError::UninitializedAccount)
        } else {
            Ok(())
        }
    }

    /// Check that `account` is uninitialized.
    pub fn uninitialized(
        account: &impl BondAccount,
        account_info: &AccountInfo,
    ) -> Result<(), ProgramError> {
        if account.is_initialized() {
            msg!("Account already initialized {}", account_info.key);
            Err(ProgramError::AccountAlreadyInitialized)
        } else {
            Ok(())
        }
    }

    /// Check that `account` has been initialized to the correct type.
    pub fn valid(
        account: &impl BondAccount,
        account_info: &AccountInfo,
    ) -> Result<(), ProgramError> {
        if !account.is_valid() {
            msg!("Invalid account type for {}", account_info.key);
            Err(ProgramError::InvalidAccountData)
        } else {
            Ok(())
        }
    }
}