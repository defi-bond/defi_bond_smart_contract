//! Error types.


// Imports
// -------------------------------------------------------------------------------------------------

use {
    num_derive::FromPrimitive,
    solana_program::{decode_error::DecodeError, program_error::ProgramError},
    thiserror::Error,
};


// Error
// -------------------------------------------------------------------------------------------------

/// Known errors returned by the Lotto program.
#[derive(Clone, Debug, Eq, Error, FromPrimitive, PartialEq)]
pub enum LottoError {}

impl From<LottoError> for ProgramError {
    fn from(e: LottoError) -> Self {
        ProgramError::Custom(e as u32)
    }
}

impl<T> DecodeError<T> for LottoError {
    fn type_of() -> &'static str {
        "Stake Pool Lotto Error"
    }
}