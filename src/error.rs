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

/// Known errors returned by the Bond program.
#[derive(Clone, Debug, Eq, Error, FromPrimitive, PartialEq)]
pub enum BondError {}

impl From<BondError> for ProgramError {
    fn from(e: BondError) -> Self {
        ProgramError::Custom(e as u32)
    }
}

impl<T> DecodeError<T> for BondError {
    fn type_of() -> &'static str {
        "Bond Error"
    }
}