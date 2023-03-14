//! A program for creating and managing stake pool lottos.


// The deployed program id.
solana_program::declare_id!("5uhudfA5RKkoAt6Bku1VUeWuenPiaVAgoVZj4pBUssuS");

#[cfg(not(feature = "no-entrypoint"))]
pub mod check;
pub mod create;
pub mod entrypoint;
pub mod error;
pub mod instruction;
pub mod processor;
pub mod state;
